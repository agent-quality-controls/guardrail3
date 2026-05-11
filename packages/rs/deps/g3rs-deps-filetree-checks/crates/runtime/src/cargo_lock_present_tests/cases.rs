use g3rs_deps_filetree_checks_assertions::cargo_lock_present as assertions;
use g3rs_deps_types::G3RsDepsFileTreeChecksInput;
use g3rs_toml_parser::types::RustProfile;

#[test]
fn reports_committed_lockfile_as_inventory() {
    let input = G3RsDepsFileTreeChecksInput {
        profile: Some(RustProfile::Service),
        cargo_lock_rel_path: "Cargo.lock".to_owned(),
        cargo_lock_exists: true,
        cargo_lock_ignored: false,
        gitignore_rel_path: None,
    };
    let mut results = Vec::new();

    crate::cargo_lock_present::check(&input, &mut results);

    assertions::assert_committed_lockfile_inventory(&results);
}

#[test]
fn reports_missing_library_lockfile_as_info() {
    let input = G3RsDepsFileTreeChecksInput {
        profile: Some(RustProfile::Library),
        cargo_lock_rel_path: "Cargo.lock".to_owned(),
        cargo_lock_exists: false,
        cargo_lock_ignored: false,
        gitignore_rel_path: None,
    };
    let mut results = Vec::new();

    crate::cargo_lock_present::check(&input, &mut results);

    assertions::assert_missing_library_lockfile_info(&results);
}
