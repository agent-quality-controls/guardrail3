use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_11_use_count_warn::{assert_files, assert_findings, RuleFinding};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn warns_at_threshold_band_in_real_owned_file() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/queries/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let imports = (0..16)
        .map(|index| format!("use crate::warn_{index};"))
        .collect::<Vec<_>>()
        .join("\n");
    let total_use_count = content
        .lines()
        .filter(|line| line.trim_start().starts_with("use "))
        .count()
        + 16;

    write_file(root, rel, &format!("{imports}\n{content}"));

    let results = run_family(root);
    let expected_message =
        format!("{total_use_count} top-level use statements (warn at 16, max 20).");

    assert_files(&results, BTreeSet::from([rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Warn,
            title: "many use statements",
            message: &expected_message,
            file: Some(rel),
            line: None,
            inventory: false,
        }],
    );
}
