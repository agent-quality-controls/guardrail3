use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_04_loose_files as assertions;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

#[test]
fn symlinked_gitkeep_is_not_treated_as_the_allowed_real_gitkeep_in_outer_container() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/app/.gitkeep"),
    )
    .expect("failed to create symlink fixture for hexarch test");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/app",
        1,
        &["loose files"],
        &[],
        &[".gitkeep"],
        &[],
    );
}

#[test]
fn symlinked_gitkeep_is_not_treated_as_the_allowed_real_gitkeep_in_nested_container() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join(format!("{}/app/handlers", inner_hex())),
        tmp.path().join(format!("{}/app/.gitkeep", inner_hex())),
    )
    .expect("failed to create symlink fixture for hexarch test");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        &format!("{}/app", inner_hex()),
        1,
        &["loose files"],
        &[],
        &[".gitkeep"],
        &[],
    );
}

#[test]
fn loose_non_gitkeep_symlink_is_reported_as_a_bad_file() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/app/core"),
        tmp.path().join("apps/devctl/crates/app/mod.rs"),
    )
    .expect("failed to create symlink fixture for hexarch test");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/app",
        1,
        &["loose files"],
        &[],
        &["mod.rs"],
        &[],
    );
}
