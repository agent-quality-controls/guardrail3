use guardrail3_domain_report::{CheckResult, Severity};

use super::deny_support::{
    known_section_keys, known_top_level_keys, parse_feature_entries_in_config, section,
};

fn warn_unknown_key(
    results: &mut Vec<CheckResult>,
    rel_path: &str,
    title: String,
    message: String,
) {
    results.push(CheckResult::from_parts(
    "RS-DENY-28".to_owned(),
    Severity::Warn,
        title,
        message,
        file: Some(rel_path.to_owned()),
        line: None,
        inventory: false,
    ));
}

fn warn_unsupported_schema(
    results: &mut Vec<CheckResult>,
    rel_path: &str,
    scope: &str,
    expected: &str,
) {
    results.push(CheckResult {
        id: "RS-DENY-28".to_owned(),
        severity: Severity::Warn,
    format!("unsupported {scope} schema"),
    format!(
            "`{rel_path}` uses unsupported schema for `{scope}`; expected {expected}."
        ),
    Some(rel_path.to_owned()),
    None,
    false,
    });
)
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
            let Some(section_table) = value.as_table() else {
                warn_unsupported_schema(
                    results,
                    &config.rel_path,
                    &format!("[{section_name}]"),
                    "table",
                );
                continue;
            };
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

    if let Some(private) = section(config, "licenses").and_then(|value| value.get("private")) {
        if !private.is_table() {
            warn_unsupported_schema(results, &config.rel_path, "[licenses.private]", "table");
        } else if let Some(private_table) = private.as_table() {
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
    }

    if let Some(exceptions) = section(config, "licenses").and_then(|value| value.get("exceptions"))
    {
        if !exceptions.is_array() {
            warn_unsupported_schema(results, &config.rel_path, "[licenses].exceptions", "array");
        } else if let Some(exceptions) = exceptions.as_array() {
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
    }

    if let Some(skip_entries) = section(config, "bans").and_then(|value| value.get("skip")) {
        if !skip_entries.is_array() {
            warn_unsupported_schema(results, &config.rel_path, "[bans].skip", "array");
        } else if let Some(skip_entries) = skip_entries.as_array() {
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
    }

    if let Some(ignore_entries) =
        section(config, "advisories").and_then(|value| value.get("ignore"))
    {
        if !ignore_entries.is_array() {
            warn_unsupported_schema(results, &config.rel_path, "[advisories].ignore", "array");
        } else if let Some(ignore_entries) = ignore_entries.as_array() {
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
    }

    if let Some(feature_entries) = section(config, "bans").and_then(|value| value.get("features")) {
        if !feature_entries.is_array() {
            warn_unsupported_schema(results, &config.rel_path, "[bans].features", "array");
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
pub(crate) fn run_check(deny_toml: &str) -> Vec<CheckResult> {
    crate::run_config_rule_for_test(deny_toml, None, check)
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    add_skip_entry, build_fixture_deny_toml, copy_fixture, set_advisory_ignores,
    set_feature_entries, set_license_exceptions, write_file,
};
#[cfg(test)]
#[path = "rs_deny_28_unknown_keys_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_28_unknown_keys_tests;
