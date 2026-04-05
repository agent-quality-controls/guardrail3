use g3_toolchain_content_checks::{
    G3ToolchainChannelAndComponentsInput, G3ToolchainMsrvConsistencyInput,
};
use guardrail3_check_types::{GrdzCheckResult, GrdzSeverity};
use guardrail3_app_rs_family_mapper::RsToolchainRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::discover::collect;
use crate::inputs::{ToolchainPolicyRootInput, all_from_facts};

pub fn check(surface: &FamilyView, route: &RsToolchainRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for input in all_from_facts(&facts) {
        crate::rs_toolchain_01_exists::check(&input, &mut results);
        run_content_checks(&input, &mut results);
        crate::rs_toolchain_04_legacy_file::check(&input, &mut results);
    }

    results
}

fn run_content_checks(input: &ToolchainPolicyRootInput<'_>, results: &mut Vec<CheckResult>) {
    if input.legacy_toolchain_rel.is_some() {
        return;
    }
    let Some(toolchain_rel_path) = input.toolchain_toml_rel else {
        if let Some(parse_error) = input.parse_error {
            results.push(CheckResult::from_parts(
                "RS-TOOLCHAIN-02".to_owned(),
                Severity::Error,
                "rust-toolchain.toml parse error".to_owned(),
                format!("Invalid TOML: {parse_error}"),
                Some("rust-toolchain.toml".to_owned()),
                None,
                false,
            ));
        }
        return;
    };
    let Some(toolchain_toml) = input.parsed.cloned() else {
        if let Some(parse_error) = input.parse_error {
            results.push(CheckResult::from_parts(
                "RS-TOOLCHAIN-02".to_owned(),
                Severity::Error,
                "rust-toolchain.toml parse error".to_owned(),
                format!("Invalid TOML: {parse_error}"),
                Some(toolchain_rel_path.to_owned()),
                None,
                false,
            ));
        }
        return;
    };

    let channel_input = G3ToolchainChannelAndComponentsInput {
        toolchain_rel_path: toolchain_rel_path.to_owned(),
        toolchain_toml: toolchain_toml.clone(),
    };
    let package_results = g3_toolchain_content_checks::check_channel_and_components(&channel_input);
    results.extend(package_results.into_iter().map(convert_check_result));

    if !uses_pinned_stable_channel(&toolchain_toml) {
        return;
    }

    let Some(cargo_toml) = input.cargo.cloned() else {
        results.push(CheckResult::from_parts(
            "RS-TOOLCHAIN-03".to_owned(),
            Severity::Error,
            match input.cargo_parse_error {
                Some(_) => "Cargo.toml parse error blocks MSRV check".to_owned(),
                None => "Cargo.toml missing blocks MSRV check".to_owned(),
            },
            match input.cargo_parse_error {
                Some(parse_error) => format!("Invalid root Cargo.toml: {parse_error}"),
                None => {
                    "Cargo.toml is required to compare pinned toolchain against declared MSRV."
                        .to_owned()
                }
            },
            Some(input.cargo_rel_path.to_owned()),
            None,
            false,
        ));
        return;
    };

    let msrv_input = G3ToolchainMsrvConsistencyInput {
        toolchain_rel_path: toolchain_rel_path.to_owned(),
        toolchain_toml,
        cargo_rel_path: input.cargo_rel_path.to_owned(),
        cargo_toml,
    };
    let msrv_results = g3_toolchain_content_checks::check_msrv_consistency(&msrv_input);
    results.extend(msrv_results.into_iter().map(convert_check_result));
}

fn uses_pinned_stable_channel(toolchain_toml: &rust_toolchain_toml_parser::RustToolchainToml) -> bool {
    let Some(channel) = toolchain_toml
        .toolchain()
        .and_then(|toolchain| toolchain.channel())
    else {
        return false;
    };

    parse_pinned_stable_version(channel).is_some()
}

fn parse_pinned_stable_version(raw: &str) -> Option<(u64, u64, u64)> {
    let normalized = raw.trim().to_ascii_lowercase();
    let mut segments = normalized.split('-');
    let version_part = segments.next()?;
    if segments.any(|segment| segment.starts_with("nightly") || segment.starts_with("beta")) {
        return None;
    }

    let version_part = version_part.trim_start_matches('v');
    let mut parts = version_part.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

fn convert_check_result(result: GrdzCheckResult) -> CheckResult {
    CheckResult::from_parts(
        result.id().to_owned(),
        convert_severity(result.severity()),
        result.title().to_owned(),
        result.message().to_owned(),
        result.file().map(str::to_owned),
        result.line(),
        result.inventory(),
    )
}

fn convert_severity(severity: GrdzSeverity) -> Severity {
    match severity {
        GrdzSeverity::Error => Severity::Error,
        GrdzSeverity::Warn => Severity::Warn,
        GrdzSeverity::Info => Severity::Info,
    }
}
