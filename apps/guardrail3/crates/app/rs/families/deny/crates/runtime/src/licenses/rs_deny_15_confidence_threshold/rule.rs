use guardrail3_domain_report::{CheckResult, Severity};

use crate::deny_support::{expected_confidence_threshold, section};
use crate::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(licenses) = section(config, "licenses") else {
        return;
    };
    let expected = expected_confidence_threshold();
    match licenses.get("confidence-threshold") {
        Some(toml::Value::Float(value)) if *value < expected => {
            results.push(CheckResult::from_parts(
                "RS-DENY-CONFIG-11".to_owned(),
                Severity::Warn,
                "confidence-threshold weaker than baseline".to_owned(),
                format!(
                    "`{}` sets `confidence-threshold = {value}`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
        Some(toml::Value::Integer(value)) if (*value as f64) < expected => {
            results.push(CheckResult::from_parts(
                "RS-DENY-CONFIG-11".to_owned(),
                Severity::Warn,
                "confidence-threshold weaker than baseline".to_owned(),
                format!(
                    "`{}` sets `confidence-threshold = {value}`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
        Some(toml::Value::Float(value)) if *value > expected => {
            results.push(
                CheckResult::from_parts(
                    "RS-DENY-CONFIG-11".to_owned(),
                    Severity::Info,
                    "confidence-threshold stricter than baseline".to_owned(),
                    format!(
                        "`{}` sets `confidence-threshold = {value}`.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
        Some(toml::Value::Integer(value)) if (*value as f64) > expected => {
            results.push(
                CheckResult::from_parts(
                    "RS-DENY-CONFIG-11".to_owned(),
                    Severity::Info,
                    "confidence-threshold stricter than baseline".to_owned(),
                    format!(
                        "`{}` sets `confidence-threshold = {value}`.",
                        config.rel_path
                    ),
                    Some(config.rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
        Some(toml::Value::Float(_)) | Some(toml::Value::Integer(_)) => {}
        _ => {
            results.push(CheckResult::from_parts(
                "RS-DENY-CONFIG-11".to_owned(),
                Severity::Warn,
                "confidence-threshold missing or invalid".to_owned(),
                format!(
                    "`{}` must set `confidence-threshold >= 0.8`.",
                    config.rel_path
                ),
                Some(config.rel_path.clone()),
                None,
                false,
            ));
        }
    }
}
