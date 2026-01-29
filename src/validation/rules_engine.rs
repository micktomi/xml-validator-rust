use serde::{Deserialize, Serialize};
use crate::domain::invoice::Invoice;
use crate::validation::result::{ValidationReport, Severity};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleLogic {
    /// Checks if a field on a specific line matches allowed values
    /// Example: "line.vat_category" must be in ["1", "2"]
    LineValueAllowed {
        field_path: String, // e.g., "vat_category" (relative to line)
        allowed_values: Vec<String>,
    },

    /// Complex check: If Header Field == X, then Line Field must be in [Y, Z]
    /// Example: If invoice_type == "1.1", then line.vat_category in ["1", "2", "3"]
    HeaderDependencyLine {
        header_field: String,      // e.g., "invoice_type"
        header_value: String,      // e.g., "1.1"
        line_check_field: String,  // e.g., "vat_category"
        allowed_values: Vec<String>,
    },

    /// Check if counterpart is required based on invoice type
    /// Example: Invoice type 1.1 requires counterpart
    CounterpartRequired {
        invoice_types: Vec<String>, // e.g., ["1.1", "1.2"]
    },

    /// Check if classification is required based on invoice type
    ClassificationRequired {
        invoice_types: Vec<String>,
        min_classifications: usize, // Minimum number required
    },

    /// Check currency rules
    /// Example: If currency != EUR, exchange_rate must exist
    CurrencyExchangeRate {
        default_currency: String, // "EUR"
    },

    /// Check that counterpart country matches expected pattern
    /// Example: For type 1.2 (intra-EU), country must not be GR
    CounterpartCountry {
        invoice_types: Vec<String>,
        excluded_countries: Vec<String>,
    },

    /// Check that all lines have negative values (for credit notes)
    NegativeAmountsOnly {
        invoice_types: Vec<String>,
    },

    /// Check that no lines have negative values (for normal invoices)
    NoNegativeAmounts {
        invoice_types: Vec<String>,
    },

    /// Check specific classification types are present
    ClassificationTypeRequired {
        invoice_types: Vec<String>,
        required_types: Vec<String>, // e.g., ["E3_561_001"]
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDefinition {
    pub id: String,
    pub description: String,
    pub severity: Severity,
    pub logic: RuleLogic,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub version: String,
    pub rules: Vec<RuleDefinition>,
}

#[derive(Default)]
pub struct RulesEngine {
    rules: Vec<RuleDefinition>,
}

impl RulesEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_yaml(content: &str) -> Result<Self, serde_yaml::Error> {
        let rule_set: RuleSet = serde_yaml::from_str(content)?;
        Ok(Self { rules: rule_set.rules })
    }

    pub fn apply(&self, invoice: &Invoice, report: &mut ValidationReport) {
        for rule in &self.rules {
            match &rule.logic {
                RuleLogic::LineValueAllowed { field_path, allowed_values } => {
                    for (idx, line) in invoice.lines.iter().enumerate() {
                        let val = match field_path.as_str() {
                            "vat_category" => Some(line.vat_category.to_string()),
                            _ => None
                        };

                        if let Some(v) = val {
                            if !allowed_values.contains(&v) {
                                Self::add_rule_error(report, rule, &rule.error_message.replace("{line}", &(idx + 1).to_string()), Some(&format!("line[{}].{}", idx+1, field_path)), Some(&v));
                            }
                        }
                    }
                },

                RuleLogic::HeaderDependencyLine { header_field, header_value, line_check_field, allowed_values } => {
                    let header_match = match header_field.as_str() {
                        "invoice_type" => invoice.header.invoice_type.to_string() == *header_value,
                        _ => false
                    };

                    if header_match {
                        for (idx, line) in invoice.lines.iter().enumerate() {
                            let line_val = match line_check_field.as_str() {
                                "vat_category" => Some(line.vat_category.to_string()),
                                _ => None
                            };

                            if let Some(v) = line_val {
                                if !allowed_values.contains(&v) {
                                    Self::add_rule_error(report, rule, &rule.error_message.replace("{line}", &(idx + 1).to_string()), Some(&format!("line[{}].{}", idx+1, line_check_field)), Some(&v));
                                }
                            }
                        }
                    }
                },

                RuleLogic::CounterpartRequired { invoice_types } => {
                    let inv_type = invoice.header.invoice_type.to_string();
                    if invoice_types.contains(&inv_type) && invoice.counterpart.is_none() {
                        Self::add_rule_error(report, rule, &rule.error_message, Some("counterpart"), None);
                    }
                },

                RuleLogic::ClassificationRequired { invoice_types, min_classifications } => {
                    let inv_type = invoice.header.invoice_type.to_string();
                    if invoice_types.contains(&inv_type)
                        && invoice.income_classifications.len() < *min_classifications {
                            Self::add_rule_error(report, rule, &rule.error_message.replace("{count}", &invoice.income_classifications.len().to_string()), Some("incomeClassification"), None);
                        }
                },

                RuleLogic::CurrencyExchangeRate { default_currency } => {
                    if invoice.header.currency != *default_currency && invoice.header.exchange_rate.is_none() {
                        Self::add_rule_error(report, rule, &rule.error_message, Some("exchangeRate"), Some(&invoice.header.currency));
                    }
                },

                RuleLogic::CounterpartCountry { invoice_types, excluded_countries } => {
                    let inv_type = invoice.header.invoice_type.to_string();
                    if invoice_types.contains(&inv_type) {
                        if let Some(cp) = &invoice.counterpart {
                            if excluded_countries.contains(&cp.country) {
                                Self::add_rule_error(report, rule, &rule.error_message, Some("counterpart.country"), Some(&cp.country));
                            }
                        }
                    }
                },

                RuleLogic::NegativeAmountsOnly { invoice_types } => {
                    let inv_type = invoice.header.invoice_type.to_string();
                    if invoice_types.contains(&inv_type) {
                        for (idx, line) in invoice.lines.iter().enumerate() {
                            if line.net_value.is_sign_positive() {
                                Self::add_rule_error(report, rule, &rule.error_message.replace("{line}", &(idx + 1).to_string()), Some(&format!("line[{}].netValue", idx+1)), Some(&line.net_value.to_string()));
                            }
                        }
                    }
                },

                RuleLogic::NoNegativeAmounts { invoice_types } => {
                    let inv_type = invoice.header.invoice_type.to_string();
                    if invoice_types.contains(&inv_type) {
                        for (idx, line) in invoice.lines.iter().enumerate() {
                            if line.net_value.is_sign_negative() {
                                Self::add_rule_error(report, rule, &rule.error_message.replace("{line}", &(idx + 1).to_string()), Some(&format!("line[{}].netValue", idx+1)), Some(&line.net_value.to_string()));
                            }
                        }
                    }
                },

                RuleLogic::ClassificationTypeRequired { invoice_types, required_types } => {
                    let inv_type = invoice.header.invoice_type.to_string();
                    if invoice_types.contains(&inv_type) {
                        for req_type in required_types {
                            let found = invoice.income_classifications.iter()
                                .any(|c| c.classification_type.as_ref() == Some(req_type));

                            if !found {
                                Self::add_rule_error(report, rule, &rule.error_message.replace("{type}", req_type), Some("incomeClassification"), Some(req_type));
                            }
                        }
                    }
                },
            }
        }
    }

    fn add_rule_error(report: &mut ValidationReport, rule: &RuleDefinition, message: &str, field: Option<&str>, value: Option<&str>) {
        match rule.severity {
            Severity::Error => report.add_error(&rule.id, message, field, value),
            Severity::Warning => report.add_warning(&rule.id, message),
            Severity::Info => {} // Could add info method if needed
        }
    }
}
