use crate::domain::report::Severity;

use super::super::super::test_support::{collected_facts, same_root_dual_config_tree};
use super::super::check;

#[test]
fn rejects_lower_precedence_same_root_sibling_config() {
    let facts = collected_facts(&same_root_dual_config_tree());
    let forbidden = facts
        .forbidden_configs
        .iter()
        .find(|forbidden| forbidden.config.rel_path == ".clippy.toml")
        .expect("expected same-root conflict");
    let mut results = Vec::new();

    check(forbidden, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-12");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "same-root clippy config conflict");
    assert_eq!(result.file.as_deref(), Some(".clippy.toml"));
    assert_eq!(
        result.message,
        "`.clippy.toml` conflicts with `clippy.toml` at the same policy root. Keep only the highest-precedence clippy config file."
    );
}
