use std::os::unix::fs::PermissionsExt;

use super::super::super::test_support::{copy_fixture, errors_by_id, run_family};

#[test]
#[cfg(unix)]
fn unreadable_valid_leaf_currently_degrades_into_missing_cargo_toml() {
    let tmp = copy_fixture();
    let leaf = tmp.path().join("apps/devctl/crates/domain/types");

    let mut perms = std::fs::metadata(&leaf).expect("metadata").permissions();
    perms.set_mode(0o000);
    std::fs::set_permissions(&leaf, perms).expect("chmod 000");

    let results = run_family(tmp.path());

    let mut restore = std::fs::metadata(&leaf).expect("metadata").permissions();
    restore.set_mode(0o755);
    std::fs::set_permissions(&leaf, restore).expect("restore perms");

    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    let owned: Vec<_> = errors
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates/domain/types"))
        .collect();

    assert_eq!(owned.len(), 1, "{owned:#?}");
    assert!(owned[0].title.contains("missing Cargo.toml"), "{owned:#?}");
}
