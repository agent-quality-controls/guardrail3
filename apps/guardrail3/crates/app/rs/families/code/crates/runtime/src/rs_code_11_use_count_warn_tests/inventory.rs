use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn warns_at_threshold_band_in_real_owned_file() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/app/queries/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read source");
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
    let rs_code_11_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-11")
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
    let rs_code_10_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-10")
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-11"),
        BTreeSet::from([rel.to_owned()])
    );
    assert_eq!(
        rs_code_11_results,
        vec![(
            rel.to_owned(),
            None,
            format!("{:?}", Severity::Warn),
            "many use statements".to_owned(),
            format!("{total_use_count} top-level use statements (warn at 16, max 20)."),
        )]
    );
    assert!(rs_code_10_results.is_empty());
}
