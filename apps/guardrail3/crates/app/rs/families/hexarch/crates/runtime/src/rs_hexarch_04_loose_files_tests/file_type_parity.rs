use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_04_loose_files as assertions;
use guardrail3_domain_report::CheckResult;
use test_support::{copy_fixture, write_file};

fn single_container_results(file_name: &str) -> Vec<CheckResult> {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/app";
    write_file(tmp.path(), &format!("{container}/{file_name}"), "stray");
    assertions::run_family(tmp.path())
}

#[test]
fn cargo_toml_is_still_a_loose_file() {
    let results = single_container_results("Cargo.toml");
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/devctl/crates/app"),
        "{errors:#?}"
    );
    assert!(errors[0].message.contains("Cargo.toml"), "{errors:#?}");
}

#[test]
fn hidden_files_other_than_gitkeep_are_loose_files() {
    let results = single_container_results(".hidden");
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/devctl/crates/app"),
        "{errors:#?}"
    );
    assert!(errors[0].message.contains(".hidden"), "{errors:#?}");
}

#[test]
fn gitignore_is_not_gitkeep() {
    let results = single_container_results(".gitignore");
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/devctl/crates/app"),
        "{errors:#?}"
    );
    assert!(errors[0].message.contains(".gitignore"), "{errors:#?}");
}

#[test]
fn near_miss_gitkeep_name_is_not_exempt() {
    let results = single_container_results(".gitkeep.bak");
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/devctl/crates/app"),
        "{errors:#?}"
    );
    assert!(errors[0].message.contains(".gitkeep.bak"), "{errors:#?}");
}
