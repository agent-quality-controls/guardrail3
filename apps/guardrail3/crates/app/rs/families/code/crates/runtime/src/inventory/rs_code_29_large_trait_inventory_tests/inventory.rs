use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_29_large_trait_inventory::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn inventories_large_traits_in_real_library_profile_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content = test_support::read_file(root, package_rel);

    let mut warn_methods = String::new();
    for index in 0..9 {
        warn_methods.push_str(&format!("    fn warn_{index}(&self);\n"));
    }
    let mut error_methods = String::new();
    for index in 0..13 {
        error_methods.push_str(&format!("    fn error_{index}(&self);\n"));
    }

    let mutated = format!(
        "{package_content}\n\npub trait SharedSurface {{\n{warn_methods}}}\n\npub trait OversizedSurface {{\n{error_methods}}}\n"
    );
    write_file(root, package_rel, &mutated);

    let results = run_family(root);
    let warn_line = mutated
        .lines()
        .position(|line| line.contains("pub trait SharedSurface"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let error_line = mutated
        .lines()
        .position(|line| line.contains("pub trait OversizedSurface"))
        .map(|index| index + 1)
        .unwrap_or_default();
    assert_files(&results, BTreeSet::from([package_rel.to_owned()]));
    assert_findings(
        &results,
        &[
            RuleFinding::new(
                Severity::Warn,
                "large trait surface",
                "Trait `SharedSurface` has 9 methods (warn above 8, error above 12).",
                Some(package_rel),
                Some(warn_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "large trait surface",
                "Trait `OversizedSurface` has 13 methods (warn above 8, error above 12).",
                Some(package_rel),
                Some(error_line),
                false,
            ),
        ],
    );
}
