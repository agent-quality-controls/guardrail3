use cargo_toml_parser::{CargoToml, InheritableValue, LintValue, ToolLints};
use g3rs_cargo_types::{
    G3RsCargoPolicyRoot, G3RsCargoPolicyRootKind, G3RsCargoRustPolicyState, G3RsCargoWaiver,
    G3RsCargoWorkspaceMember,
};
use guardrail3_rs_toml_parser::RustProfile;
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) struct LintExpectation {
    pub name: &'static str,
    pub expected_level: &'static str,
    pub priority: Option<i64>,
}

pub(crate) const EXPECTED_RUST_LINTS: &[LintExpectation] = &[
    LintExpectation {
        name: "warnings",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unsafe_code",
        expected_level: "forbid",
        priority: None,
    },
    LintExpectation {
        name: "dead_code",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unused_results",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "unused_crate_dependencies",
        expected_level: "deny",
        priority: None,
    },
    LintExpectation {
        name: "missing_debug_implementations",
        expected_level: "warn",
        priority: None,
    },
];

pub(crate) const EXPECTED_LIBRARY_RUST_LINTS: &[LintExpectation] = &[LintExpectation {
    name: "unreachable_pub",
    expected_level: "deny",
    priority: None,
}];

pub(crate) const EXPECTED_CLIPPY_GROUPS: &[LintExpectation] = &[
    LintExpectation {
        name: "all",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "pedantic",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "cargo",
        expected_level: "deny",
        priority: Some(-1),
    },
    LintExpectation {
        name: "nursery",
        expected_level: "deny",
        priority: Some(-1),
    },
];

pub(crate) const EXPECTED_CLIPPY_DENY: &[&str] = &[
    "unwrap_used",
    "expect_used",
    "panic",
    "unimplemented",
    "todo",
    "dbg_macro",
    "print_stdout",
    "print_stderr",
    "disallowed_methods",
    "disallowed_types",
    "indexing_slicing",
    "string_slice",
    "arithmetic_side_effects",
    "shadow_unrelated",
    "missing_assert_message",
    "partial_pub_fields",
    "str_to_string",
    "implicit_clone",
    "as_conversions",
    "float_cmp",
    "lossy_float_literal",
    "wildcard_enum_match_arm",
    "rest_pat_in_fully_bound_structs",
    "large_stack_arrays",
    "needless_pass_by_value",
    "redundant_else",
    "large_futures",
    "semicolon_if_nothing_returned",
    "redundant_closure_for_method_calls",
    "map_unwrap_or",
    "verbose_file_reads",
];

pub(crate) struct RequiredAllowLint {
    pub name: &'static str,
    pub reason: &'static str,
}

pub(crate) const EXPECTED_CLIPPY_REQUIRED_ALLOW: &[RequiredAllowLint] = &[RequiredAllowLint {
    name: "redundant_pub_crate",
    reason: "Conflicts with rustc `unreachable_pub` lint. Keep `unreachable_pub = \"deny\"` and allow this clippy lint.",
}];

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CargoRole {
    WorkspaceRoot,
    PackageRoot,
    Other,
}

pub(crate) fn cargo_role(cargo: &CargoToml) -> CargoRole {
    if cargo.workspace.is_some() {
        CargoRole::WorkspaceRoot
    } else if cargo.package.is_some() {
        CargoRole::PackageRoot
    } else {
        CargoRole::Other
    }
}

pub(crate) fn role_label(role: CargoRole) -> &'static str {
    match role {
        CargoRole::WorkspaceRoot => "workspace root",
        CargoRole::PackageRoot => "standalone package root",
        CargoRole::Other => "Cargo manifest",
    }
}

pub(crate) fn policy_lints<'a>(cargo: &'a CargoToml, family: &str) -> Option<&'a ToolLints> {
    match cargo_role(cargo) {
        CargoRole::WorkspaceRoot => cargo
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.lints.as_ref())
            .and_then(|lints| lints.tools.get(family))
            .or_else(|| cargo.lints.as_ref().and_then(|lints| lints.tools.get(family))),
        CargoRole::PackageRoot => cargo.lints.as_ref()?.tools.get(family),
        CargoRole::Other => None,
    }
}

