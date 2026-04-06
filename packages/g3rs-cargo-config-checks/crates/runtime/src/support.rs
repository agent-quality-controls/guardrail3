use cargo_toml_parser::{CargoToml, InheritableValue, LintValue, ToolLints};
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
        expected_level: "deny",
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
        CargoRole::WorkspaceRoot => cargo.workspace.as_ref()?.lints.as_ref()?.tools.get(family),
        CargoRole::PackageRoot => cargo.lints.as_ref()?.tools.get(family),
        CargoRole::Other => None,
    }
}

pub(crate) fn policy_lints_table_label(cargo: &CargoToml, family: &str) -> &'static str {
    match (cargo_role(cargo), family) {
        (CargoRole::WorkspaceRoot, "rust") => "[workspace.lints.rust]",
        (CargoRole::WorkspaceRoot, "clippy") => "[workspace.lints.clippy]",
        (CargoRole::PackageRoot, "rust") => "[lints.rust]",
        (CargoRole::PackageRoot, "clippy") => "[lints.clippy]",
        (CargoRole::WorkspaceRoot, _) => "[workspace.lints]",
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
        CargoRole::WorkspaceRoot => cargo
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.package.as_ref())
            .map(|package| package.edition.as_deref().ok_or(())),
        CargoRole::PackageRoot => cargo.package.as_ref().and_then(|package| {
            package.edition.as_ref().map(|edition| match edition {
                InheritableValue::Value(value) => Ok(value.as_str()),
                InheritableValue::Inherit(_) => Err(()),
            })
        }),
        CargoRole::Other => None,
    }
}

pub(crate) fn workspace_resolver(cargo: &CargoToml) -> Option<&str> {
    cargo.workspace.as_ref()?.resolver.as_deref()
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
