use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_file_length_using_real_owned_file_surface() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read source");
    let filler = "fn filler() {}\n".repeat(501);

    write_file(root, rel, &format!("{content}\n{filler}"));

    let results = run_family(root);
    let rs_code_09_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-09")
        .map(|result| {
            (
                result.file.clone().expect("file"),
                result.line,
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-09"),
        BTreeSet::from([rel.to_owned()])
    );
    assert_eq!(
        rs_code_09_results,
        vec![(
            rel.to_owned(),
            None,
            format!("{:?}", Severity::Error),
            "file too long".to_owned(),
            "538 effective lines (max 500). Long files are hard to review and maintain.".to_owned(),
        )]
    );
}
