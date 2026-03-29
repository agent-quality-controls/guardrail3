use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_09_file_length::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn attacks_file_length_using_real_owned_file_surface() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let filler = "fn filler() {}\n".repeat(501);

    write_file(root, rel, &format!("{content}\n{filler}"));

    let results = run_family(root);

    assert_files(&results, BTreeSet::from([rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Error,
            title: "file too long",
            message: "538 effective code-bearing lines (max 500). Long files are hard to review and maintain.",
            file: Some(rel),
            line: None,
            inventory: false,
        }],
    );
}
