use guardrail3_domain_report::{CheckResult, Severity};
#[cfg(test)]
use std::path::{Path, PathBuf};

use super::inputs::{CargoRootFailureInput, CoveredRustUnitInput, UncoveredRustUnitInput};

const ID: &str = "RS-CLIPPY-01";

pub fn check_covered(input: &CoveredRustUnitInput<'_>, results: &mut Vec<CheckResult>) {
    let scope = if input.rel_dir.is_empty() {
        input.kind.label().to_owned()
    } else {
        format!("{} `{}`", input.kind.label(), input.rel_dir)
    };
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "Rust unit covered by clippy.toml".to_owned(),
            message: format!("{scope} is covered by `{}`.", input.covering_config_rel),
            file: Some(input.covering_config_rel.to_owned()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

pub fn check_root_failure(input: &CargoRootFailureInput<'_>, results: &mut Vec<CheckResult>) {
    let scope = if input.rel_dir.is_empty() {
        "validation-root Cargo.toml".to_owned()
    } else {
        format!("routed Cargo root `{}`", input.rel_dir)
    };
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "Rust unit coverage could not be determined".to_owned(),
        message: format!(
            "{scope} could not be parsed from `{}` while resolving clippy coverage and policy roots: {}",
            input.cargo_rel_path, input.parse_error
        ),
        file: Some(input.cargo_rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

pub fn check_uncovered(input: &UncoveredRustUnitInput<'_>, results: &mut Vec<CheckResult>) {
    let scope = if input.rel_dir.is_empty() {
        input.kind.label().to_owned()
    } else {
        format!("{} `{}`", input.kind.label(), input.rel_dir)
    };
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "Rust unit uncovered by clippy.toml".to_owned(),
        message: format!(
            "{scope} is not covered by any allowed clippy.toml at the validation root, a workspace root, or a standalone package root."
        ),
        file: Some(input.rel_dir.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub(crate) fn fixture_root_for_tests() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../../../../tests/fixtures/r_arch_01/golden")
}

#[cfg(test)]
pub(crate) fn copy_fixture_for_tests() -> test_support::TempDir {
    test_support::copy_tree(&fixture_root_for_tests())
}

#[cfg(test)]
pub(crate) fn run_for_tests(root: &Path) -> Vec<CheckResult> {
    let tree = guardrail3_app_core::project_walker::walk_project(
        &guardrail3_adapters_outbound_fs::RealFileSystem,
        root,
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Clippy,
        ]));
    let route =
        guardrail3_app_rs_family_mapper::FamilyMapper::new(&tree, &scope, None, &selected, None)
            .map_rs_clippy();
    crate::check(&tree, &route)
}

#[cfg(test)]
#[path = "rs_clippy_01_coverage_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_01_coverage_tests;
