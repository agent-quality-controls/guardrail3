use cargo_toml_parser::types::{
    CargoBoolFieldState, CargoLintTableState, CargoStringFieldState, CargoToml, InheritableValue,
    LintValue, ToolLints,
};
use g3rs_cargo_types::{
    G3RsCargoPolicyRoot, G3RsCargoRustPolicyState, G3RsCargoWaiver, G3RsCargoWorkspaceMember,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_rs_toml_parser::types::RustProfile;

/// Lint Expectation struct.
pub(crate) struct LintExpectation {
    /// str static.
    pub name: &'static str,
    /// str static.
    pub expected_level: &'static str,
    /// Internal item.
    pub priority: Option<i64>,
}

/// EXPECTED RUST LINTS const.
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

/// EXPECTED LIBRARY RUST LINTS const.
pub(crate) const EXPECTED_LIBRARY_RUST_LINTS: &[LintExpectation] = &[LintExpectation {
    name: "unreachable_pub",
    expected_level: "deny",
    priority: None,
}];

/// EXPECTED CLIPPY GROUPS const.
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

/// EXPECTED CLIPPY DENY const.
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

/// Required Allow Lint struct.
pub(crate) struct RequiredAllowLint {
    /// str static.
    pub name: &'static str,
    /// str static.
    pub reason: &'static str,
}

/// EXPECTED CLIPPY REQUIRED ALLOW const.
pub(crate) const EXPECTED_CLIPPY_REQUIRED_ALLOW: &[RequiredAllowLint] = &[RequiredAllowLint {
    name: "redundant_pub_crate",
    reason: "Conflicts with rustc `unreachable_pub` lint. Keep `unreachable_pub = \"deny\"` and allow this clippy lint.",
}];

/// EXPECTED CLIPPY ALLOW const.
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

/// Cargo Role enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CargoRole {
    /// Internal item.
    WorkspaceRoot,
    /// Internal item.
    PackageRoot,
    /// Internal item.
    Other,
}

/// fn const.
pub(crate) const fn cargo_role(cargo: &CargoToml) -> CargoRole {
    if cargo.workspace.is_some() {
        CargoRole::WorkspaceRoot
    } else if cargo.package.is_some() {
        CargoRole::PackageRoot
    } else {
        CargoRole::Other
    }
}

/// fn const.
pub(crate) const fn role_label(role: CargoRole) -> &'static str {
    match role {
        CargoRole::WorkspaceRoot => "workspace root",
        CargoRole::PackageRoot => "standalone package root",
        CargoRole::Other => "Cargo manifest",
    }
}

/// policy lints fn.
pub(crate) fn policy_lints<'a>(cargo: &'a CargoToml, family: &str) -> Option<&'a ToolLints> {
    match cargo_role(cargo) {
        CargoRole::WorkspaceRoot => cargo
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.lints.as_ref())
            .and_then(|lints| lints.tools.get(family))
            .or_else(|| {
                cargo
                    .lints
                    .as_ref()
                    .and_then(|lints| lints.tools.get(family))
            }),
        CargoRole::PackageRoot => cargo.lints.as_ref()?.tools.get(family),
        CargoRole::Other => None,
    }
}

/// policy lints table label fn.
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
        (CargoRole::WorkspaceRoot | CargoRole::PackageRoot, "rust") => "[lints.rust]",
        (CargoRole::WorkspaceRoot | CargoRole::PackageRoot, "clippy") => "[lints.clippy]",
        (CargoRole::WorkspaceRoot, _)
            if cargo
                .workspace
                .as_ref()
                .and_then(|workspace| workspace.lints.as_ref())
                .is_some() =>
        {
            "[workspace.lints]"
        }
        (CargoRole::WorkspaceRoot | CargoRole::PackageRoot | CargoRole::Other, _) => "[lints]",
    }
}

/// lint level fn.
pub(crate) fn lint_level<'a>(lints: &'a ToolLints, name: &str) -> Option<&'a str> {
    match lints.get(name) {
        Some(LintValue::Level(level)) => Some(level.as_str()),
        Some(LintValue::Detailed(detail)) => Some(detail.level.as_str()),
        None => None,
    }
}

