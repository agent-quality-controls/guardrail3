use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, remove_ban_path, root_workspace_tree,
};
use super::super::check;

#[test]
fn errors_for_each_missing_required_service_type_ban() {
    let mut clippy = canonical_clippy_toml();
    for path in ["std::collections::HashMap", "std::any::Any"] {
        clippy = remove_ban_path(&clippy, "disallowed-types", path);
    }

    let tree = root_workspace_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    let actual_errors = results
        .iter()
        .filter(|result| result.severity == Severity::Error)
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_errors = BTreeSet::from([
        "`std::any::Any` is not present in `disallowed-types`.".to_owned(),
        "`std::collections::HashMap` is not present in `disallowed-types`.".to_owned(),
    ]);

    assert_eq!(actual_errors, expected_errors);
    assert!(results.iter().all(|result| result.id == "RS-CLIPPY-05"));
}
