use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn root_workspace_including_all_rust_apps_hits_every_owned_app_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/devctl\", \"apps/backend\", \"apps/worker\"]\nresolver = \"2\"\n",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-11");
    let actual_titles = errors
        .iter()
        .map(|error| error.title.clone())
        .collect::<BTreeSet<_>>();
    let expected_titles = [
        "root workspace includes app member `apps/devctl`".to_owned(),
        "root workspace includes app member `apps/backend`".to_owned(),
        "root workspace includes app member `apps/worker`".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_titles, expected_titles,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert_eq!(error.file.as_deref(), Some("Cargo.toml"));
    }
}
