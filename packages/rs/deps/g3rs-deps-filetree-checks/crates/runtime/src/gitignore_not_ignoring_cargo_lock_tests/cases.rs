use g3rs_deps_filetree_checks_assertions::gitignore_not_ignoring_cargo_lock as assertions;
use g3rs_deps_types::G3RsDepsFileTreeChecksInput;

#[test]
fn reports_ignored_lockfile() {
    let input = G3RsDepsFileTreeChecksInput {
        profile: None,
        cargo_lock_rel_path: "Cargo.lock".to_owned(),
        cargo_lock_exists: true,
        cargo_lock_ignored: true,
        gitignore_rel_path: Some(".gitignore".to_owned()),
    };
    let mut results = Vec::new();

    crate::gitignore_not_ignoring_cargo_lock::check(&input, &mut results);

    assertions::assert_ignored_lockfile(&results);
}
