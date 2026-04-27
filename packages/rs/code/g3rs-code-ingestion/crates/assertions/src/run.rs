use cargo_toml_parser::types::LintValue;
use g3rs_code_types as code_types;
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn require_source_file<'a>(
    inputs: &'a [code_types::G3RsCodeSourceChecksInput],
    rel_path: &str,
) -> &'a code_types::G3RsCodeSourceChecksInput {
    let index = inputs
        .iter()
        .position(|input| input.source_file.rel_path == rel_path);
    assert!(
        index.is_some(),
        "missing ingested source file {rel_path}; inputs: {inputs:#?}"
    );
    &inputs[index.unwrap_or(0)]
}

pub fn assert_source_file(
    input: &code_types::G3RsCodeSourceChecksInput,
    rel_path: &str,
    is_test: bool,
    profile_name: Option<&str>,
    is_library_root: bool,
    content: &str,
) {
    assert_eq!(input.source_file.rel_path, rel_path, "unexpected rel_path");
    assert_eq!(input.source_file.is_test, is_test, "unexpected is_test");
    assert_eq!(
        input.source_file.profile_name.as_deref(),
        profile_name,
        "unexpected profile_name"
    );
    assert_eq!(
        input.source_file.is_library_root, is_library_root,
        "unexpected is_library_root"
    );
    assert_eq!(input.source_file.content, content, "unexpected content");
    assert!(
        matches!(
            input.parsed_source,
            code_types::G3RsCodeParsedSourceState::Parsed(_)
        ),
        "unexpected parsed_source: {input:#?}"
    );
}

pub fn assert_shared_crate(input: &code_types::G3RsCodeSourceChecksInput) {
    assert!(input.is_shared_crate, "{input:#?}");
}

pub fn assert_not_shared_crate(input: &code_types::G3RsCodeSourceChecksInput) {
    assert!(!input.is_shared_crate, "{input:#?}");
}

pub fn assert_source_parse_failure(input: &code_types::G3RsCodeSourceChecksInput, rel_path: &str) {
    assert_eq!(input.source_file.rel_path, rel_path, "unexpected rel_path");
    assert!(
        matches!(
            input.parsed_source,
            code_types::G3RsCodeParsedSourceState::Invalid { .. }
        ),
        "unexpected parsed_source: {input:#?}"
    );
}

pub fn assert_source_waiver(
    input: &code_types::G3RsCodeSourceChecksInput,
    rule: &str,
    file: &str,
    selector: &str,
    reason: &str,
) {
    let waiver = input
        .waivers
        .iter()
        .find(|waiver| waiver.rule == rule && waiver.file == file && waiver.selector == selector);
    assert!(waiver.is_some(), "{input:#?}");
    let waiver =
        waiver.expect("assert_source_waiver should only unwrap an asserted-present waiver");
    assert_eq!(waiver.reason, reason, "{input:#?}");
}

pub fn assert_code_ast_results(lib_results: &[G3CheckResult], test_results: &[G3CheckResult]) {
    assert!(
        lib_results
            .iter()
            .any(|result| result.id() == "g3rs-code/todo-macros"),
        "lib input should preserve todo! detection: {lib_results:#?}"
    );
    assert!(
        test_results.is_empty(),
        "test-owned source should preserve current no-findings behavior for the migrated rules: {test_results:#?}"
    );
}

pub fn require_config_file<'a>(
    input: &'a code_types::G3RsCodeConfigChecksInput,
    rel_path: &str,
) -> &'a code_types::G3RsCodeConfigFile {
    let index = input
        .files
        .iter()
        .position(|file| file.rel_path == rel_path);
    assert!(
        index.is_some(),
        "missing config file {rel_path}; input: {input:#?}"
    );
    &input.files[index.unwrap_or(0)]
}

