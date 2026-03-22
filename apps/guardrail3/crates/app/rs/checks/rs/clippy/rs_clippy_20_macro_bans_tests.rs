use crate::domain::report::Severity;

use super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::check;

#[test]
fn errors_when_required_macro_bans_are_missing() {
    let tree =
        root_workspace_tree(r#"disallowed-macros = [{ path = "println", reason = "good enough reason text" }]"#);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "clippy.toml"), &mut results);
    assert_eq!(results.len(), 5);
    assert!(results.iter().all(|result| result.id == "RS-CLIPPY-20"));
    assert!(results.iter().any(|result| {
        result.inventory
            && result.severity == Severity::Info
            && result.title == "macro ban present"
            && result.message == "`println!` is banned."
    }));
    assert!(results.iter().any(|result| {
        !result.inventory
            && result.severity == Severity::Error
            && result.title == "missing macro ban"
            && result.message == "`eprintln!` is not present in `disallowed-macros`."
    }));
}
