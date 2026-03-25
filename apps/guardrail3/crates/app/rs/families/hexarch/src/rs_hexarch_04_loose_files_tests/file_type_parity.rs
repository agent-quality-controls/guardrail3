use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

fn assert_single_container_hit(file_name: &str) {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/app";
    write_file(tmp.path(), &format!("{container}/{file_name}"), "stray");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(errors[0].file.as_deref(), Some(container), "{errors:#?}");
    assert!(
        errors[0].message.contains(file_name),
        "expected message to mention {file_name}: {errors:#?}"
    );
}

#[test]
fn cargo_toml_is_still_a_loose_file() {
    assert_single_container_hit("Cargo.toml");
}

#[test]
fn hidden_files_other_than_gitkeep_are_loose_files() {
    assert_single_container_hit(".hidden");
}

#[test]
fn gitignore_is_not_gitkeep() {
    assert_single_container_hit(".gitignore");
}

#[test]
fn near_miss_gitkeep_name_is_not_exempt() {
    assert_single_container_hit(".gitkeep.bak");
}
