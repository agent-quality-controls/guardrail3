use g3rs_deps_filetree_checks_assertions::run as assertions;
use g3rs_deps_types::G3RsDepsFileTreeChecksInput;
use guardrail3_rs_toml_parser::RustProfile;

#[test]
fn run_emits_lockfile_and_gitignore_findings_together() {
    let input = G3RsDepsFileTreeChecksInput {
        profile: Some(RustProfile::Service),
        cargo_lock_rel_path: "Cargo.lock".to_owned(),
        cargo_lock_exists: false,
        cargo_lock_ignored: true,
        gitignore_rel_path: Some(".gitignore".to_owned()),
    };

    let results = crate::run::check(&input);

    assertions::assert_combined_missing_and_ignored(&results);
}
