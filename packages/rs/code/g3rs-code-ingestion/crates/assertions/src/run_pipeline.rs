#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_pipeline_reports_expected_findings_on_real_source_files(results: &[G3CheckResult]) {
    for (file, id) in [
        ("src/has_todo.rs", "g3rs-code/todo-macros"),
        ("src/direct_std_fs.rs", "g3rs-code/direct-fs-usage"),
        ("src/panic_probe.rs", "g3rs-code/panic-macro"),
    ] {
        assert_result_set(results, file, 1, &[id]);
    }
    assert_no_results_for_file(results, "src/clean_file.rs");
}

pub fn assert_pipeline_reports_new_single_file_ast_rules(results: &[G3CheckResult]) {
    for (file, id) in [
        ("src/crate_allow.rs", "g3rs-code/crate-level-allow"),
        (
            "src/unused_crate_deps.rs",
            "g3rs-code/unused-crate-dependencies-allow",
        ),
        (
            "src/item_allow_missing_reason.rs",
            "g3rs-code/item-level-allow-without-reason",
        ),
        (
            "src/item_allow_with_reason.rs",
            "g3rs-code/item-level-allow-with-reason",
        ),
        ("src/garde_skip.rs", "g3rs-code/garde-skip-with-comment"),
        (
            "src/garde_skip_no_comment.rs",
            "g3rs-code/garde-skip-without-comment",
        ),
        (
            "src/too_many_lines.rs",
            "g3rs-code/too-many-effective-code-lines",
        ),
        ("src/too_many_uses.rs", "g3rs-code/too-many-use-imports"),
        (
            "src/use_error_boundary_clean.rs",
            "g3rs-code/many-use-imports",
        ),
        ("src/many_uses.rs", "g3rs-code/many-use-imports"),
        ("src/large_struct.rs", "g3rs-code/large-type-inventory"),
        ("src/large_enum.rs", "g3rs-code/large-type-inventory"),
        ("src/path_reason.rs", "g3rs-code/path-attr-with-reason"),
        (
            "src/path_missing_reason.rs",
            "g3rs-code/path-attr-with-reason",
        ),
        ("src/path_weak_reason.rs", "g3rs-code/path-attr-with-reason"),
        ("src/path_escape.rs", "g3rs-code/path-attr-with-reason"),
        (
            "src/path_cfg_attr_reason.rs",
            "g3rs-code/path-attr-with-reason",
        ),
        (
            "src/cfg_attr_unknown.rs",
            "g3rs-code/cfg-attr-allow-inventory",
        ),
        (
            "src/deny_without_reason.rs",
            "g3rs-code/deny-forbid-without-reason",
        ),
        ("src/cfg_attr.rs", "g3rs-code/always-true-cfg-attr-bypass"),
        ("src/ffi.rs", "g3rs-code/extern-allow"),
        ("src/include_probe.rs", "g3rs-code/include-bypass"),
        (
            "src/forbid_inventory.rs",
            "g3rs-code/deny-forbid-without-reason",
        ),
        (
            "tests/expect_probe.rs",
            "g3rs-code/test-expect-message-quality",
        ),
        ("src/generic_probe.rs", "g3rs-code/generic-parameter-cap"),
        (
            "src/public_weak_error.rs",
            "g3rs-code/public-weak-error-forms",
        ),
        (
            "src/public_trait_weak_error.rs",
            "g3rs-code/public-weak-error-forms",
        ),
        (
            "src/public_impl_weak_error.rs",
            "g3rs-code/public-weak-error-forms",
        ),
        (
            "src/public_str_ref_error.rs",
            "g3rs-code/public-weak-error-forms",
        ),
        ("src/string_dispatch.rs", "g3rs-code/string-dispatch-cap"),
    ] {
        assert_result_set(results, file, 1, &[id]);
    }

    assert_result_set(
        results,
        "src/impl_allow.rs",
        2,
        &[
            "g3rs-code/item-level-allow-without-reason",
            "g3rs-code/impl-allow-blast-radius",
        ],
    );
    assert_result_set(
        results,
        "src/fs_glob.rs",
        2,
        &["g3rs-code/direct-fs-usage", "g3rs-code/fs-glob-import"],
    );

    for (file, title) in [
        ("src/path_reason.rs", "#[path] with reason"),
        ("src/path_missing_reason.rs", "#[path] without reason"),
        ("src/path_weak_reason.rs", "#[path] reason too weak"),
        ("src/path_escape.rs", "#[path] escapes parent directory"),
    ] {
        assert_eq!(
            finding_for_file(results, file).title(),
            title,
            "{results:#?}"
        );
    }
    assert!(
        finding_for_file(results, "src/forbid_inventory.rs").inventory(),
        "{results:#?}"
    );

    for (file, severity) in [
        ("src/large_trait.rs", G3Severity::Warn),
        ("src/large_trait_boundary.rs", G3Severity::Warn),
        ("src/public_field_bag.rs", G3Severity::Warn),
        ("src/public_field_warn_boundary.rs", G3Severity::Warn),
        ("src/public_field_error_boundary.rs", G3Severity::Error),
    ] {
        let id = if file.starts_with("src/large_trait") {
            "g3rs-code/large-trait-surface"
        } else {
            "g3rs-code/public-struct-named-fields"
        };
        assert_file_result_count(results, file, 1);
        assert_has_result_id_with_severity(results, file, id, severity);
    }

    for file in [
        "src/raw_string_payload_only.rs",
        "src/use_warn_boundary_clean.rs",
        "tests/use_exempt.rs",
        "src/path_cfg_attr_known_false.rs",
        "src/path_sidecar_exempt.rs",
        "src/small_trait.rs",
        "src/private_field_struct.rs",
        "src/typed_public_error.rs",
        "src/private_weak_error.rs",
    ] {
        assert_no_results_for_file(results, file);
    }
}