/// lint priority fn.
pub(crate) fn lint_priority(lints: &ToolLints, name: &str) -> Option<i64> {
    match lints.get(name) {
        Some(LintValue::Detailed(detail)) => detail.priority,
        Some(LintValue::Level(_)) | None => None,
    }
}

/// level rank fn.
pub(crate) fn level_rank(level: &str) -> usize {
    match level {
        "warn" => 1,
        "deny" => 2,
        "forbid" => 3,
        // `"allow"` is the canonical zero rank; anything unrecognized falls through to allow-strength.
        _ => 0,
    }
}

/// is weaker fn.
pub(crate) fn is_weaker(expected_level: &str, actual_level: &str) -> bool {
    level_rank(actual_level) < level_rank(expected_level)
}

/// Resolved edition string or an `Err(())` marker for "present but not a literal value".
pub(crate) type EditionResolution<'a> = Result<&'a str, ()>;

/// policy root edition fn.
pub(crate) fn policy_root_edition(cargo: &CargoToml) -> Option<EditionResolution<'_>> {
    match cargo_role(cargo) {
        CargoRole::WorkspaceRoot => workspace_root_edition(cargo),
        CargoRole::PackageRoot => package_root_edition(cargo),
        CargoRole::Other => None,
    }
}

/// Resolve `[workspace.package].edition`, falling back to `[package].edition` when absent.
fn workspace_root_edition(cargo: &CargoToml) -> Option<EditionResolution<'_>> {
    let workspace_package = cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.package.as_ref());
    let Some(package) = workspace_package else {
        return package_root_edition(cargo);
    };
    Some(package.edition.as_deref().ok_or(()))
}

/// Resolve `[package].edition` honouring `workspace = true` inheritance.
fn package_root_edition(cargo: &CargoToml) -> Option<EditionResolution<'_>> {
    cargo.package.as_ref().and_then(|package| {
        package.edition.as_ref().map(|edition| match edition {
            InheritableValue::Value(value) => Ok(value.as_str()),
            InheritableValue::Inherit(_) => Err(()),
        })
    })
}

/// policy override lints fn.
pub(crate) fn policy_override_lints<'a>(
    root: &'a G3RsCargoPolicyRoot,
    family: &str,
) -> Option<&'a ToolLints> {
    cargo_toml_parser::document::policy_lints(&root.cargo, family)
}

/// root package policy lints fn.
pub(crate) fn root_package_policy_lints<'a>(
    root: &'a G3RsCargoPolicyRoot,
    family: &str,
) -> Option<&'a ToolLints> {
    cargo_toml_parser::document::typed(&root.cargo)?
        .lints
        .as_ref()
        .and_then(|lints| lints.tools.get(family))
}

/// member override lints fn.
pub(crate) fn member_override_lints<'a>(
    member: &'a G3RsCargoWorkspaceMember,
    family: &str,
) -> Option<&'a ToolLints> {
    cargo_toml_parser::document::member_lints(&member.cargo, family)
}

/// member override lints state fn.
pub(crate) fn member_override_lints_state<'a>(
    member: &'a G3RsCargoWorkspaceMember,
    family: &str,
) -> CargoLintTableState<'a> {
    cargo_toml_parser::document::member_lints_state(&member.cargo, family)
}

/// explicit allow entries fn.
pub(crate) fn explicit_allow_entries(lints: Option<&ToolLints>) -> Vec<String> {
    let Some(lints) = lints else {
        return Vec::new();
    };
    let mut entries = lints
        .iter()
        .filter(|&(_name, value)| lint_level_from_value(value) == "allow")
        .map(|(name, _value)| name.clone())
        .collect::<Vec<_>>();
    entries.sort();
    entries
}

/// is approved allow fn.
pub(crate) fn is_approved_allow(name: &str) -> bool {
    EXPECTED_CLIPPY_ALLOW.contains(&name)
}

/// allow selector fn.
pub(crate) fn allow_selector(family: &str, lint_name: &str) -> String {
    format!("{family}:{lint_name}")
}

