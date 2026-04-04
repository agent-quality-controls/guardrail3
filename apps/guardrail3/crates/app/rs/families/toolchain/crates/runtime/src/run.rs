use g3_toolchain_content_checks::{G3CargoRustVersion, G3ToolchainContentChecksInput};
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

    let package_input = G3ToolchainContentChecksInput {
        toolchain_rel_path: toolchain_rel_path.to_owned(),
        toolchain_toml,
        cargo_rel_path: input.cargo_rel_path.to_owned(),
        cargo_rust_version: cargo_rust_version(input),
    };
    let package_results = g3_toolchain_content_checks::check(&package_input);
    results.extend(package_results.into_iter().map(convert_check_result));
}

fn cargo_rust_version(input: &ToolchainPolicyRootInput<'_>) -> G3CargoRustVersion {
    if input.cargo_toml_rel.is_none() {
        return G3CargoRustVersion::MissingManifest;
    }
    if let Some(parse_error) = input.cargo_parse_error {
        return G3CargoRustVersion::ParseError(parse_error.to_owned());
    }
    if input.cargo_rust_version_invalid {
        return G3CargoRustVersion::InvalidType;
    }
    match input.cargo_rust_version {
        Some(version) => G3CargoRustVersion::Version(version.to_owned()),
        None => G3CargoRustVersion::Missing,
    }
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
