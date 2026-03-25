use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::super::check;

#[test]
fn accepts_github_and_sparse_crates_io_forms() {
    let github = config_facts(&canonical_deny_toml_service());
    let sparse = config_facts(&canonical_deny_toml_service().replace(
        "allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]",
        "allow-registry = [\"sparse+https://index.crates.io/\"]",
    ));
    let mut results = Vec::new();

    check(&ConfigDenyInput { config: &github }, &mut results);
    check(&ConfigDenyInput { config: &sparse }, &mut results);

    assert!(results.is_empty());
}
