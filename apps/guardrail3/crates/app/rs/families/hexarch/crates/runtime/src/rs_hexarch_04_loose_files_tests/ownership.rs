use super::{copy_fixture, create_dir, empty_dir, remove_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_04_loose_files as assertions;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

#[test]
fn files_only_container_is_owned_by_rule_05_not_rule_04() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# stray");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/domain",
        0,
        &[],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "RS-HEXARCH-05",
        "apps/devctl/crates/domain",
        1,
        &[],
        &[],
        &["README.md"],
        &[],
    );
}

#[test]
fn empty_container_is_owned_by_rule_05_not_rule_04() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_error_count_matching_file(
        &results,
        "RS-HEXARCH-05",
        "apps/devctl/crates/domain",
        1,
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn missing_container_dir_does_not_emit_rule_04_for_that_path() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/domain",
        0,
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn removing_outer_adapters_parent_does_not_create_nested_rule_04_hits() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates/adapters");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching(
        &results,
        "",
        0,
        None,
        Some(inner_hex()),
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn ts_apps_and_packages_stay_out_of_scope() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/README.md",
        "# stray",
    );
    write_file(tmp.path(), "packages/shared-types/README.md", "# stray");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
#[cfg(unix)]
fn symlink_only_container_does_not_trigger_rule_04() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/domain/README.md"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/domain",
        0,
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn mixed_rule_04_and_rule_05_states_split_ownership_exactly() {
    let tmp = copy_fixture();

    remove_dir(tmp.path(), "apps/backend/crates/app/commands");
    write_file(
        tmp.path(),
        "apps/backend/crates/app/commands",
        "// replaced child dir",
    );

    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# stray");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/backend/crates/app",
        1,
        &["loose files"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "RS-HEXARCH-05",
        "apps/devctl/crates/domain",
        1,
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn mixed_root_structural_and_container_loose_files_split_across_neighbor_rules() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/stray.rs", "// root stray");
    create_dir(tmp.path(), "apps/devctl/crates/adapters/diagonal");
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/stray.rs",
        "// container stray",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "RS-HEXARCH-02",
        "apps/devctl/crates",
        1,
        &[],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "RS-HEXARCH-03",
        "apps/devctl/crates/adapters/diagonal",
        1,
        &[],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/domain",
        1,
        &[],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "RS-HEXARCH-05",
        0,
        None,
        None,
        &[],
        &[],
        &[],
        &[],
    );
}
