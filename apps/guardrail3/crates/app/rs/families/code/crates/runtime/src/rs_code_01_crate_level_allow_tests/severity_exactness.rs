use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_01_crate_level_allow::{
    RuleFinding, assert_findings,
};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn uses_info_severity_for_real_test_paths() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let crate_test_rel = "apps/worker/tests/crate_allow_tests.rs";
    let inline_test_rel = "apps/devctl/tests/module_allow_tests.rs";

    write_file(
        root,
        crate_test_rel,
        "#![allow(clippy::unwrap_used)]\npub fn crate_level_fixture() {}\n",
    );
    write_file(
        root,
        inline_test_rel,
        "mod nested_test_allow {\n    #![allow(clippy::expect_used)]\n    pub fn helper() {}\n}\n",
    );

    let results = run_family(root);
    let relevant_results = results
        .into_iter()
        .filter(|result| {
            matches!(
                result.file.as_deref(),
                Some(path) if [crate_test_rel, inline_test_rel].contains(&path)
            )
        })
        .collect::<Vec<_>>();

    assert_findings(
        &relevant_results,
        &[
            RuleFinding {
                severity: Severity::Info,
                title: "crate-level allow",
                message: "Crate/module-wide allow for `clippy::unwrap_used` is test-file exempt.",
                file: Some(crate_test_rel),
                line: Some(1),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "module-level allow in nested_test_allow",
                message: "Crate/module-wide allow for `clippy::expect_used` is test-file exempt.",
                file: Some(inline_test_rel),
                line: Some(2),
                inventory: false,
            },
        ],
    );
}
