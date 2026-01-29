use aade_validator::xml::parser::AadeBook;
use aade_validator::xml::normalizer::Normalizer;
use aade_validator::validation::business_rules::BusinessRules;
use aade_validator::validation::result::ValidationStatus;
use quick_xml::de::from_str;
use std::fs;

#[test]
fn test_valid_invoice_end_to_end() {
    // 1. Load XML
    let xml_content = fs::read_to_string("tests/samples/valid_invoice.xml")
        .expect("Failed to read sample file");

    // 2. Parse
    let book: AadeBook = from_str(&xml_content).expect("Failed to parse XML");
    assert_eq!(book.invoices.len(), 1);

    let xml_invoice = book.invoices.into_iter().next().unwrap();

    // 3. Normalize
    let invoice = Normalizer::normalize(xml_invoice).expect("Normalization failed");

    // 4. Validate
    let report = BusinessRules::validate(&invoice);

    // 5. Assert
    assert_eq!(report.status, ValidationStatus::Green, "Invoice should be valid but found errors: {:?}", report.errors);
    assert!(report.errors.is_empty());
}

#[test]
fn test_invalid_invoice_totals_mismatch() {
    // 1. Load Invalid XML
    let xml_content = fs::read_to_string("tests/samples/invalid_invoice.xml")
        .expect("Failed to read sample file");

    // 2. Parse
    let book: AadeBook = from_str(&xml_content).expect("Failed to parse XML");
    let xml_invoice = book.invoices.into_iter().next().unwrap();

    // 3. Normalize
    let invoice = Normalizer::normalize(xml_invoice).expect("Normalization failed");

    // 4. Validate
    let report = BusinessRules::validate(&invoice);

    // 5. Assert
    assert_eq!(report.status, ValidationStatus::Red, "Invoice should be INVALID due to totals mismatch");
    
    // Check for specific error codes
    let has_net_error = report.errors.iter().any(|m| m.code == "BR-001");
    let has_vat_error = report.errors.iter().any(|m| m.code == "BR-002");

    assert!(has_net_error, "Missing expected error BR-001 (Net Amount Mismatch)");
    assert!(has_vat_error, "Missing expected error BR-002 (VAT Amount Mismatch)");
    
    // Print errors to verify (visible with --nocapture)
    println!("Found expected errors: {:?}", report.errors);
}
