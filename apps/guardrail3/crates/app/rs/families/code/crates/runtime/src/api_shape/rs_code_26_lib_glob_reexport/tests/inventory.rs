use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_26_lib_glob_reexport::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn warns_on_glob_reexport_in_real_library_lib_rs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content = test_support::read_file(root, package_rel);

    let mutated = format!(
        "{package_content}\n\nmod internal {{ pub struct Hidden; }}\npub use internal::*;\n"
    );
    write_file(root, package_rel, &mutated);

    let results = run_family(root);
    let glob_line = mutated
        .lines()
        .position(|line| line.contains("pub use internal::*;"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_files(&results, BTreeSet::from([package_rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding::new(
            Severity::Warn,
            "glob re-export in lib.rs",
            "`pub use internal::*` creates an unstable API surface.",
            Some(package_rel),
            Some(glob_line),
            false,
        )],
    );
}
