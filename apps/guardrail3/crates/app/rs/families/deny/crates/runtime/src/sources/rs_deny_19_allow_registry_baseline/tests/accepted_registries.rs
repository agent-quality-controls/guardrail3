use crate::inputs::ConfigDenyInput;
use super::super::check;
use super::helpers::{build_fixture_deny_toml, config_facts};

#[test]
fn accepts_only_the_canonical_sparse_crates_io_form() {
    let sparse = config_facts(&build_fixture_deny_toml("service"));
    let mut results = Vec::new();

    check(&ConfigDenyInput { config: &sparse }, &mut results);

    assert!(results.is_empty());
}
