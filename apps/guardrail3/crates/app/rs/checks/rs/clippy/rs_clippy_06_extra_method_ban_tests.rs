use crate::domain::report::Severity;

use super::super::test_support::{canonical_clippy_toml, collected_facts, config_input, root_workspace_tree};
use super::check;

#[test]
fn inventories_extra_method_bans() {
    let clippy = canonical_clippy_toml().replace(
        "disallowed-methods = [\n",
        "disallowed-methods = [\n    { path = \"std::io::stdin\", reason = \"good enough reason text\" },\n",
    );
    let tree = root_workspace_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "clippy.toml"), &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-06");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "extra method ban");
    assert_eq!(
        result.message,
        "Additional method ban `std::io::stdin` beyond baseline."
    );
}