pub fn assert_exception_comments(
    input: &code_types::G3RsCodeConfigChecksInput,
    rel_path: &str,
    expected: &[(usize, &str)],
) {
    let comments = input
        .exception_comments
        .iter()
        .filter(|comment| comment.rel_path == rel_path)
        .collect::<Vec<&code_types::G3RsCodeExceptionComment>>();

    assert_eq!(comments.len(), expected.len(), "{input:#?}");
    for (comment, (line, text)) in comments.iter().zip(expected.iter()) {
        assert_eq!(comment.line, *line, "{input:#?}");
        assert_eq!(comment.text, *text, "{input:#?}");
    }
}

pub fn assert_parser_backed_config_files_only(input: &code_types::G3RsCodeConfigChecksInput) {
    assert!(
        input.files.iter().all(|file| {
            matches!(
                file.kind,
                code_types::G3RsCodeConfigFileKind::Guardrail3RsToml { .. }
                    | code_types::G3RsCodeConfigFileKind::ClippyToml { .. }
                    | code_types::G3RsCodeConfigFileKind::DenyToml { .. }
                    | code_types::G3RsCodeConfigFileKind::CargoToml { .. }
                    | code_types::G3RsCodeConfigFileKind::RustfmtToml { .. }
                    | code_types::G3RsCodeConfigFileKind::RustToolchainToml { .. }
            )
        }),
        "code config inputs must not expose raw Text files: {input:#?}"
    );
}

pub fn assert_workspace_unsafe_code_level(
    input: &code_types::G3RsCodeConfigChecksInput,
    rel_path: &str,
    expected: &str,
) {
    let file = require_config_file(input, rel_path);
    assert!(
        matches!(
            file.kind,
            code_types::G3RsCodeConfigFileKind::CargoToml { .. }
        ),
        "expected Cargo.toml config file in {rel_path}"
    );
    let code_types::G3RsCodeConfigFileKind::CargoToml { cargo } = &file.kind else {
        return;
    };
    assert_eq!(
        cargo
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.lints.as_ref())
            .and_then(|lints| lints.tools.get("rust"))
            .and_then(|tool| tool.get("unsafe_code")),
        Some(&LintValue::Level(expected.to_owned()))
    );
}

pub fn assert_workspace_unsafe_code_detailed_level(
    input: &code_types::G3RsCodeConfigChecksInput,
    rel_path: &str,
    expected: &str,
) {
    let file = require_config_file(input, rel_path);
    assert!(
        matches!(
            file.kind,
            code_types::G3RsCodeConfigFileKind::CargoToml { .. }
        ),
        "expected Cargo.toml config file in {rel_path}"
    );
    let code_types::G3RsCodeConfigFileKind::CargoToml { cargo } = &file.kind else {
        return;
    };
    let value = cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.lints.as_ref())
        .and_then(|lints| lints.tools.get("rust"))
        .and_then(|tool| tool.get("unsafe_code"));
    match value {
        Some(LintValue::Detailed(detail)) => assert_eq!(detail.level, expected),
        _ => assert!(false, "expected detailed unsafe_code lint in {rel_path}"),
    }
}

pub fn assert_config_file_paths(input: &code_types::G3RsCodeConfigChecksInput, expected: &[&str]) {
    let actual = input
        .files
        .iter()
        .map(|file| file.rel_path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "{input:#?}");
}

pub fn assert_results_empty(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

pub fn assert_result_count(results: &[G3CheckResult], expected: usize) {
    assert_eq!(results.len(), expected, "{results:#?}");
}

pub fn assert_file_result_count(results: &[G3CheckResult], file: &str, expected: usize) {
    let actual = results
        .iter()
        .filter(|result| result.file() == Some(file))
        .count();
    assert_eq!(actual, expected, "{results:#?}");
}

pub fn assert_has_result_id(results: &[G3CheckResult], file: &str, id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.file() == Some(file) && result.id() == id),
        "{results:#?}"
    );
}

