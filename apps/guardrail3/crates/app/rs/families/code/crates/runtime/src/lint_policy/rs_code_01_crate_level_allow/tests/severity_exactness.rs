use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_01_crate_level_allow::{
    RuleFinding, Severity, assert_findings,
};
use test_support::write_file;

#[test]
fn uses_info_severity_for_real_test_paths() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let crate_test_rel = "apps/worker/tests/crate_allow_tests.rs";
    let inline_test_rel = "apps/devctl/tests/module_allow_tests.rs";
    let sidecar_test_rel =
        "apps/backend/crates/ports/inbound/api/src/rs_code_01_probe_tests/mod.rs";

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
    write_file(
        root,
        sidecar_test_rel,
        "#![allow(clippy::panic)]\npub fn sidecar_fixture() {}\n",
    );

    let results = run_family(root);
    let relevant_results = results
        .into_iter()
        .filter(|result| {
            matches!(
                result.file(),
                Some(path) if [crate_test_rel, inline_test_rel, sidecar_test_rel].contains(&path)
            )
        })
        .collect::<Vec<_>>();

    assert_findings(
        &relevant_results,
        &[
            RuleFinding::new(
                Severity::Info,
                "crate-level allow",
                "Crate/module-wide allow for `clippy::unwrap_used` is test-file exempt.",
                Some(crate_test_rel),
                Some(1),
                false,
            ),
            RuleFinding::new(
                Severity::Info,
                "module-level allow in nested_test_allow",
                "Crate/module-wide allow for `clippy::expect_used` is test-file exempt.",
                Some(inline_test_rel),
                Some(2),
                false,
            ),
            RuleFinding::new(
                Severity::Info,
                "crate-level allow",
                "Crate/module-wide allow for `clippy::panic` is test-file exempt.",
                Some(sidecar_test_rel),
                Some(1),
                false,
            ),
        ],
    );
}
