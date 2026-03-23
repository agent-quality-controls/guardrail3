use crate::domain::report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn errors_when_max_fn_params_bools_is_wrong() {
    let tree = root_workspace_tree("max-fn-params-bools = 4");
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-03");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "max-fn-params-bools wrong value");
    assert_eq!(result.message, "Expected 3, got 4.");
}
