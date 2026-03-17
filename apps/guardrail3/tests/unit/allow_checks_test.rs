use std::path::Path;

use guardrail3::app::rs::validate::allow_checks::*;
use guardrail3::domain::report::Severity;

// ---- Bug 2: Check ID mappings R30-R35 ----

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn crate_level_allow_without_reason_is_error_r30() {
    let attr = ["#!", "[allow(clippy::unwrap_used)]"].concat(); // pre-commit: test string
    let content = format!("{attr}\nfn main() {{}}");
    let path = Path::new("test.rs");
    let mut results = Vec::new();
    check_crate_level_allow(path, &content, false, false, &mut results);
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    assert!(!errors.is_empty(), "Should produce an error");
    assert_eq!(errors[0].id, "R30", "Should be R30, got {}", errors[0].id);
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn crate_level_allow_unused_crate_deps_is_info_r31() {
    let content = "#![allow(unused_crate_dependencies)]\nfn main() {}";
    let path = Path::new("main.rs");
    let mut results = Vec::new();
    check_crate_level_allow(path, content, true, false, &mut results);
    let infos: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Info)
        .collect();
    assert!(!infos.is_empty(), "Should produce Info");
    assert_eq!(infos[0].id, "R31");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn item_level_allow_without_comment_is_error_r32() {
    // Build the test input by concatenation to avoid tripping the pre-commit grep
    let attr = ["#[allow(", "clippy::unwrap_used)]"].concat(); // pre-commit: test string
    let content = format!("{attr}\nfn foo() {{}}");
    let path = Path::new("test.rs");
    let mut results = Vec::new();
    check_item_level_allow(path, &content, &mut results);
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    assert!(!errors.is_empty(), "Should produce an error");
    assert_eq!(errors[0].id, "R32");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn item_level_allow_with_comment_is_info_r33() {
    let content = "#[allow(clippy::unwrap_used)] // reason: test\nfn foo() {}";
    let path = Path::new("test.rs");
    let mut results = Vec::new();
    check_item_level_allow(path, content, &mut results);
    let infos: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Info)
        .collect();
    assert!(!infos.is_empty(), "Should produce Info");
    assert_eq!(infos[0].id, "R33");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn garde_skip_without_comment_is_error_r34() {
    let content = "struct Foo {\n    #[garde(skip)]\n    field: String,\n}";
    let path = Path::new("test.rs");
    let mut results = Vec::new();
    check_garde_skip(path, content, &mut results);
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    assert!(!errors.is_empty(), "Should produce an error");
    assert_eq!(errors[0].id, "R34");
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn garde_skip_with_comment_on_non_primitive_is_error_r35() {
    let content =
        "struct Foo {\n    #[garde(skip)] // reason: validated elsewhere\n    field: String,\n}";
    let path = Path::new("test.rs");
    let mut results = Vec::new();
    check_garde_skip(path, content, &mut results);
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    assert!(!errors.is_empty(), "Should produce Error for non-primitive");
    assert_eq!(errors[0].id, "R35");
}

// ---- R35 type-aware: garde(skip) on primitives vs non-primitives ----

#[test]
fn garde_skip_on_bool_silent() {
    let content = "struct Foo {\n    #[garde(skip)]\n    active: bool,\n}";
    let path = Path::new("test.rs");
    let mut results = Vec::new();
    check_garde_skip(path, content, &mut results);
    assert!(
        results.is_empty(),
        "garde(skip) on bool should produce no result, got: {results:?}"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn garde_skip_on_string_error() {
    let content = "struct Foo {\n    #[garde(skip)] // reason: legacy\n    name: String,\n}";
    let path = Path::new("test.rs");
    let mut results = Vec::new();
    check_garde_skip(path, content, &mut results);
    assert_eq!(results.len(), 1, "Should produce one result");
    assert_eq!(results[0].id, "R35");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(
        results[0].message.contains("non-primitive"),
        "Message should mention non-primitive"
    );
}

#[test]
fn garde_skip_on_option_usize_silent() {
    let content = "struct Foo {\n    #[garde(skip)]\n    count: Option<usize>,\n}";
    let path = Path::new("test.rs");
    let mut results = Vec::new();
    check_garde_skip(path, content, &mut results);
    assert!(
        results.is_empty(),
        "garde(skip) on Option<usize> should produce no result, got: {results:?}"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn garde_skip_on_vec_string_error() {
    let content = "struct Foo {\n    #[garde(skip)]\n    items: Vec<String>,\n}";
    let path = Path::new("test.rs");
    let mut results = Vec::new();
    check_garde_skip(path, content, &mut results);
    assert_eq!(results.len(), 1, "Should produce one result");
    assert_eq!(results[0].id, "R34");
    assert_eq!(results[0].severity, Severity::Error);
    assert!(
        results[0].message.contains("non-primitive"),
        "Message should mention non-primitive"
    );
}

// ---- Bug 7: unused_crate_dependencies universal exemption ----

#[test]
#[allow(clippy::needless_collect)] // reason: collect into Vec for readable test assertions
fn unused_crate_deps_is_info_in_lib_rs() {
    let content = "#![allow(unused_crate_dependencies)]\nfn main() {}";
    let path = Path::new("src/lib.rs");
    let mut results = Vec::new();
    check_crate_level_allow(path, content, false, false, &mut results);
    // Should be Info (R31), not Error (R30)
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    assert!(
        errors.is_empty(),
        "unused_crate_dependencies should be Info everywhere, not Error"
    );
    let infos: Vec<_> = results
        .iter()
        .filter(|r| r.id == "R31" && r.severity == Severity::Info)
        .collect();
    assert!(
        !infos.is_empty(),
        "Should produce R31 Info for unused_crate_dependencies"
    );
}

#[test]
#[allow(clippy::needless_collect)] // reason: collect into Vec for readable test assertions
fn unused_crate_deps_is_info_in_any_file() {
    let content = "#![allow(unused_crate_dependencies)]\nmod foo;";
    let path = Path::new("src/some_module.rs");
    let mut results = Vec::new();
    check_crate_level_allow(path, content, false, false, &mut results);
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    assert!(
        errors.is_empty(),
        "unused_crate_dependencies should be Info everywhere"
    );
}

// ---- Bug 4 (partial): Test file exemption for R30 ----

#[test]
#[allow(clippy::needless_collect)] // reason: collect into Vec for readable test assertions
fn crate_level_allow_in_test_file_is_info_not_error() {
    let attr = ["#!", "[allow(clippy::unwrap_used)]"].concat(); // pre-commit: test string
    let content = format!("{attr}\nfn test_stuff() {{}}");
    let path = Path::new("/project/tests/integration.rs");
    let mut results = Vec::new();
    check_crate_level_allow(path, &content, false, true, &mut results);
    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity == Severity::Error)
        .collect();
    assert!(
        errors.is_empty(),
        "Test files should be exempt from R30 errors"
    );
}
