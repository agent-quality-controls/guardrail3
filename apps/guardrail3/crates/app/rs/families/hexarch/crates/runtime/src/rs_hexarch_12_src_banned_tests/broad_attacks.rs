use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_12_src_banned as assertions;
use super::{copy_fixture, create_dir, write_file};

#[test]
fn app_level_src_dirs_hit_every_mutated_rust_app() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    write_file(tmp.path(), "apps/backend/src/main.rs", "fn main() {}");
    write_file(tmp.path(), "apps/worker/src/main.rs", "fn main() {}");

    let results = super::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "");
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
fn src_in_one_rust_app_hits_only_that_app() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");

    let results = super::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/devctl/src"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected one-app hit set: {errors:#?}"
    );
}

#[test]
fn empty_src_dirs_still_hit_every_mutated_rust_app() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/devctl/src");
    create_dir(tmp.path(), "apps/backend/src");
    create_dir(tmp.path(), "apps/worker/src");

    let results = super::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "");
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
fn src_with_only_non_rust_files_still_hits_rule_12() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/README.md", "# readme\n");

    let results = super::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/devctl/src"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "src with non-rust files should still hit rule 12: {errors:#?}"
    );
}