pub fn assert_has_result_id_with_severity(
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

pub fn assert_no_results_for_file(results: &[G3CheckResult], file: &str) {
    assert!(
        !results.iter().any(|result| result.file() == Some(file)),
        "{results:#?}"
    );
}

pub fn assert_single_parse_failed_error(err: &g3rs_code_ingestion_runtime::IngestionError) {
    assert!(
        matches!(
            err,
            g3rs_code_ingestion_runtime::IngestionError::ParseFailed { .. }
        ),
        "unexpected error: {err:?}"
    );
}

pub fn assert_single_unreadable_error(err: &g3rs_code_ingestion_runtime::IngestionError) {
    assert!(
        matches!(
            err,
            g3rs_code_ingestion_runtime::IngestionError::Unreadable { .. }
        ),
        "unexpected error: {err:?}"
    );
}

pub fn assert_root_cargo_paths(input: &code_types::G3RsCodeFileTreeChecksInput, expected: &[&str]) {
    let actual = input
        .roots
        .iter()
        .map(|root| root.cargo_rel_path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "{input:#?}");
}

pub fn assert_single_zero_structural_root(
    input: &code_types::G3RsCodeFileTreeChecksInput,
    cargo_rel_path: &str,
) {
    assert_eq!(input.roots.len(), 1, "{input:#?}");
    assert_eq!(input.roots[0].cargo_rel_path, cargo_rel_path);
    assert_eq!(input.roots[0].max_module_depth, 0);
    assert_eq!(input.roots[0].max_sibling_dirs, 0);
    assert_eq!(input.roots[0].max_sibling_rs_files, 0);
}

fn finding_for_file<'a>(results: &'a [G3CheckResult], file: &str) -> &'a G3CheckResult {
    results
        .iter()
        .find(|result| result.file() == Some(file))
        .expect("expected finding for file")
}

fn assert_result_set(results: &[G3CheckResult], file: &str, count: usize, ids: &[&str]) {
    assert_file_result_count(results, file, count);
    for id in ids {
        assert_has_result_id(results, file, id);
    }
}

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

pub fn assert_config_pipeline_reports_exception_comments_and_unsafe_code_lints(
    results: &[G3CheckResult],
) {
    assert_has_result_id(
        results,
        "Cargo.toml",
        "g3rs-code/exception-comment-inventory",
    );
    assert_has_result_id_with_severity(
        results,
        "Cargo.toml",
        "g3rs-code/unsafe-code-lint",
        G3Severity::Info,
    );
    assert_has_result_id(
        results,
        "deny.toml",
        "g3rs-code/exception-comment-inventory",
    );
}

pub fn assert_config_pipeline_stays_clean_for_harmless_comments_and_non_workspace_manifests(
    results: &[G3CheckResult],
) {
    assert_result_count(results, 1);
    assert_result_set(
        results,
        "crates/core/Cargo.toml",
        1,
        &["g3rs-code/exception-comment-inventory"],
    );
    for file in ["Cargo.toml", "deny.toml"] {
        assert_no_results_for_file(results, file);
    }
}

pub fn assert_config_pipeline_reports_exact_exception_comment_counts(results: &[G3CheckResult]) {
    assert_result_count(results, 3);
    for (file, count) in [("Cargo.toml", 2), ("deny.toml", 1)] {
        assert_file_result_count(results, file, count);
    }
}

pub fn assert_config_pipeline_ignores_foreign_nested_repo_findings(results: &[G3CheckResult]) {
    assert_result_count(results, 1);
    assert_result_set(results, "Cargo.toml", 1, &["g3rs-code/unsafe-code-lint"]);
    for file in ["vendor/foreign/Cargo.toml", "vendor/foreign/deny.toml"] {
        assert_no_results_for_file(results, file);
    }
}

pub fn assert_config_pipeline_reports_deny_through_full_lane(results: &[G3CheckResult]) {
    assert_result_count(results, 1);
    assert_has_result_id_with_severity(
        results,
        "Cargo.toml",
        "g3rs-code/unsafe-code-lint",
        G3Severity::Error,
    );
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