/// waiver reason fn.
pub(crate) fn waiver_reason<'a>(
    entries: &'a [G3RsCargoWaiver],
    rule: &str,
    file: &str,
    selector: &str,
) -> Option<&'a str> {
    entries
        .iter()
        .find(|entry| entry.rule == rule && entry.file == file && entry.selector == selector)
        .map(|entry| entry.reason.as_str())
}

/// fn const.
pub(crate) const fn rust_policy_valid(root: &G3RsCargoPolicyRoot) -> bool {
    matches!(
        root.rust_policy,
        G3RsCargoRustPolicyState::Missing | G3RsCargoRustPolicyState::Parsed { .. }
    )
}

/// fn const.
pub(crate) const fn rust_profile(root: &G3RsCargoPolicyRoot) -> Option<RustProfile> {
    match &root.rust_policy {
        G3RsCargoRustPolicyState::Parsed { profile, .. } => *profile,
        G3RsCargoRustPolicyState::Missing
        | G3RsCargoRustPolicyState::Unreadable { .. }
        | G3RsCargoRustPolicyState::ParseError { .. } => None,
    }
}

/// rust policy waivers fn.
pub(crate) fn rust_policy_waivers(root: &G3RsCargoPolicyRoot) -> &[G3RsCargoWaiver] {
    match &root.rust_policy {
        G3RsCargoRustPolicyState::Parsed { waivers, .. } => waivers.as_slice(),
        G3RsCargoRustPolicyState::Missing
        | G3RsCargoRustPolicyState::Unreadable { .. }
        | G3RsCargoRustPolicyState::ParseError { .. } => &[],
    }
}

/// fn const.
pub(crate) const fn lints_table_is_well_formed(lints: Option<&ToolLints>) -> bool {
    lints.is_some()
}

/// has valid lint level fn.
pub(crate) fn has_valid_lint_level(value: &LintValue) -> bool {
    is_valid_lint_level(lint_level_from_value(value))
}

/// lint level for name fn.
pub(crate) fn lint_level_for_name(lints: &ToolLints, name: &str) -> Option<String> {
    lints
        .get(name)
        .map(lint_level_from_value)
        .map(str::to_owned)
}

/// is valid lint level fn.
pub(crate) fn is_valid_lint_level(level: &str) -> bool {
    matches!(level, "allow" | "warn" | "deny" | "forbid")
}

/// workspace resolver fn.
pub(crate) fn workspace_resolver(cargo: &CargoToml) -> Option<&str> {
    cargo.workspace.as_ref()?.resolver.as_deref()
}

/// lint level from value fn.
fn lint_level_from_value(value: &LintValue) -> &str {
    match value {
        LintValue::Level(level) => level.as_str(),
        LintValue::Detailed(detail) => detail.level.as_str(),
    }
}

/// root edition state fn.
pub(crate) fn root_edition_state(root: &G3RsCargoPolicyRoot) -> CargoStringFieldState<'_> {
    cargo_toml_parser::document::root_package_string_field(&root.cargo, "edition")
}

/// root rust version state fn.
pub(crate) fn root_rust_version_state(root: &G3RsCargoPolicyRoot) -> CargoStringFieldState<'_> {
    cargo_toml_parser::document::root_package_string_field(&root.cargo, "rust-version")
}

/// member edition state fn.
pub(crate) fn member_edition_state(member: &G3RsCargoWorkspaceMember) -> CargoStringFieldState<'_> {
    cargo_toml_parser::document::package_string_field(&member.cargo, "edition")
}

/// member lints workspace state fn.
pub(crate) fn member_lints_workspace_state(
    member: &G3RsCargoWorkspaceMember,
) -> CargoBoolFieldState<'_> {
    cargo_toml_parser::document::lints_workspace_state(&member.cargo)
}

/// member package name fn.
pub(crate) fn member_package_name(member: &G3RsCargoWorkspaceMember) -> Option<&str> {
    cargo_toml_parser::document::package_name(&member.cargo)
}

/// info fn.
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

/// error fn.
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

/// warn fn.
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
