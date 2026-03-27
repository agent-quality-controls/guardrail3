use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_28_inline_pub_mod_in_lib::{
    assert_files,
    assert_findings,
    RuleFinding,
};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn warns_on_inline_public_module_in_real_library_lib_rs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        test_support::read_file(root, package_rel);

    let mutated = format!("{package_content}\n\npub mod api {{ pub fn ping() {{}} }}\n");
    write_file(root, package_rel, &mutated);

    let results = run_family(root);
    let inline_line = mutated
        .lines()
        .position(|line| line.contains("pub mod api")).map(|index| index + 1).unwrap_or_default();

    assert_files(&results, BTreeSet::from([package_rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Warn,
            title: "inline public module in lib.rs",
            message: "`pub mod api { ... }` should live in its own file.",
            file: Some(package_rel),
            line: Some(inline_line),
            inventory: false,
        }],
    );
}
