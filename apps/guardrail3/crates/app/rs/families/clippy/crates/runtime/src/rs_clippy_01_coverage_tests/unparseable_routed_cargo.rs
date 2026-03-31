use test_support::{build_fixture_clippy_toml, create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn ignores_malformed_workspace_roots_because_arch_owns_root_legality() {
    let tmp = create_temp_dir("rs-clippy-01-unparseable-routed-cargo");
    create_dir_all(&tmp.path().join("apps/backend/crates/core"));
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace\nmembers = [\"crates/*\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/core/Cargo.toml",
        "[package]\nname = \"core\"\n",
    );

    let results = run_for_tests(tmp.path());
    assert!(
        results
            .iter()
            .all(|result| result.file() != Some("apps/backend/Cargo.toml")),
        "clippy should not report malformed workspace roots now that arch owns root legality: {results:#?}"
    );
}
