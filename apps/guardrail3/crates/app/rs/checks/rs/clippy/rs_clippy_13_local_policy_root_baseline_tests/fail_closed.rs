use crate::domain::report::Severity;

use super::super::super::test_support::{
    collected_facts, config_input, library_workspace_root_tree,
};
use super::super::check;

#[test]
fn errors_when_local_policy_root_cannot_be_parsed() {
    let tree = library_workspace_root_tree("not = [valid");
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(
        &config_input(&facts, "apps/libsite/clippy.toml"),
        &mut results,
    );

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-13");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "local clippy policy root is not parseable");
    assert_eq!(result.file.as_deref(), Some("apps/libsite/clippy.toml"));
    assert!(result.message.contains("replaces inherited policy"));
    assert!(result.message.contains("could not be parsed"));
}
