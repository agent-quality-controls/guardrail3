use super::super::test_support::{
    RUST_APPS, copy_fixture, create_dir, errors_by_id, remove_dir, run_family, write_file,
};

#[test]
fn golden_has_no_rule_01_errors() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(errors.is_empty(), "golden should pass rule 01: {errors:#?}");
}

#[test]
fn missing_outer_crates_dir_errors_per_rust_app() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert_eq!(errors.len(), 3, "expected one error per Rust app: {errors:#?}");
    for app in RUST_APPS {
        let error = errors
            .iter()
            .find(|error| error.title.contains(app))
            .expect("missing per-app error");
        assert!(
            error.title.contains("missing crates/"),
            "expected missing crates title, got: {:?}",
            error
        );
        let expected = format!("apps/{app}");
        assert_eq!(error.file.as_deref(), Some(expected.as_str()));
    }
}

#[test]
fn empty_or_file_crates_dir_is_treated_like_missing() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");
    write_file(tmp.path(), "apps/devctl/crates", "not a dir");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert_eq!(errors.len(), 1, "expected one file-as-crates error: {errors:#?}");
    assert!(errors[0].title.contains("missing crates/"));

    remove_file_and_replace_empty(tmp.path(), "apps/devctl/crates");
    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert_eq!(errors.len(), 1, "expected one empty-dir error: {errors:#?}");
    assert!(errors[0].title.contains("missing crates/"));
}

fn remove_file_and_replace_empty(root: &std::path::Path, rel: &str) {
    std::fs::remove_file(root.join(rel)).expect("remove file");
    create_dir(root, rel);
}
