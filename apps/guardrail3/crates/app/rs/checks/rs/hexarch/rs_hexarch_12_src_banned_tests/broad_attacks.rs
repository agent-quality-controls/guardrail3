use std::collections::BTreeSet;

use super::super::super::test_support::{
    copy_fixture, create_dir, errors_by_id, run_family, write_file,
};

#[test]
fn app_level_src_dirs_hit_every_mutated_rust_app() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    write_file(tmp.path(), "apps/backend/src/main.rs", "fn main() {}");
    write_file(tmp.path(), "apps/worker/src/main.rs", "fn main() {}");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-12");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/devctl/src", "apps/backend/src", "apps/worker/src"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
}

#[test]
fn empty_src_dirs_still_hit_every_mutated_rust_app() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/devctl/src");
    create_dir(tmp.path(), "apps/backend/src");
    create_dir(tmp.path(), "apps/worker/src");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-12");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/devctl/src", "apps/backend/src", "apps/worker/src"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
}
