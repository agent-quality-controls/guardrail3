use guardrail3_app_rs_family_libarch_assertions::rs_libarch_01_escalation_required as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_01_escalation_required::{
    ExpectedRuleResult, Severity,
};

mod golden;

const ROOT_CARGO: &str = "packages/shared/Cargo.toml";
const ROOT_LIB: &str = "packages/shared/src/lib.rs";

fn flat_package() -> crate::facts::LibraryPackageFacts {
    crate::facts::LibraryPackageFacts {
        package_rel_dir: "packages/shared".to_owned(),
        cargo_rel_path: ROOT_CARGO.to_owned(),
        has_package: true,
        is_library: true,
        cargo_parse_error: None,
        lib_rel_path: Some(ROOT_LIB.to_owned()),
        facade_source_error: None,
        measurement_error: None,
        escalation_required: false,
        threshold_reasons: Vec::new(),
        is_workspace: false,
        workspace_members: Vec::new(),
        workspace_members_parse_error: None,
        crates_dir_exists: false,
        layer_dirs: Vec::new(),
        uses_layered_mode: false,
        facade_exports: Vec::new(),
        member_manifests: Vec::new(),
    }
}

#[test]
fn stays_quiet_below_escalation_thresholds() {
    let package = flat_package();
    let input = crate::inputs::PackageLibarchInput::new(&package);
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assertions::assert_rule_quiet(&results);
}

#[test]
fn errors_when_flat_library_exceeds_dependency_threshold() {
    let mut package = flat_package();
    package.escalation_required = true;
    package.threshold_reasons = vec!["13 direct dependencies".to_owned()];
    let input = crate::inputs::PackageLibarchInput::new(&package);
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(ROOT_CARGO),
            message_contains: Some("exceeds the flat-library thresholds"),
            ..Default::default()
        }],
    );
}

#[test]
fn fails_closed_when_declared_lib_source_is_missing() {
    let mut package = flat_package();
    package.measurement_error = Some("declared lib source missing".to_owned());
    let input = crate::inputs::PackageLibarchInput::new(&package);
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assertions::assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(ROOT_CARGO),
            message_contains: Some("Cannot verify whether"),
            ..Default::default()
        }],
    );
}
