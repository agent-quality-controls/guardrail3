use guardrail3_rs_app_types::{SUPPORTED_FAMILIES, SupportedFamily, ValidateWorkspaceRequest};

#[must_use]
pub const fn family_cli_name(family: SupportedFamily) -> &'static str {
    family.cli_name()
}

/// Per-workspace default families (Hooks is moved to validate-repo).
const PER_WORKSPACE_DEFAULT_FAMILIES: &[SupportedFamily] = &[
    SupportedFamily::Topology,
    SupportedFamily::Toolchain,
    SupportedFamily::Fmt,
    SupportedFamily::Cargo,
    SupportedFamily::Clippy,
    SupportedFamily::Deny,
    SupportedFamily::Code,
    SupportedFamily::Arch,
    SupportedFamily::Deps,
    SupportedFamily::Garde,
    SupportedFamily::Test,
    SupportedFamily::Release,
    SupportedFamily::Apparch,
];

/// Repo-level families (validate-repo runs only these).
pub const REPO_LEVEL_FAMILIES: &[SupportedFamily] =
    &[SupportedFamily::Hooks, SupportedFamily::Topology];

#[must_use]
pub fn selected_families(request: &ValidateWorkspaceRequest) -> Vec<SupportedFamily> {
    if request.families.is_empty() {
        return PER_WORKSPACE_DEFAULT_FAMILIES.to_vec();
    }

    SUPPORTED_FAMILIES
        .into_iter()
        .filter(|family| request.families.contains(family))
        .collect()
}

/// Returns the families to run for a per-workspace validate, after applying the
/// workspace's `guardrail3-rs.toml` opt-out for disabled families.
#[must_use]
pub fn selected_families_with_opt_out(
    request: &ValidateWorkspaceRequest,
    disabled: &[SupportedFamily],
) -> Vec<SupportedFamily> {
    selected_families(request)
        .into_iter()
        .filter(|family| !disabled.contains(family))
        .collect()
}
