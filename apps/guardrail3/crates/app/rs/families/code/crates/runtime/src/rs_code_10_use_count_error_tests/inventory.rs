use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use crate::parse::{count_top_level_use_imports, parse_rust_file};
use guardrail3_app_rs_family_code_assertions::rs_code_10_use_count_error::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn attacks_excessive_top_level_use_counts_in_real_owned_file() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let imports = (0..21)
        .map(|index| format!("use crate::synthetic_{index};"))
        .collect::<Vec<_>>()
        .join("\n");
    write_file(root, rel, &format!("{imports}\n{content}"));
    let updated = test_support::read_file(root, rel);
    let ast = parse_rust_file(&updated).unwrap_or_else(|error| panic!("valid rust: {error}"));
    let total_use_count = count_top_level_use_imports(&ast);

    let results = run_family(root);
    let expected_message = format!("{total_use_count} top-level use imports (max 20).");

    assert_files(&results, BTreeSet::from([rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Error,
            title: "too many use imports",
            message: &expected_message,
            file: Some(rel),
            line: None,
            inventory: false,
        }],
    );
}
