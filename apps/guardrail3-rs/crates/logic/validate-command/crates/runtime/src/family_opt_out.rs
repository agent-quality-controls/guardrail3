//! Reads `guardrail3-rs.toml` and computes the list of disabled families.

use std::path::Path;

use guardrail3_rs_app_types::SupportedFamily;
use guardrail3_rs_toml_parser_runtime::{Error, from_path};
use guardrail3_rs_toml_parser_types::guardrail3_rs_toml::{Guardrail3RsToml, RustChecksConfig};

/// Returns the list of disabled families based on a workspace's `guardrail3-rs.toml`.
///
/// Missing file or unreadable file is treated as "no opt-outs".
#[must_use]
pub fn disabled_families(workspace_root: &Path) -> Vec<SupportedFamily> {
    let path = workspace_root.join("guardrail3-rs.toml");
    let parsed: Result<Guardrail3RsToml, Error> = from_path(&path);
    let Ok(toml) = parsed else {
        return Vec::new();
    };
    let Some(checks) = toml.checks else {
        return Vec::new();
    };
    collect_disabled(&checks)
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
