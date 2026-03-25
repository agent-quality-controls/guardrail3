use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{expected_confidence_threshold, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        return;
    };
    let expected = expected_confidence_threshold();
    match licenses.get("confidence-threshold") {
        Some(toml::Value::Float(value)) if *value < expected => results.push(CheckResult {
            id: "RS-DENY-15".to_owned(),
            severity: Severity::Warn,
            title: "confidence-threshold weaker than baseline".to_owned(),
            message: format!(
                "`{}` sets `confidence-threshold = {value}`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
        Some(toml::Value::Integer(value)) if (*value as f64) < expected => {
            results.push(CheckResult {
                id: "RS-DENY-15".to_owned(),
                severity: Severity::Warn,
                title: "confidence-threshold weaker than baseline".to_owned(),
                message: format!(
                    "`{}` sets `confidence-threshold = {value}`.",
                    config.rel_path
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            })
        }
        Some(toml::Value::Float(value)) if *value > expected => results.push(
            CheckResult {
                id: "RS-DENY-15".to_owned(),
                severity: Severity::Info,
                title: "confidence-threshold stricter than baseline".to_owned(),
                message: format!(
                    "`{}` sets `confidence-threshold = {value}`.",
                    config.rel_path
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        Some(toml::Value::Integer(value)) if (*value as f64) > expected => results.push(
            CheckResult {
                id: "RS-DENY-15".to_owned(),
                severity: Severity::Info,
                title: "confidence-threshold stricter than baseline".to_owned(),
                message: format!(
                    "`{}` sets `confidence-threshold = {value}`.",
                    config.rel_path
                ),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        Some(toml::Value::Float(_)) | Some(toml::Value::Integer(_)) => {}
        _ => results.push(CheckResult {
            id: "RS-DENY-15".to_owned(),
            severity: Severity::Warn,
            title: "confidence-threshold missing or invalid".to_owned(),
            message: format!(
                "`{}` must set `confidence-threshold >= 0.8`.",
                config.rel_path
            ),
            file: Some(config.rel_path.clone()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_deny_15_confidence_threshold_tests/mod.rs"]
mod tests;
