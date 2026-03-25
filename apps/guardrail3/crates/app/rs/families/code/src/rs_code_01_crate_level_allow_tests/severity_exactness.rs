use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

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
    let mut rs_code_01_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-01")
        .collect::<Vec<_>>();
    rs_code_01_results.sort_by_key(|result| result.file.clone().expect("file"));

    assert_eq!(
        files_for_rule(&results, "RS-CODE-01"),
        BTreeSet::from([crate_test_rel.to_owned(), inline_test_rel.to_owned()])
    );
    assert_eq!(rs_code_01_results.len(), 2);
    assert!(
        rs_code_01_results
            .iter()
            .all(|result| result.severity == Severity::Info)
    );
    assert_eq!(
        rs_code_01_results
            .iter()
            .map(|result| result.line)
            .collect::<Vec<_>>(),
        vec![Some(2), Some(1)]
    );
    assert_eq!(
        rs_code_01_results
            .iter()
            .map(|result| result.title.as_str())
            .collect::<Vec<_>>(),
        vec![
            "module-level allow in nested_test_allow",
            "crate-level allow"
        ]
    );
    assert_eq!(
        rs_code_01_results
            .iter()
            .map(|result| result.message.as_str())
            .collect::<Vec<_>>(),
        vec![
            "Crate/module-wide allow for `clippy::expect_used` is test-file exempt.",
            "Crate/module-wide allow for `clippy::unwrap_used` is test-file exempt.",
        ]
    );
}