pub fn assert_pipeline_reports_effective_line_and_dispatch_boundaries(results: &[G3CheckResult]) {
    assert_no_results_for_file(results, "src/line_cap.rs");
    assert_result_set(
        results,
        "src/line_over_cap.rs",
        1,
        &["g3rs-code/too-many-effective-code-lines"],
    );
    assert_no_results_for_file(results, "src/string_dispatch_clean.rs");
}

pub fn assert_pipeline_reports_trait_and_public_error_boundaries(results: &[G3CheckResult]) {
    assert_no_results_for_file(results, "src/trait_clean.rs");
    for file in ["src/trait_warn.rs", "src/trait_error.rs"] {
        assert_result_set(results, file, 1, &["g3rs-code/large-trait-surface"]);
    }
    for file in [
        "src/public_string_error.rs",
        "src/public_str_error.rs",
        "src/public_anyhow_error.rs",
        "src/public_box_error.rs",
        "src/public_trait_error.rs",
        "src/public_impl_error.rs",
    ] {
        assert_result_set(results, file, 1, &["g3rs-code/public-weak-error-forms"]);
    }
    assert_no_results_for_file(results, "src/private_string_error.rs");
}

pub fn assert_pipeline_reports_include_str_traversal(results: &[G3CheckResult]) {
    assert_result_set(
        results,
        "src/include_str_escape.rs",
        1,
        &["g3rs-code/include-bypass"],
    );
    assert_eq!(
        finding_for_file(results, "src/include_str_escape.rs").title(),
        "include path traversal",
        "{results:#?}"
    );
}

pub fn assert_pipeline_preserves_current_test_owned_rule_behavior(results: &[G3CheckResult]) {
    assert_result_count(results, 2);
    assert!(
        results
            .iter()
            .all(|result| result.id() == "g3rs-code/todo-macros"),
        "{results:#?}"
    );
}

pub fn assert_pipeline_emits_explicit_input_failure_for_parse_error(results: &[G3CheckResult]) {
    assert_result_count(results, 1);
    let result = &results[0];
    assert_eq!(result.id(), "g3rs-code/input-failures");
    assert_eq!(result.title(), "code-family input failure");
    assert_eq!(result.file(), Some("src/broken.rs"));
    assert!(
        result
            .message()
            .starts_with("Failed to parse Rust source file:"),
        "unexpected message: {result:#?}"
    );
}

pub fn assert_pipeline_keeps_other_findings_when_one_file_fails_to_parse(
    results: &[G3CheckResult],
) {
    assert_has_result_id(results, "src/broken.rs", "g3rs-code/input-failures");
    assert_has_result_id(results, "src/has_todo.rs", "g3rs-code/todo-macros");
}

pub fn assert_pipeline_classifies_custom_target_paths_before_checks_run(results: &[G3CheckResult]) {
    for (file, severity) in [
        ("lib/api.rs", G3Severity::Warn),
        ("cmd/worker.rs", G3Severity::Error),
    ] {
        assert_file_result_count(results, file, 1);
        assert_has_result_id_with_severity(
            results,
            file,
            "g3rs-code/large-trait-surface",
            severity,
        );
    }
}

/// `assert_file_result_count` test helper.
fn assert_file_result_count(results: &[G3CheckResult], file: &str, expected: usize) {
    let actual = results
        .iter()
        .filter(|result| result.file() == Some(file))
        .count();
    assert_eq!(actual, expected, "{results:#?}");
}

/// `assert_has_result_id` test helper.
fn assert_has_result_id(results: &[G3CheckResult], file: &str, id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.file() == Some(file) && result.id() == id),
        "{results:#?}"
    );
}

/// `assert_has_result_id_with_severity` test helper.
fn assert_has_result_id_with_severity(
    results: &[G3CheckResult],
    file: &str,
    id: &str,
    severity: G3Severity,
) {
    assert!(
        results.iter().any(|result| {
            result.file() == Some(file) && result.id() == id && result.severity() == severity
        }),
        "{results:#?}"
    );
}

/// `assert_no_results_for_file` test helper.
fn assert_no_results_for_file(results: &[G3CheckResult], file: &str) {
    assert!(
        !results.iter().any(|result| result.file() == Some(file)),
        "{results:#?}"
    );
}

/// `assert_result_count` test helper.
fn assert_result_count(results: &[G3CheckResult], expected: usize) {
    assert_eq!(results.len(), expected, "{results:#?}");
}

/// Returns the first finding in `results` that points at `file`. Panics if no such finding exists.
fn finding_for_file<'a>(results: &'a [G3CheckResult], file: &str) -> &'a G3CheckResult {
    results
        .iter()
        .find(|result| result.file() == Some(file))
        .expect("expected finding for file")
}

/// `assert_result_set` test helper.
fn assert_result_set(results: &[G3CheckResult], file: &str, count: usize, ids: &[&str]) {
    assert_file_result_count(results, file, count);
    for id in ids {
        assert_has_result_id(results, file, id);
    }
}
