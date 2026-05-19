//! Reads `guardrail3-ts.toml` and computes the list of disabled families.

use std::path::Path;

use g3ts_toml_parser_runtime::{Error, from_path};
use g3ts_toml_parser_types::guardrail3_ts_toml::{Guardrail3TsToml, TsChecksConfig};
use guardrail3_ts_app_types::SupportedFamily;

/// Failure to load the required `guardrail3-ts.toml` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardrailConfigError {
    Missing,
    Invalid(String),
}

/// Families disabled by the workspace-level guardrail config.
type DisabledFamilies = Vec<SupportedFamily>;

impl std::fmt::Display for GuardrailConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(
                formatter,
                "guardrail3-ts.toml missing at workspace root. Create `guardrail3-ts.toml` before running `g3ts validate workspace`."
            ),
            Self::Invalid(message) => {
                write!(
                    formatter,
                    "guardrail3-ts.toml invalid at workspace root. {message}"
                )
            }
        }
    }
}

/// Returns the list of disabled families based on a per-package `guardrail3-ts.toml`.
///
/// The config file is required because it is the adoption marker for a G3TS
/// workspace. Missing or invalid config must fail before family checks run.
///
/// # Errors
///
/// Returns [`GuardrailConfigError`] when `guardrail3-ts.toml` is missing or invalid.
pub fn disabled_families(package_root: &Path) -> Result<DisabledFamilies, GuardrailConfigError> {
    let path = package_root.join("guardrail3-ts.toml");
    if !path.is_file() {
        return Err(GuardrailConfigError::Missing);
    }
    let parsed: Result<Guardrail3TsToml, Error> = from_path(&path);
    let toml = parsed.map_err(|error| GuardrailConfigError::Invalid(error.to_string()))?;
    let Some(checks) = toml.checks else {
        return Ok(Vec::new());
    };
    Ok(collect_disabled(&checks))
}

/// Walks the typed `[checks]` table and returns the disabled families in
/// canonical order.
fn collect_disabled(checks: &TsChecksConfig) -> Vec<SupportedFamily> {
    let mut disabled = Vec::new();
    if matches!(checks.eslint, Some(false)) {
        disabled.push(SupportedFamily::Eslint);
    }
    if matches!(checks.astro_setup, Some(false)) {
        disabled.push(SupportedFamily::AstroSetup);
    }
    if matches!(checks.astro_content, Some(false)) {
        disabled.push(SupportedFamily::AstroContent);
    }
    if matches!(checks.astro_mdx, Some(false)) {
        disabled.push(SupportedFamily::AstroMdx);
    }
    if matches!(checks.astro_i18n, Some(false)) {
        disabled.push(SupportedFamily::AstroI18n);
    }
    if matches!(checks.astro_media, Some(false)) {
        disabled.push(SupportedFamily::AstroMedia);
    }
    if matches!(checks.astro_seo, Some(false)) {
        disabled.push(SupportedFamily::AstroSeo);
    }
    if matches!(checks.astro_state, Some(false)) {
        disabled.push(SupportedFamily::AstroState);
    }
    if matches!(checks.arch, Some(false)) {
        disabled.push(SupportedFamily::Arch);
    }
    if matches!(checks.apparch, Some(false)) {
        disabled.push(SupportedFamily::Apparch);
    }
    if matches!(checks.tsconfig, Some(false)) {
        disabled.push(SupportedFamily::Tsconfig);
    }
    if matches!(checks.package, Some(false)) {
        disabled.push(SupportedFamily::Package);
    }
    if matches!(checks.npmrc, Some(false)) {
        disabled.push(SupportedFamily::Npmrc);
    }
    if matches!(checks.jscpd, Some(false)) {
        disabled.push(SupportedFamily::Jscpd);
    }
    if matches!(checks.style, Some(false)) {
        disabled.push(SupportedFamily::Style);
    }
    if matches!(checks.fmt, Some(false)) {
        disabled.push(SupportedFamily::Fmt);
    }
    if matches!(checks.spelling, Some(false)) {
        disabled.push(SupportedFamily::Spelling);
    }
    if matches!(checks.typecov, Some(false)) {
        disabled.push(SupportedFamily::Typecov);
    }
    if matches!(checks.hooks, Some(false)) {
        disabled.push(SupportedFamily::Hooks);
    }
    if matches!(checks.topology, Some(false)) {
        disabled.push(SupportedFamily::Topology);
    }
    disabled
}
