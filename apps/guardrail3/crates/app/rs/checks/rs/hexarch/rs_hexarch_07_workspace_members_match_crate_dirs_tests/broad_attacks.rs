use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn discovered_crates_missing_from_workspace_members_hit_every_mutated_app() {
    let tmp = copy_fixture();
    for (rel, name) in [
        ("apps/devctl/crates/domain/events", "devctl-domain-events"),
        ("apps/backend/crates/domain/events", "backend-domain-events"),
        ("apps/worker/crates/domain/events", "worker-domain-events"),
    ] {
        write_file(
            tmp.path(),
            &format!("{rel}/Cargo.toml"),
            &format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\n"),
        );
        write_file(tmp.path(), &format!("{rel}/src/lib.rs"), "// events");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-07");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/devctl", "apps/backend", "apps/worker"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("not a workspace member"));
        assert!(error.title.contains("crates/domain/events"));
    }
}
