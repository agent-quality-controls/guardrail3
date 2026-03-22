use super::super::check;
use super::super::test_support::{canonical_clippy_toml, library_workspace_root_tree};

#[test]
fn errors_when_library_profile_root_lacks_global_state_bans() {
    let results = check(&library_workspace_root_tree(canonical_clippy_toml()));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-14" && !r.inventory && r.message.contains("LazyLock")));
}
