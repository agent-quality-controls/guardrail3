use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_11_root_workspace_doesnt_include_apps as assertions;

#[test]
fn root_workspace_including_all_rust_apps_hits_every_owned_app_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/devctl\", \"apps/backend\", \"apps/worker\"]\nresolver = \"2\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_title_set(
        &results,
        "",
        &[
            "root workspace includes app member `apps/devctl`",
            "root workspace includes app member `apps/backend`",
            "root workspace includes app member `apps/worker`",
        ],
    );
    assertions::assert_error_file_set(&results, "", 3, &["Cargo.toml"]);
}

#[test]
fn normalized_root_workspace_app_member_still_hits_rule_11() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"./apps/devctl/\"]\nresolver = \"2\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_title_contains(&results, "", 1, &["Cargo.toml"], &["./apps/devctl/"]);
}

#[test]
fn absolute_root_workspace_app_member_still_hits_rule_11() {
    let tmp = copy_fixture();
    let absolute_member = tmp.path().join("apps/devctl").display().to_string();
    write_file(
        tmp.path(),
        "Cargo.toml",
        &format!(
            "[workspace]\nmembers = [\"packages/shared-types\", \"{absolute_member}\"]\nresolver = \"2\"\n"
        ),
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_title_contains(&results, "", 1, &["Cargo.toml"], &[&absolute_member]);
}

#[test]
fn app_subpath_member_still_hits_rule_11() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/devctl/crates/domain/types\"]\nresolver = \"2\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_title_contains(
        &results,
        "",
        1,
        &["Cargo.toml"],
        &["apps/devctl/crates/domain/types"],
    );
}

#[test]
fn root_workspace_glob_covering_apps_hits_every_owned_app_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/*\"]\nresolver = \"2\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_title_set(
        &results,
        "",
        &["root workspace includes app member `apps/*`"],
    );
    assertions::assert_error_file_set(&results, "", 1, &["Cargo.toml"]);
}

#[test]
fn normalized_package_member_does_not_false_positive_under_rule_11() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"./packages/shared-types/\"]\nresolver = \"2\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
