use super::super::super::test_support::{
    assert_no_error, copy_fixture, create_dir, errors_by_id, run_family, write_file,
};

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
fn single_app_gitkeep_only_outer_crates_is_not_owned_by_rule_01() {
    let tmp = copy_fixture();
    std::fs::remove_dir_all(tmp.path().join("apps/devctl/crates")).expect("remove");
    create_dir(tmp.path(), "apps/devctl/crates");
    write_file(tmp.path(), "apps/devctl/crates/.gitkeep", "");

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-01");
}

#[test]
fn existing_but_wrong_crates_contents_is_not_owned_by_rule_01() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/misc/.gitkeep", "");

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
fn unconfigured_app_dir_without_crates_does_not_hit_rule_01() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/fake-service/src");
    write_file(
        tmp.path(),
        "apps/fake-service/package.json",
        "{ \"name\": \"fake-service\" }\n",
    );

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

#[test]
fn valid_crates_with_banned_src_belongs_only_to_rule_12() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}\n");

    let results = run_family(tmp.path());
    assert_eq!(
        errors_by_id(&results, "RS-HEXARCH-01").len(),
        0,
        "{results:#?}"
    );
    assert_eq!(
        errors_by_id(&results, "RS-HEXARCH-12").len(),
        1,
        "{results:#?}"
    );
}
