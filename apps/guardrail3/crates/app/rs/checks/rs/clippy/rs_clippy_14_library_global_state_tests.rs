use crate::domain::report::Severity;

use super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, library_workspace_root_tree,
};
use super::check;

#[test]
fn errors_when_library_profile_root_lacks_global_state_bans() {
    let tree = library_workspace_root_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "apps/libsite/clippy.toml"), &mut results);
    assert_eq!(results.len(), 4);
    assert!(results.iter().all(|result| {
        result.id == "RS-CLIPPY-14"
            && !result.inventory
            && result.severity == Severity::Error
            && result.title == "library clippy.toml missing global-state type ban"
    }));
    assert!(results.iter().any(|result| {
        result.message == "Library profile must ban `std::sync::LazyLock` in `disallowed-types`."
    }));
}
