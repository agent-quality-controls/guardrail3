use super::super::ConfigDenyInput;
use super::super::check;
use super::super::{build_fixture_deny_toml, config_facts};

#[test]
fn accepts_github_and_sparse_crates_io_forms() {
    let github = config_facts(&build_fixture_deny_toml("service"));
    let sparse = config_facts(&build_fixture_deny_toml("service").replace(
        "allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]",
        "allow-registry = [\"sparse+https://index.crates.io/\"]",
    ));
    let mut results = Vec::new();

    check(&ConfigDenyInput { config: &github }, &mut results);
    check(&ConfigDenyInput { config: &sparse }, &mut results);

    assert!(results.is_empty());
}
