use deny_toml_parser::{BanDenyEntry, DenyToml};
use guardrail3_check_types::G3CheckResult;

use crate::support::{
    advisories_unknown_keys, advisory_ignore_unknown_keys, allow_org_unknown_keys, bans_unknown_keys,
    feature_entry_name, feature_entry_unknown_keys, graph_unknown_keys, known_top_level_keys,
    license_exception_name, license_exception_unknown_keys, licenses_unknown_keys,
    output_unknown_keys, private_unknown_keys, sources_unknown_keys, warn,
};

const ID: &str = "RS-DENY-28";

fn warn_unknown_key(results: &mut Vec<G3CheckResult>, rel_path: &str, title: String, message: String) {
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
        if !known_top_level_keys().contains(key.as_str()) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown top-level deny key".to_owned(),
                format!("`{deny_rel_path}` uses unknown top-level key `{key}`."),
            );
        }
    }

    if let Some(graph) = deny.graph.as_ref() {
        for key in graph_unknown_keys(graph) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown graph key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[graph].{key}`."),
            );
        }
    }

    if let Some(advisories) = deny.advisories.as_ref() {
        for key in advisories_unknown_keys(advisories) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown advisories key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[advisories].{key}`."),
            );
        }
        for (index, entry) in advisories.ignore.iter().enumerate() {
            for key in advisory_ignore_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown advisories.ignore key".to_owned(),
                    format!("`{deny_rel_path}` uses unknown `[[advisories.ignore]].{key}` at index {index}."),
                );
            }
        }
    }

    if let Some(bans) = deny.bans.as_ref() {
        for key in bans_unknown_keys(bans) {
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
                        format!("`{deny_rel_path}` uses unknown `[[bans.skip]].{key}` at index {index}."),
                    );
                }
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
                    if detail.wrappers.iter().any(|wrapper| wrapper.trim().is_empty()) {
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
            for key in feature_entry_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown bans.features key".to_owned(),
                    format!("`{deny_rel_path}` uses unknown `[[bans.features]].{key}` at index {index}."),
                );
            }
        }
    }

    if let Some(licenses) = deny.licenses.as_ref() {
        for key in licenses_unknown_keys(licenses) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown licenses key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[licenses].{key}`."),
            );
        }

        if let Some(private) = licenses.private.as_ref() {
            for key in private_unknown_keys(private) {
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
            for key in license_exception_unknown_keys(entry) {
                warn_unknown_key(
                    results,
                    deny_rel_path,
                    "unknown licenses.exceptions key".to_owned(),
                    format!("`{deny_rel_path}` uses unknown `[[licenses.exceptions]].{key}` at index {index}."),
                );
            }
        }
    }

    if let Some(sources) = deny.sources.as_ref() {
        for key in sources_unknown_keys(sources) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown sources key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[sources].{key}`."),
            );
        }

        if let Some(allow_org) = sources.allow_org.as_ref() {
            for key in allow_org_unknown_keys(allow_org) {
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
        for key in output_unknown_keys(output) {
            warn_unknown_key(
                results,
                deny_rel_path,
                "unknown output key".to_owned(),
                format!("`{deny_rel_path}` uses unknown `[output].{key}`."),
            );
        }
    }
}
