use deny_toml_parser::{BanDenyEntry, DenyToml};
use guardrail3_check_types::G3CheckResult;

use crate::support::findings::warn;
use crate::support::identities::{feature_entry_name, license_exception_name};
use crate::support::unknown_keys;

const ID: &str = "RS-DENY-CONFIG-21";

fn warn_unknown_key(
    results: &mut Vec<G3CheckResult>,
    rel_path: &str,
    title: String,
    message: String,
) {
    results.push(warn(ID, title, message, rel_path));
}

fn warn_unsupported_schema(
    results: &mut Vec<G3CheckResult>,
    rel_path: &str,
    scope: &str,
    expected: &str,
) {
    results.push(warn(
        ID,
        format!("unsupported {scope} schema"),
        format!("`{rel_path}` uses unsupported schema for `{scope}`; expected {expected}."),
        rel_path,
    ));
}

fn warn_unsupported_entry_schema(
    results: &mut Vec<G3CheckResult>,
    rel_path: &str,
    scope: &str,
    index: usize,
    expected: &str,
) {
    results.push(warn(
        ID,
        format!("unsupported {scope} entry schema"),
        format!(
            "`{rel_path}` uses unsupported schema for `{scope}` entry at index {index}; expected {expected}."
        ),
        rel_path,
    ));
}

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    for key in deny.extra.keys() {
        if !unknown_keys::known_top_level_keys().contains(key.as_str()) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown top-level deny key".to_owned(),
                format!("`{deny_rel_path}` uses unknown top-level key `{key}`."),
            );
        }
    }

    if let Some(graph) = deny.graph.as_ref() {
        for key in unknown_keys::graph_unknown_keys(graph) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown graph key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[graph].{key}`."),
            );
        }
        for (index, entry) in graph.targets.iter().enumerate() {
            for key in unknown_keys::graph_target_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown graph.targets key".to_owned(),
                    format!(
                        "`{deny_rel_path}` uses unknown `[[graph.targets]].{key}` at index {index}."
                    ),
                );
            }
        }
    }

    if let Some(advisories) = deny.advisories.as_ref() {
        for key in unknown_keys::advisories_unknown_keys(advisories) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown advisories key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[advisories].{key}`."),
            );
        }
        for (index, entry) in advisories.ignore.iter().enumerate() {
            for key in unknown_keys::advisory_ignore_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown advisories.ignore key".to_owned(),
                    format!(
                        "`{deny_rel_path}` uses unknown `[[advisories.ignore]].{key}` at index {index}."
                    ),
                );
            }
        }
    }

    if let Some(bans) = deny.bans.as_ref() {
        for key in unknown_keys::bans_unknown_keys(bans) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown bans key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[bans].{key}`."),
            );
        }

        for (index, entry) in bans.skip.iter().enumerate() {
            if let deny_toml_parser::BanSkipEntry::Detailed(detail) = entry {
                for key in detail.extra.keys() {
                    warn_unknown_key(
                        results,
                        deny_rel_path,
                        "unknown bans.skip key".to_owned(),
                        format!(
                            "`{deny_rel_path}` uses unknown `[[bans.skip]].{key}` at index {index}."
                        ),
                    );
                }
            }
        }

        for (index, entry) in bans.allow.iter().enumerate() {
            for key in unknown_keys::allow_entry_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown bans.allow key".to_owned(),
                    format!(
                        "`{deny_rel_path}` uses unknown `[[bans.allow]].{key}` at index {index}."
                    ),
                );
            }
        }

        for (index, entry) in bans.deny.iter().enumerate() {
            match entry {
                BanDenyEntry::Simple(_) => {}
                BanDenyEntry::Detailed(detail) => {
                    if detail.name.is_none() && detail.crate_name.is_none() {
                        warn_unsupported_entry_schema(
                            results,
                            deny_rel_path,
                            "[bans].deny",
                            index,
                            "string or table with `name` or `crate`",
                        );
                    }
                    if detail
                        .wrappers
                        .iter()
                        .any(|wrapper| wrapper.trim().is_empty())
                    {
                        warn_unsupported_schema(
                            results,
                            deny_rel_path,
                            "[[bans.deny]].wrappers",
                            "non-empty strings",
                        );
                    }
                }
            }
        }

        for (index, entry) in bans.features.iter().enumerate() {
            if feature_entry_name(entry).is_none() {
                warn_unsupported_entry_schema(
                    results,
                    deny_rel_path,
                    "[bans.features]",
                    index,
                    "table with `name` or `crate`",
                );
            }
            for key in unknown_keys::feature_entry_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown bans.features key".to_owned(),
                    format!(
                        "`{deny_rel_path}` uses unknown `[[bans.features]].{key}` at index {index}."
                    ),
                );
            }
        }

        for (index, entry) in bans.skip_tree.iter().enumerate() {
            for key in unknown_keys::skip_tree_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown bans.skip-tree key".to_owned(),
                    format!(
                        "`{deny_rel_path}` uses unknown `[[bans.skip-tree]].{key}` at index {index}."
                    ),
                );
            }
        }

        if let Some(workspace_dependencies) = bans.workspace_dependencies.as_ref() {
            for key in unknown_keys::workspace_dependencies_unknown_keys(workspace_dependencies) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown bans.workspace-dependencies key".to_owned(),
                    format!(
                        "`{deny_rel_path}` uses unknown `[bans.workspace-dependencies].{key}`."
                    ),
                );
            }
        }

        if let Some(build) = bans.build.as_ref() {
            for key in unknown_keys::build_unknown_keys(build) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown bans.build key".to_owned(),
                    format!("`{deny_rel_path}` uses unknown `[bans.build].{key}`."),
                );
            }
            for (index, entry) in build.allow_build_scripts.iter().enumerate() {
                for key in unknown_keys::build_allow_build_script_unknown_keys(entry) {
                    warn_unknown_key(
                        results,
                        deny_rel_path,
                        "unknown bans.build.allow-build-scripts key".to_owned(),
                        format!(
                            "`{deny_rel_path}` uses unknown `[[bans.build.allow-build-scripts]].{key}` at index {index}."
                        ),
                    );
                }
            }
            for (index, entry) in build.bypass.iter().enumerate() {
                for key in unknown_keys::build_bypass_unknown_keys(entry) {
                    warn_unknown_key(
                        results,
                        deny_rel_path,
                        "unknown bans.build.bypass key".to_owned(),
                        format!(
                            "`{deny_rel_path}` uses unknown `[[bans.build.bypass]].{key}` at index {index}."
                        ),
                    );
                }
                for (allow_index, allow_entry) in entry.allow.iter().enumerate() {
                    for key in unknown_keys::build_bypass_allow_unknown_keys(allow_entry) {
                        warn_unknown_key(
                            results,
                            deny_rel_path,
                            "unknown bans.build.bypass.allow key".to_owned(),
                            format!(
                                "`{deny_rel_path}` uses unknown `[[bans.build.bypass.allow]].{key}` at index {index}.{allow_index}."
                            ),
                        );
                    }
                }
            }
        }
    }

    if let Some(licenses) = deny.licenses.as_ref() {
        for key in unknown_keys::licenses_unknown_keys(licenses) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown licenses key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[licenses].{key}`."),
            );
        }

        if let Some(private) = licenses.private.as_ref() {
            for key in unknown_keys::private_unknown_keys(private) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown licenses.private key".to_owned(),
                    format!("`{deny_rel_path}` uses unknown `[licenses.private].{key}`."),
                );
            }
        }

        for (index, entry) in licenses.exceptions.iter().enumerate() {
            if license_exception_name(entry).is_none() {
                warn_unsupported_entry_schema(
                    results,
                    deny_rel_path,
                    "[licenses].exceptions",
                    index,
                    "table with `name` or `crate`",
                );
            }
            for key in unknown_keys::license_exception_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown licenses.exceptions key".to_owned(),
                    format!(
                        "`{deny_rel_path}` uses unknown `[[licenses.exceptions]].{key}` at index {index}."
                    ),
                );
            }
        }

        for (index, entry) in licenses.clarify.iter().enumerate() {
            for key in unknown_keys::license_clarification_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown licenses.clarify key".to_owned(),
                    format!(
                        "`{deny_rel_path}` uses unknown `[[licenses.clarify]].{key}` at index {index}."
                    ),
                );
            }
            for (file_index, file) in entry.license_files.iter().enumerate() {
                for key in unknown_keys::license_clarification_file_unknown_keys(file) {
                    warn_unknown_key(
                        results,
                        deny_rel_path,
                        "unknown licenses.clarify.license-files key".to_owned(),
                        format!(
                            "`{deny_rel_path}` uses unknown `[[licenses.clarify.license-files]].{key}` at index {index}.{file_index}."
                        ),
                    );
                }
            }
        }
    }

    if let Some(sources) = deny.sources.as_ref() {
        for key in unknown_keys::sources_unknown_keys(sources) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown sources key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[sources].{key}`."),
            );
        }

        if let Some(allow_org) = sources.allow_org.as_ref() {
            for key in unknown_keys::allow_org_unknown_keys(allow_org) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown sources.allow-org key".to_owned(),
                    format!("`{deny_rel_path}` uses unknown `[sources.allow-org].{key}`."),
                );
            }
        }
    }

    if let Some(output) = deny.output.as_ref() {
        for key in unknown_keys::output_unknown_keys(output) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown output key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[output].{key}`."),
            );
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
