use crate::domain::invoice::Invoice;
use super::result::ValidationReport;
use super::rules_engine::RulesEngine;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::Utc;

use crate::utils::afm; // Import the AFM utility

// Embed the rules file into the binary
const RULES_YAML: &str = include_str!("../../rules/mydata_v1.yaml");

pub struct BusinessRules;

impl BusinessRules {
    pub fn validate(invoice: &Invoice) -> ValidationReport {
        let mut report = ValidationReport::new();

        // 1. Static Hardcoded Rules (Legacy/Complex Logic)
        Self::check_line_vat_consistency(invoice, &mut report);
        Self::check_classifications(invoice, &mut report);
        Self::check_totals(invoice, &mut report);
        Self::check_vat_numbers(invoice, &mut report);
        Self::check_dates(invoice, &mut report);

        // 2. Dynamic Rules Engine (YAML)
        match RulesEngine::load_from_yaml(RULES_YAML) {
            Ok(engine) => engine.apply(invoice, &mut report),
            Err(e) => {
                // If rules fail to load, this is a system error, but we log it as an error in report for now
                report.add_error(
                    "SYS-001", 
                    &format!("Failed to load validation rules: {}", e), 
                    None, 
                    None
                );
            }
        }

        report
    }

    fn check_totals(invoice: &Invoice, report: &mut ValidationReport) {
        let calc_net: Decimal = invoice.lines.iter().map(|l| l.net_value).sum();
        let calc_vat: Decimal = invoice.lines.iter().map(|l| l.vat_amount).sum();

        if calc_net != invoice.totals.total_net_amount {
            report.add_error(
                "BR-001", 
                &format!("Calculated Net Amount ({}) mismatch", calc_net),
                Some("totalNetValue"),
                Some(&invoice.totals.total_net_amount.to_string())
            );
        }

        if calc_vat != invoice.totals.total_vat_amount {
            report.add_error(
                "BR-002", 
                &format!("Calculated VAT Amount ({}) mismatch", calc_vat),
                Some("totalVatAmount"),
                Some(&invoice.totals.total_vat_amount.to_string())
            );
        }
    }

    fn check_vat_numbers(invoice: &Invoice, report: &mut ValidationReport) {
        // Validate Issuer VAT (Only for GR)
        if invoice.issuer.country == "GR" && !afm::validate_afm(&invoice.issuer.vat_number) {
            report.add_error(
                "BR-003",
                "Invalid Issuer VAT Number (AFM)",
                Some("issuer.vatNumber"),
                Some(&invoice.issuer.vat_number)
            );
        }

        // Validate Counterpart VAT (Only for GR)
        if let Some(cp) = &invoice.counterpart {
            if cp.country == "GR" && !afm::validate_afm(&cp.vat_number) {
                report.add_error(
                    "BR-004",
                    "Invalid Counterpart VAT Number (AFM)",
                    Some("counterpart.vatNumber"),
                    Some(&cp.vat_number)
                );
            }
        }
    }

    fn check_dates(invoice: &Invoice, report: &mut ValidationReport) {
        let now = Utc::now().date_naive();
        if invoice.header.issue_date > now {
            report.add_error(
                "BR-005", 
                "Issue date cannot be in the future",
                Some("invoiceHeader.issueDate"),
                Some(&invoice.header.issue_date.to_string())
            );
        }
    }

    fn check_line_vat_consistency(invoice: &Invoice, report: &mut ValidationReport) {
        for line in &invoice.lines {
            let rate = line.vat_category.rate();
            let expected_vat = (line.net_value * rate).round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero);
            
            let diff = (expected_vat - line.vat_amount).abs();
            
            // Tolerance of 0.05 EUR for rounding differences
            if diff > dec!(0.05) {
                 report.add_error(
                    "BR-VAT-CALC", 
                    &format!(
                        "VAT Amount mismatch on Line {}. Net: {}, Rate: {}, Expected: {}, Found: {}", 
                        line.line_number, line.net_value, rate, expected_vat, line.vat_amount
                    ),
                    Some("invoiceDetails.vatAmount"),
                    Some(&line.vat_amount.to_string())
                );
            }
        }
    }

    fn check_classifications(invoice: &Invoice, report: &mut ValidationReport) {
        // If there are no classifications, check if net value is > 0.
        // Some invoices might not require classification (e.g. retail sometimes), but generally they do.
        // For now, we enforce consistency if any classification exists OR if we want to be strict.
        
        let total_classification_amount: Decimal = invoice.income_classifications.iter()
            .map(|c| c.amount)
            .sum();

        let diff = (total_classification_amount - invoice.totals.total_net_amount).abs();

        if diff > dec!(0.05) {
             report.add_error(
                "BR-CLS-TOTAL", 
                &format!(
                    "Income Classification Total ({}) mismatch with Net Value ({})", 
                    total_classification_amount, invoice.totals.total_net_amount
                ),
                Some("invoiceSummary.incomeClassification"),
                Some(&total_classification_amount.to_string())
            );
        }
    }
}