use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_12_src_banned as assertions;
use super::{copy_fixture, create_dir, write_file};

#[test]
fn app_level_src_dirs_hit_every_mutated_rust_app() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");
    write_file(tmp.path(), "apps/backend/src/main.rs", "fn main() {}");
    write_file(tmp.path(), "apps/worker/src/main.rs", "fn main() {}");

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_set(
        &results,
        "",
        3,
        &[
            "apps/devctl/src",
            "apps/backend/src",
            "apps/worker/src",
        ],
    );
}

#[test]
fn src_in_one_rust_app_hits_only_that_app() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_set(&results, "", 1, &["apps/devctl/src"]);
}

#[test]
fn empty_src_dirs_still_hit_every_mutated_rust_app() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/devctl/src");
    create_dir(tmp.path(), "apps/backend/src");
    create_dir(tmp.path(), "apps/worker/src");

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_set(
        &results,
        "",
        3,
        &[
            "apps/devctl/src",
            "apps/backend/src",
            "apps/worker/src",
        ],
    );
}

#[test]
fn src_with_only_non_rust_files_still_hits_rule_12() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/README.md", "# readme\n");

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_set(&results, "", 1, &["apps/devctl/src"]);
}
