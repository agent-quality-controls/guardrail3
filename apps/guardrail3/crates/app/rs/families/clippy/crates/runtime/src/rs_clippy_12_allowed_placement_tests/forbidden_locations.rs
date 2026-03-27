use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{collected_facts, nested_workspace_member_shadow_tree};
use super::super::check;

#[test]
fn reports_every_forbidden_nested_clippy_config_variant() {
    let mut results = Vec::new();

    for file_name in ["clippy.toml", ".clippy.toml"] {
        let facts = collected_facts(&nested_workspace_member_shadow_tree(file_name));
        for forbidden in &facts.forbidden_configs {
            check(forbidden, &mut results);
        }
    }

    let actual_files = results
        .iter()
        .map(|result| result.file.clone().expect("file"))
        .collect::<BTreeSet<_>>();
    let expected_files = BTreeSet::from([
        "workspace/crates/core/.clippy.toml".to_owned(),
        "workspace/crates/core/clippy.toml".to_owned(),
    ]);

    assert_eq!(actual_files, expected_files);
    assert_eq!(results.len(), expected_files.len());
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-12"
            && result.severity == Severity::Error
            && result.title == "clippy.toml in forbidden location"
    }));
    assert!(results.iter().all(|result| {
        result.message.contains("allowed clippy policy root")
            && result.message.contains("workspace roots")
            && result.message.contains("standalone package roots")
    }));
}
