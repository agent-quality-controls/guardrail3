use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{known_section_keys, known_top_level_keys, parse_feature_entries_in_config, section};
use super::inputs::ConfigDenyInput;

pub fn check(input: &ConfigDenyInput<'_>, results: &mut Vec<CheckResult>) {
    let config = input.config;
    let Some(parsed) = &config.parsed else {
        return;
    };
    let Some(table) = parsed.as_table() else {
        return;
    };

    for key in table.keys() {
        if !known_top_level_keys().contains(key.as_str()) {
            results.push(CheckResult {
                id: "RS-DENY-28".to_owned(),
                severity: Severity::Warn,
                title: "unknown top-level deny key".to_owned(),
                message: format!("`{}` uses unknown top-level key `{key}`.", config.rel_path),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }

    for section_name in ["advisories", "bans", "graph", "licenses", "sources"] {
        if let Some(value) = section(config, section_name) {
            if let Some(section_table) = value.as_table() {
                for key in section_table.keys() {
                    if !known_section_keys(section_name).contains(key.as_str()) {
                        results.push(CheckResult {
                            id: "RS-DENY-28".to_owned(),
                            severity: Severity::Warn,
                            title: format!("unknown {section_name} key"),
                            message: format!("`{}` uses unknown `[{section_name}].{key}`.", config.rel_path),
                            file: Some(config.rel_path.clone()),
                            line: None,
                            inventory: false,
                        });
                    }
                }
            }
        }
    }

    if let Some(private_table) = section(config, "licenses")
        .and_then(|value| value.get("private"))
        .and_then(toml::Value::as_table)
    {
        for key in private_table.keys() {
            if !known_section_keys("private").contains(key.as_str()) {
                results.push(CheckResult {
                    id: "RS-DENY-28".to_owned(),
                    severity: Severity::Warn,
                    title: "unknown licenses.private key".to_owned(),
                    message: format!("`{}` uses unknown `[licenses.private].{key}`.", config.rel_path),
                    file: Some(config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    for entry in parse_feature_entries_in_config(parsed) {
        for key in entry.unknown_keys {
            results.push(CheckResult {
                id: "RS-DENY-28".to_owned(),
                severity: Severity::Warn,
                title: "unknown feature-ban key".to_owned(),
                message: format!("`{}` uses unknown `[[bans.features]].{key}`.", config.rel_path),
                file: Some(config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_28_unknown_keys_tests.rs"]
mod tests;
