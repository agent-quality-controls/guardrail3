//! Reads `guardrail3-ts.toml` and computes the list of disabled families.

use std::path::Path;

use guardrail3_ts_app_types::SupportedFamily;
use guardrail3_ts_toml_parser_runtime::{Error, from_path};
use guardrail3_ts_toml_parser_types::guardrail3_ts_toml::{Guardrail3TsToml, TsChecksConfig};

/// Returns the list of disabled families based on a per-package `guardrail3-ts.toml`.
///
/// Missing file or unreadable file is treated as "no opt-outs".
#[must_use]
pub fn disabled_families(package_root: &Path) -> Vec<SupportedFamily> {
    let path = package_root.join("guardrail3-ts.toml");
    let parsed: Result<Guardrail3TsToml, Error> = from_path(&path);
    let Ok(toml) = parsed else {
        return Vec::new();
    };
    let Some(checks) = toml.checks else {
        return Vec::new();
    };
    collect_disabled(&checks)
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

#[cfg(test)]
#[path = "family_opt_out_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod family_opt_out_tests;