pub(crate) fn policy_lints_table_label(cargo: &CargoToml, family: &str) -> &'static str {
    match (cargo_role(cargo), family) {
        (CargoRole::WorkspaceRoot, "rust")
            if cargo
                .workspace
                .as_ref()
                .and_then(|workspace| workspace.lints.as_ref())
                .and_then(|lints| lints.tools.get("rust"))
                .is_some() =>
        {
            "[workspace.lints.rust]"
        }
        (CargoRole::WorkspaceRoot, "clippy")
            if cargo
                .workspace
                .as_ref()
                .and_then(|workspace| workspace.lints.as_ref())
                .and_then(|lints| lints.tools.get("clippy"))
                .is_some() =>
        {
            "[workspace.lints.clippy]"
        }
        (CargoRole::WorkspaceRoot, "rust") => "[lints.rust]",
        (CargoRole::WorkspaceRoot, "clippy") => "[lints.clippy]",
        (CargoRole::PackageRoot, "rust") => "[lints.rust]",
        (CargoRole::PackageRoot, "clippy") => "[lints.clippy]",
        (CargoRole::WorkspaceRoot, _)
            if cargo
                .workspace
                .as_ref()
                .and_then(|workspace| workspace.lints.as_ref())
                .is_some() =>
        {
            "[workspace.lints]"
        }
        (CargoRole::WorkspaceRoot, _) => "[lints]",
        (CargoRole::PackageRoot, _) => "[lints]",
        (CargoRole::Other, _) => "[lints]",
    }
}

pub(crate) fn lint_level<'a>(lints: &'a ToolLints, name: &str) -> Option<&'a str> {
    match lints.get(name) {
        Some(LintValue::Level(level)) => Some(level.as_str()),
        Some(LintValue::Detailed(detail)) => Some(detail.level.as_str()),
        None => None,
    }
}

pub(crate) fn lint_priority(lints: &ToolLints, name: &str) -> Option<i64> {
    match lints.get(name) {
        Some(LintValue::Detailed(detail)) => detail.priority,
        _ => None,
    }
}

pub(crate) fn level_rank(level: &str) -> usize {
    match level {
        "allow" => 0,
        "warn" => 1,
        "deny" => 2,
        "forbid" => 3,
        _ => 0,
    }
}

pub(crate) fn is_weaker(expected_level: &str, actual_level: &str) -> bool {
    level_rank(actual_level) < level_rank(expected_level)
}

pub(crate) fn policy_root_edition(cargo: &CargoToml) -> Option<Result<&str, ()>> {
    match cargo_role(cargo) {
        CargoRole::WorkspaceRoot => {
            let workspace_edition = cargo
                .workspace
                .as_ref()
                .and_then(|workspace| workspace.package.as_ref())
                .and_then(|package| package.edition.as_deref());
            if let Some(edition) = workspace_edition {
                Some(Ok(edition))
            } else if cargo
                .workspace
                .as_ref()
                .and_then(|workspace| workspace.package.as_ref())
                .is_some()
            {
                Some(Err(()))
            } else {
                cargo.package.as_ref().and_then(|package| {
                    package.edition.as_ref().map(|edition| match edition {
                        InheritableValue::Value(value) => Ok(value.as_str()),
                        InheritableValue::Inherit(_) => Err(()),
                    })
                })
            }
        }
        CargoRole::PackageRoot => cargo.package.as_ref().and_then(|package| {
            package.edition.as_ref().map(|edition| match edition {
                InheritableValue::Value(value) => Ok(value.as_str()),
                InheritableValue::Inherit(_) => Err(()),
            })
        }),
        CargoRole::Other => None,
    }
}

pub(crate) fn raw_policy_lints<'a>(
    root: &'a G3RsCargoPolicyRoot,
    family: &str,
) -> Option<&'a toml::Value> {
    match root.kind {
        G3RsCargoPolicyRootKind::WorkspaceRoot => root
            .raw_cargo
            .get("workspace")
            .and_then(|value| value.get("lints"))
            .and_then(|value| value.get(family))
            .or_else(|| root.raw_cargo.get("lints").and_then(|value| value.get(family))),
        G3RsCargoPolicyRootKind::StandalonePackageRoot => {
            root.raw_cargo.get("lints").and_then(|value| value.get(family))
        }
        G3RsCargoPolicyRootKind::Other => None,
    }
}

