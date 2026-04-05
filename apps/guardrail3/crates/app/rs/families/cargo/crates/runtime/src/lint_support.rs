use guardrail3_domain_config::types::EscapeHatchConfig;

use super::facts::{PolicyRootCargoFacts, PolicyRootKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LintEntryValidity {
    Valid,
    InvalidLevel,
    InvalidPriority,
}

pub(crate) const EXPECTED_CLIPPY_ALLOW: &[&str] = &[
    "missing_docs_in_private_items",
    "module_name_repetitions",
    "must_use_candidate",
    "option_if_let_else",
    "empty_line_after_doc_comments",
    "single_match_else",
    "ref_option_ref",
    "trivially_copy_pass_by_ref",
    "multiple_crate_versions",
    "redundant_pub_crate",
];

/// Clippy lints that MUST be set to `allow`. These are lints where deny/warn
/// conflicts with other required lints and cannot be enforced.
pub(crate) struct RequiredAllowLint {
    pub name: &'static str,
}

pub(crate) const EXPECTED_CLIPPY_REQUIRED_ALLOW: &[RequiredAllowLint] = &[
    RequiredAllowLint {
        name: "redundant_pub_crate",
    },
];

pub fn policy_lints<'a>(root: &'a PolicyRootCargoFacts, family: &str) -> Option<&'a toml::Value> {
    let parsed = root.parsed.as_ref()?;
    match root.kind {
        PolicyRootKind::WorkspaceRoot => parsed
            .get("workspace")
            .and_then(|value| value.get("lints"))
            .and_then(|value| value.get(family)),
        PolicyRootKind::StandalonePackageRoot => {
            parsed.get("lints").and_then(|value| value.get(family))
        }
    }
}

pub fn member_lints<'a>(parsed: &'a toml::Value, family: &str) -> Option<&'a toml::Value> {
    parsed.get("lints").and_then(|value| value.get(family))
}

pub fn lint_level(lints: &toml::Value, name: &str) -> Option<String> {
    lints
        .get(name)
        .and_then(lint_level_from_value)
        .map(str::to_owned)
}

pub fn has_valid_lint_level(value: &toml::Value) -> bool {
    lint_entry_validity(value) == LintEntryValidity::Valid
}

pub fn lint_entry_validity(value: &toml::Value) -> LintEntryValidity {
    match value {
        toml::Value::String(level) => {
            if is_valid_lint_level(level) {
                LintEntryValidity::Valid
            } else {
                LintEntryValidity::InvalidLevel
            }
        }
        toml::Value::Table(table) => {
            let Some(level) = table.get("level").and_then(toml::Value::as_str) else {
                return LintEntryValidity::InvalidLevel;
            };
            if !is_valid_lint_level(level) {
                return LintEntryValidity::InvalidLevel;
            }
            if table
                .get("priority")
                .is_some_and(|priority| priority.as_integer().is_none())
            {
                return LintEntryValidity::InvalidPriority;
            }
            LintEntryValidity::Valid
        }
        _ => LintEntryValidity::InvalidLevel,
    }
}

pub fn is_valid_lint_level(level: &str) -> bool {
    matches!(level, "allow" | "warn" | "deny" | "forbid")
}

pub fn lints_table_is_well_formed(lints: Option<&toml::Value>) -> bool {
    let Some(lints) = lints else {
        return false;
    };
    let Some(table) = lints.as_table() else {
        return false;
    };

    table.values().all(has_valid_lint_level)
}

pub fn explicit_allow_entries(lints: Option<&toml::Value>) -> Vec<String> {
    let Some(table) = lints.and_then(toml::Value::as_table) else {
        return Vec::new();
    };
    let mut entries: Vec<_> = table
        .iter()
        .filter_map(|(name, value)| {
            (lint_level_from_value(value) == Some("allow")).then(|| name.clone())
        })
        .collect();
    entries.sort();
    entries
}

pub fn is_approved_allow(name: &str) -> bool {
    EXPECTED_CLIPPY_ALLOW.contains(&name)
}

pub fn allow_selector(family: &str, lint_name: &str) -> String {
    format!("{family}:{lint_name}")
}

pub fn escape_hatch_reason<'a>(
    entries: &'a [EscapeHatchConfig],
    family: &str,
    file: &str,
    kind: &str,
    selector: &str,
) -> Option<&'a str> {
    entries
        .iter()
        .find(|entry| {
            entry.family() == family
                && entry.file() == file
                && entry.kind() == kind
                && entry.selector() == selector
        })
        .map(EscapeHatchConfig::reason)
}

pub fn level_rank(level: &str) -> usize {
    match level {
        "allow" => 0,
        "warn" => 1,
        "deny" => 2,
        "forbid" => 3,
        _ => 0,
    }
}

pub fn is_weaker(expected_level: &str, actual_level: &str) -> bool {
    level_rank(actual_level) < level_rank(expected_level)
}

fn lint_level_from_value(value: &toml::Value) -> Option<&str> {
    match value {
        toml::Value::String(level) => Some(level.as_str()),
        toml::Value::Table(table) => table.get("level").and_then(toml::Value::as_str),
        _ => None,
    }
}
