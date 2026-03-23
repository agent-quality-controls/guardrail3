use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{
    collected_facts, forbidden_input, nested_member_shadow_tree,
};
use super::super::check_forbidden;

#[test]
fn reports_nested_shadowing_for_every_deny_filename_variant() {
    let mut results = Vec::new();

    for file_name in ["deny.toml", ".deny.toml", ".cargo/deny.toml"] {
        let facts = collected_facts(&nested_member_shadow_tree(file_name));
        let input = forbidden_input(&facts, &format!("workspace/crates/core/{file_name}"));
        check_forbidden(&input, &mut results);
    }

    let actual_files = results
        .iter()
        .map(|result| result.file.clone().expect("file"))
        .collect::<BTreeSet<_>>();
    let expected_files = BTreeSet::from([
        "workspace/crates/core/.cargo/deny.toml".to_owned(),
        "workspace/crates/core/.deny.toml".to_owned(),
        "workspace/crates/core/deny.toml".to_owned(),
    ]);

    assert_eq!(actual_files, expected_files);
    assert_eq!(results.len(), expected_files.len());
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-03"
            && result.severity == Severity::Error
            && result.title == "nested deny config shadows parent policy"
            && result.message
                == format!(
                    "`{}` shadows deny policy rooted at `workspace`.",
                    result.file.as_deref().expect("file")
                )
    }));
}