pub(crate) fn raw_member_lints<'a>(
    member: &'a G3RsCargoWorkspaceMember,
    family: &str,
) -> Option<&'a toml::Value> {
    member.raw_cargo.get("lints").and_then(|value| value.get(family))
}

pub(crate) fn explicit_allow_entries(lints: Option<&toml::Value>) -> Vec<String> {
    let Some(table) = lints.and_then(toml::Value::as_table) else {
        return Vec::new();
    };
    let mut entries = table
        .iter()
        .filter_map(|(name, value)| {
            (lint_level_from_value(value) == Some("allow")).then(|| name.clone())
        })
        .collect::<Vec<_>>();
    entries.sort();
    entries
}

pub(crate) fn is_approved_allow(name: &str) -> bool {
    EXPECTED_CLIPPY_ALLOW.contains(&name)
}

pub(crate) fn allow_selector(family: &str, lint_name: &str) -> String {
    format!("{family}:{lint_name}")
}

pub(crate) fn waiver_reason<'a>(
    entries: &'a [G3RsCargoWaiver],
    rule: &str,
    file: &str,
    selector: &str,
) -> Option<&'a str> {
    entries
        .iter()
        .find(|entry| {
            entry.rule == rule
                && entry.file == file
                && entry.selector == selector
        })
        .map(|entry| entry.reason.as_str())
}

pub(crate) fn rust_policy_valid(root: &G3RsCargoPolicyRoot) -> bool {
    matches!(
        root.rust_policy,
        G3RsCargoRustPolicyState::Missing | G3RsCargoRustPolicyState::Parsed { .. }
    )
}

pub(crate) fn rust_profile(root: &G3RsCargoPolicyRoot) -> Option<RustProfile> {
    match &root.rust_policy {
        G3RsCargoRustPolicyState::Parsed { profile, .. } => *profile,
        G3RsCargoRustPolicyState::Missing
        | G3RsCargoRustPolicyState::Unreadable { .. }
        | G3RsCargoRustPolicyState::ParseError { .. } => None,
    }
}

pub(crate) fn rust_policy_waivers(root: &G3RsCargoPolicyRoot) -> &[G3RsCargoWaiver] {
    match &root.rust_policy {
        G3RsCargoRustPolicyState::Parsed { waivers, .. } => waivers.as_slice(),
        G3RsCargoRustPolicyState::Missing
        | G3RsCargoRustPolicyState::Unreadable { .. }
        | G3RsCargoRustPolicyState::ParseError { .. } => &[],
    }
}

pub(crate) fn lints_table_is_well_formed(lints: Option<&toml::Value>) -> bool {
    let Some(lints) = lints else {
        return false;
    };
    let Some(table) = lints.as_table() else {
        return false;
    };
    table.values().all(has_valid_lint_level)
}

pub(crate) fn has_valid_lint_level(value: &toml::Value) -> bool {
    matches!(
        lint_level_from_value(value),
        Some(level) if is_valid_lint_level(level)
    ) && value.as_table().is_none_or(|table| {
        table
            .get("priority")
            .is_none_or(|priority| priority.as_integer().is_some())
    })
}

pub(crate) fn raw_lint_level(lints: &toml::Value, name: &str) -> Option<String> {
    lints.get(name)
        .and_then(lint_level_from_value)
        .map(str::to_owned)
}

pub(crate) fn is_valid_lint_level(level: &str) -> bool {
    matches!(level, "allow" | "warn" | "deny" | "forbid")
}

pub(crate) fn workspace_resolver(cargo: &CargoToml) -> Option<&str> {
    cargo.workspace.as_ref()?.resolver.as_deref()
}

fn lint_level_from_value(value: &toml::Value) -> Option<&str> {
    match value {
        toml::Value::String(level) => Some(level.as_str()),
        toml::Value::Table(table) => table.get("level").and_then(toml::Value::as_str),
        _ => None,
    }
}

pub(crate) fn info(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

pub(crate) fn error(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

pub(crate) fn warn(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}
