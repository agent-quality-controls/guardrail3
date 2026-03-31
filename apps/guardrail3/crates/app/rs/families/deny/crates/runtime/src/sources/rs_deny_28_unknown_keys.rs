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
        Some(rel_path.to_owned()),
        None,
        false,
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
        title: format!("unsupported {scope} schema"),
        message: format!(
            "`{rel_path}` uses unsupported schema for `{scope}`; expected {expected}."
        ),
        file: Some(rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

fn warn_unsupported_entry_schema(
    results: &mut Vec<CheckResult>,
    rel_path: &str,
    scope: &str,
    index: usize,
    expected: &str,
) {
    results.push(CheckResult {
        id: "RS-DENY-28".to_owned(),
        severity: Severity::Warn,
        title: format!("unsupported {scope} entry schema"),
        message: format!(
            "`{rel_path}` uses unsupported schema for `{scope}` entry at index {index}; expected {expected}."
        ),
        file: Some(rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

fn warn_string_array_members(
    results: &mut Vec<CheckResult>,
    rel_path: &str,
    scope: &str,
    value: &toml::Value,
) {
    let Some(entries) = value.as_array() else {
        warn_unsupported_schema(results, rel_path, scope, "array");
        return;
    };
    for (index, entry) in entries.iter().enumerate() {
        if !entry.is_str() {
            warn_unsupported_entry_schema(results, rel_path, scope, index, "string");
        }
    }
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
                let Some(table) = entry.as_table() else {
                    warn_unsupported_entry_schema(
                        results,
                        &config.rel_path,
                        "[licenses].exceptions",
                        index,
                        "table",
                    );
                    continue;
                };
                if !table.contains_key("name") && !table.contains_key("crate") {
                    warn_unsupported_entry_schema(
                        results,
                        &config.rel_path,
                        "[licenses].exceptions",
                        index,
                        "table with `name` or `crate`",
                    );
                }
                if let Some(allow) = table.get("allow") {
                    warn_string_array_members(
                        results,
                        &config.rel_path,
                        "[[licenses.exceptions]].allow",
                        allow,
                    );
                }
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

    if let Some(deny_entries) = section(config, "bans").and_then(|value| value.get("deny")) {
        if !deny_entries.is_array() {
            warn_unsupported_schema(results, &config.rel_path, "[bans].deny", "array");
        } else if let Some(deny_entries) = deny_entries.as_array() {
            for (index, entry) in deny_entries.iter().enumerate() {
                if entry.is_str() {
                    continue;
                }
                let Some(table) = entry.as_table() else {
                    warn_unsupported_entry_schema(
                        results,
                        &config.rel_path,
                        "[bans].deny",
                        index,
                        "string or table",
                    );
                    continue;
                };
                if !table.contains_key("name") && !table.contains_key("crate") {
                    warn_unsupported_entry_schema(
                        results,
                        &config.rel_path,
                        "[bans].deny",
                        index,
                        "string or table with `name` or `crate`",
                    );
                }
                if let Some(wrappers) = table.get("wrappers") {
                    warn_string_array_members(
                        results,
                        &config.rel_path,
                        "[[bans.deny]].wrappers",
                        wrappers,
                    );
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
        } else if let Some(feature_entries) = feature_entries.as_array() {
            for (index, entry) in feature_entries.iter().enumerate() {
                let Some(table) = entry.as_table() else {
                    warn_unsupported_entry_schema(
                        results,
                        &config.rel_path,
                        "[bans].features",
                        index,
                        "table",
                    );
                    continue;
                };
                if !table.contains_key("name") && !table.contains_key("crate") {
                    warn_unsupported_entry_schema(
                        results,
                        &config.rel_path,
                        "[bans].features",
                        index,
                        "table with `name` or `crate`",
                    );
                }
                if let Some(deny) = table.get("deny") {
                    warn_string_array_members(
                        results,
                        &config.rel_path,
                        "[[bans.features]].deny",
                        deny,
                    );
                }
                if let Some(allow) = table.get("allow") {
                    warn_string_array_members(
                        results,
                        &config.rel_path,
                        "[[bans.features]].allow",
                        allow,
                    );
                }
            }
        }
    }

    if let Some(licenses_allow) = section(config, "licenses").and_then(|value| value.get("allow")) {
        warn_string_array_members(
            results,
            &config.rel_path,
            "[licenses].allow",
            licenses_allow,
        );
    }

    if let Some(allow_registry) =
        section(config, "sources").and_then(|value| value.get("allow-registry"))
    {
        warn_string_array_members(
            results,
            &config.rel_path,
            "[sources].allow-registry",
            allow_registry,
        );
    }

    if let Some(allow_git) = section(config, "sources").and_then(|value| value.get("allow-git")) {
        warn_string_array_members(results, &config.rel_path, "[sources].allow-git", allow_git);
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
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) use ::test_support::{
    add_skip_entry, build_fixture_deny_toml, set_advisory_ignores, set_feature_entries,
    set_license_exceptions,
};
#[cfg(test)]
#[path = "rs_deny_28_unknown_keys_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_deny_28_unknown_keys_tests;
