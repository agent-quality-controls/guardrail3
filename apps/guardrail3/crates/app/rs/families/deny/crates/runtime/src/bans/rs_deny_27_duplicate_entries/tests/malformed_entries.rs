use super::helpers::{add_skip_entry, build_fixture_deny_toml, set_advisory_ignores};

#[test]
fn ignores_malformed_skip_and_ignore_entries_for_duplicate_detection() {
    let with_skip = add_skip_entry(
        &add_skip_entry(
            &build_fixture_deny_toml("service"),
            toml::Value::Table(toml::map::Map::new()),
        ),
        toml::Value::Table(toml::map::Map::new()),
    );
    let deny = set_advisory_ignores(
        &with_skip,
        vec![
            toml::Value::Table(toml::map::Map::new()),
            toml::Value::Table(toml::map::Map::new()),
        ],
    );

    let results = super::helpers::run_check(&deny);
    assert!(
        results.is_empty(),
        "malformed skip/ignore entries should not create fake duplicate warnings: {results:#?}"
    );
}
