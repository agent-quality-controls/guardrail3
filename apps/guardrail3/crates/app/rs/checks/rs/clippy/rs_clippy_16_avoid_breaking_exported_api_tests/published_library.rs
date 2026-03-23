use crate::domain::report::Severity;

use super::super::super::test_support::{
    collected_facts, config_input, published_library_package_root_tree,
};
use super::super::check;

#[test]
fn inventories_true_value_for_published_library_packages() {
    let tree = published_library_package_root_tree("avoid-breaking-exported-api = true");
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-16");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(
        result.title,
        "library keeps avoid-breaking-exported-api enabled"
    );
    assert_eq!(
        result.message,
        "Published library profile may legitimately keep `avoid-breaking-exported-api = true`."
    );
}
