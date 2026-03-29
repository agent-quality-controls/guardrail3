use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_25_public_result_error_type::{Severity, 
    RuleFinding, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn attacks_weak_public_result_error_types_in_library_profile_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content = test_support::read_file(root, package_rel);

    let mutated = format!(
        "{package_content}\n\npub fn parse_shared_slug() -> Result<TenantSlug, String> {{\n    Err(\"missing tenant\".to_owned())\n}}\n"
    );
    write_file(root, package_rel, &mutated);

    let results = run_family(root);
    let weak_line = mutated
        .lines()
        .position(|line| line.contains("pub fn parse_shared_slug()"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_files(&results, BTreeSet::from([package_rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Warn,
            title: "weak public error type",
            message: "Public function `parse_shared_slug` returns `Result<_, String>`. Use a typed error instead.",
            file: Some(package_rel),
            line: Some(weak_line),
            inventory: false,
        }],
    );
}
