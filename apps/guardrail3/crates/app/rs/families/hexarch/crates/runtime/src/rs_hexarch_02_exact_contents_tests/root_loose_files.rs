const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;

#[test]
fn root_loose_files_hit_each_owned_hex_root_once() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        Some(&["loose files in"]),
        None,
        Some(&["mod.rs"]),
    );
}

#[test]
fn root_gitkeep_is_still_allowed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/.gitkeep", "");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn nested_root_gitkeep_is_still_allowed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{}/.gitkeep", inner_hex()), "");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(&results, "", inner_hex(), 0, &[], &[], &[], &[]);
}

#[test]
fn root_gitignore_is_a_loose_file_not_an_allowed_dotfile() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/.gitignore", "*\n");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        1,
        &["loose files"],
        &[],
        &[".gitignore"],
        &[],
    );
}

#[test]
fn loose_cargo_toml_at_crates_root_is_still_reported_as_a_bad_file() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/Cargo.toml",
        "[package]\nname = \"wrong-place\"\nversion = \"0.1.0\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        1,
        &["loose files"],
        &[],
        &["Cargo.toml"],
        &[],
    );
}

#[test]
fn symlinked_gitkeep_to_file_is_not_treated_as_the_allowed_real_gitkeep() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/.gitkeep"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        1,
        &["loose files"],
        &[],
        &[".gitkeep"],
        &[],
    );
}

#[test]
fn symlinked_gitkeep_to_directory_is_not_treated_as_the_allowed_real_gitkeep() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/app"),
        tmp.path().join("apps/devctl/crates/.gitkeep"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        1,
        &["loose files"],
        &[],
        &[".gitkeep"],
        &[],
    );
}
