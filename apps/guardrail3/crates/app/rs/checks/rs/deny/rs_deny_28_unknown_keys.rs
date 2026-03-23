use crate::domain::report::{CheckResult, Severity};

use super::deny_support::{
    known_section_keys, known_top_level_keys, parse_feature_entries_in_config, section,
};

fn warn_unknown_key(
    results: &mut Vec<CheckResult>,
    rel_path: &str,
    title: String,
    message: String,
) {
    results.push(CheckResult {
        id: "RS-DENY-28".to_owned(),
        severity: Severity::Warn,
        title,
        message,
        file: Some(rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}
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
            warn_unknown_key(
                results,
                &config.rel_path,
                "unknown top-level deny key".to_owned(),
                format!("`{}` uses unknown top-level key `{key}`.", config.rel_path),
            );
        }
    }

    for section_name in ["advisories", "bans", "graph", "licenses", "sources"] {
        if let Some(value) = section(config, section_name) {
            if let Some(section_table) = value.as_table() {
                for key in section_table.keys() {
                    if !known_section_keys(section_name).contains(key.as_str()) {
                        warn_unknown_key(
                            results,
                            &config.rel_path,
                            format!("unknown {section_name} key"),
                            format!(
                                "`{}` uses unknown `[{section_name}].{key}`.",
                                config.rel_path
                            ),
                        );
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
                warn_unknown_key(
                    results,
                    &config.rel_path,
                    "unknown licenses.private key".to_owned(),
                    format!(
                        "`{}` uses unknown `[licenses.private].{key}`.",
                        config.rel_path
                    ),
                );
            }
        }
    }

    if let Some(exceptions) = section(config, "licenses")
        .and_then(|value| value.get("exceptions"))
        .and_then(toml::Value::as_array)
    {
        for (index, entry) in exceptions.iter().enumerate() {
            if let Some(table) = entry.as_table() {
                for key in table.keys() {
                    if !known_section_keys("exception").contains(key.as_str()) {
                        warn_unknown_key(
                            results,
                            &config.rel_path,
                            "unknown licenses.exceptions key".to_owned(),
                            format!(
                                "`{}` uses unknown `[[licenses.exceptions]].{key}` at index {index}.",
                                config.rel_path
                            ),
                        );
                    }
                }
            }
        }
    }

    if let Some(skip_entries) = section(config, "bans")
        .and_then(|value| value.get("skip"))
        .and_then(toml::Value::as_array)
    {
        for (index, entry) in skip_entries.iter().enumerate() {
            if let Some(table) = entry.as_table() {
                for key in table.keys() {
                    if !known_section_keys("skip").contains(key.as_str()) {
                        warn_unknown_key(
                            results,
                            &config.rel_path,
                            "unknown bans.skip key".to_owned(),
                            format!(
                                "`{}` uses unknown `[[bans.skip]].{key}` at index {index}.",
                                config.rel_path
                            ),
                        );
                    }
                }
            }
        }
    }

    if let Some(ignore_entries) = section(config, "advisories")
        .and_then(|value| value.get("ignore"))
        .and_then(toml::Value::as_array)
    {
        for (index, entry) in ignore_entries.iter().enumerate() {
            if let Some(table) = entry.as_table() {
                for key in table.keys() {
                    if !known_section_keys("ignore").contains(key.as_str()) {
                        warn_unknown_key(
                            results,
                            &config.rel_path,
                            "unknown advisories.ignore key".to_owned(),
                            format!(
                                "`{}` uses unknown `[[advisories.ignore]].{key}` at index {index}.",
                                config.rel_path
                            ),
                        );
                    }
                }
            }
        }
    }

    for entry in parse_feature_entries_in_config(parsed) {
        for key in entry.unknown_keys {
            warn_unknown_key(
                results,
                &config.rel_path,
                "unknown feature-ban key".to_owned(),
                format!(
                    "`{}` uses unknown `[[bans.features]].{key}`.",
                    config.rel_path
                ),
            );
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_28_unknown_keys_tests/mod.rs"]
mod tests;
