//! Reads `guardrail3-rs.toml` and computes the list of disabled families.

use std::path::Path;

use g3rs_toml_parser_runtime::{Error, from_path};
use g3rs_toml_parser_types::guardrail3_rs_toml::{Guardrail3RsToml, RustChecksConfig};
use guardrail3_rs_app_types::SupportedFamily;

/// Failure to load the required `guardrail3-rs.toml` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardrailConfigError {
    Missing,
    Invalid(String),
}

/// Families disabled by the workspace-level guardrail config.
pub type DisabledFamilies = Vec<SupportedFamily>;

impl std::fmt::Display for GuardrailConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(
                formatter,
                "guardrail3-rs.toml missing at workspace root. Create `guardrail3-rs.toml` before running `g3rs validate`."
            ),
            Self::Invalid(message) => {
                write!(
                    formatter,
                    "guardrail3-rs.toml invalid at workspace root. {message}"
                )
            }
        }
    }
}

/// Returns the list of disabled families based on a workspace's `guardrail3-rs.toml`.
///
/// The config file is required because it is the adoption marker for a G3RS
/// workspace. Missing or invalid config must fail before family checks run.
///
/// # Errors
///
/// Returns [`GuardrailConfigError`] when `guardrail3-rs.toml` is missing or invalid.
pub fn disabled_families(workspace_root: &Path) -> Result<DisabledFamilies, GuardrailConfigError> {
    let path = workspace_root.join("guardrail3-rs.toml");
    if !path.is_file() {
        return Err(GuardrailConfigError::Missing);
    }
    let parsed: Result<Guardrail3RsToml, Error> = from_path(&path);
    let toml = parsed.map_err(|error| GuardrailConfigError::Invalid(error.to_string()))?;
    let Some(checks) = toml.checks else {
        return Ok(Vec::new());
    };
    Ok(collect_disabled(&checks))
}

/// Collects the families that are explicitly disabled in the `[checks]` table.
fn collect_disabled(checks: &RustChecksConfig) -> Vec<SupportedFamily> {
    let mut disabled = Vec::new();
    if matches!(checks.topology, Some(false)) {
        disabled.push(SupportedFamily::Topology);
    }
    if matches!(checks.arch, Some(false)) {
        disabled.push(SupportedFamily::Arch);
    }
    if matches!(checks.apparch, Some(false)) {
        disabled.push(SupportedFamily::Apparch);
    }
    if matches!(checks.fmt, Some(false)) {
        disabled.push(SupportedFamily::Fmt);
    }
    if matches!(checks.toolchain, Some(false)) {
        disabled.push(SupportedFamily::Toolchain);
    }
    if matches!(checks.clippy, Some(false)) {
        disabled.push(SupportedFamily::Clippy);
    }
    if matches!(checks.deny, Some(false)) {
        disabled.push(SupportedFamily::Deny);
    }
    if matches!(checks.cargo, Some(false)) {
        disabled.push(SupportedFamily::Cargo);
    }
    if matches!(checks.code, Some(false)) {
        disabled.push(SupportedFamily::Code);
    }
    if matches!(checks.deps, Some(false)) {
        disabled.push(SupportedFamily::Deps);
    }
    if matches!(checks.garde, Some(false)) {
        disabled.push(SupportedFamily::Garde);
    }
    if matches!(checks.test, Some(false)) {
        disabled.push(SupportedFamily::Test);
    }
    if matches!(checks.release, Some(false)) {
        disabled.push(SupportedFamily::Release);
    }
    disabled
}
