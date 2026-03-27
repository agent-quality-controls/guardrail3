use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn ignores_missing_or_non_workspace_unsafe_code_lints() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "packages/shared-types/Cargo.toml";
    let content = std::fs::read_to_string(root.join(rel)).expect("read package cargo");

    write_file(
        root,
        rel,
        &format!("{content}\n[lints.rust]\nunsafe_code = \"deny\"\n"),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-12"),
        BTreeSet::from([
            "apps/backend/Cargo.toml".to_owned(),
            "apps/devctl/Cargo.toml".to_owned(),
            "apps/worker/Cargo.toml".to_owned(),
        ])
    );
}
