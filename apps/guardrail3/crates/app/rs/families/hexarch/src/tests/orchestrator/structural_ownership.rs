use crate::test_support::{
    assert_no_error, copy_fixture, create_dir, errors_by_id, remove_dir, run_family, write_file,
};

#[test]
fn missing_inner_hex_crates_is_not_owned_by_app_level_rule_01() {
    let tmp = copy_fixture();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 is app-level only and should not own nested missing-crates cases: {errors:#?}"
    );
}

#[test]
fn empty_inner_hex_crates_is_not_owned_by_app_level_rule_01() {
    let tmp = copy_fixture();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
    );
    std::fs::create_dir_all(
        tmp.path()
            .join("apps/backend/crates/adapters/inbound/mcp/crates"),
    )
    .expect("recreate nested crates dir");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 is app-level only and should not own nested empty-crates cases: {errors:#?}"
    );
}

#[test]
fn broken_nested_crates_symlink_is_not_owned_by_app_level_rule_01() {
    let tmp = copy_fixture();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
    );
    std::os::unix::fs::symlink(
        "/nonexistent/path",
        tmp.path()
            .join("apps/backend/crates/adapters/inbound/mcp/crates"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 is app-level only and should not own nested broken-symlink cases: {errors:#?}"
    );
}

#[test]
fn nested_crates_symlink_loop_does_not_become_a_rule_01_hit() {
    let tmp = copy_fixture();
    let inner = tmp
        .path()
        .join("apps/backend/crates/adapters/inbound/mcp/crates");
    let outer = tmp.path().join("apps/backend/crates");
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
    );
    std::os::unix::fs::symlink(&outer, &inner).expect("symlink");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 must not start owning nested symlink-loop cases after link-following: {errors:#?}"
    );
}

#[test]
fn gitkeep_only_outer_crates_is_not_owned_by_rule_01() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        std::fs::remove_dir_all(tmp.path().join(format!("apps/{app}/crates"))).expect("remove");
        create_dir(tmp.path(), &format!("apps/{app}/crates"));
        write_file(tmp.path(), &format!("apps/{app}/crates/.gitkeep"), "");
    }

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-01");
}

#[test]
fn top_level_file_inside_crates_counts_as_present_for_rule_01() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/README.md", "placeholder\n");

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-01");
}

#[test]
fn packages_crate_without_crates_dir_is_not_owned_by_rule_01() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "packages/phantom/Cargo.toml",
        "[package]\nname = \"phantom\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-01");
}

#[test]
fn missing_crates_and_banned_src_can_coexist_on_the_same_app() {
    let tmp = copy_fixture();
    std::fs::remove_dir_all(tmp.path().join("apps/devctl/crates")).expect("remove");
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}\n");

    let results = run_family(tmp.path());
    assert_eq!(
        errors_by_id(&results, "RS-HEXARCH-01").len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        errors_by_id(&results, "RS-HEXARCH-12").len(),
        1,
        "{results:#?}"
    );
}
