use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_04_loose_files as assertions;

fn single_container_results(file_name: &str) -> Vec<guardrail3_domain_report::CheckResult> {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/app";
    write_file(tmp.path(), &format!("{container}/{file_name}"), "stray");
    super::run_family(tmp.path())
}

#[test]
fn cargo_toml_is_still_a_loose_file() {
    let results = single_container_results("Cargo.toml");
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/app",
        1,
        &["loose files"],
        &[],
        &["Cargo.toml"],
        &[],
    );
}

#[test]
fn hidden_files_other_than_gitkeep_are_loose_files() {
    let results = single_container_results(".hidden");
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/app",
        1,
        &["loose files"],
        &[],
        &[".hidden"],
        &[],
    );
}

#[test]
fn gitignore_is_not_gitkeep() {
    let results = single_container_results(".gitignore");
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/app",
        1,
        &["loose files"],
        &[],
        &[".gitignore"],
        &[],
    );
}

#[test]
fn near_miss_gitkeep_name_is_not_exempt() {
    let results = single_container_results(".gitkeep.bak");
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/app",
        1,
        &["loose files"],
        &[],
        &[".gitkeep.bak"],
        &[],
    );
}
