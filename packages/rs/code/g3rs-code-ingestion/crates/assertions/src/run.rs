use cargo_toml_parser::types::LintValue;
use std::collections::BTreeMap;

use g3rs_code_types::{
    G3RsCodeConfigChecksInput, G3RsCodeConfigFile, G3RsCodeConfigFileKind,
    G3RsCodeExceptionComment, G3RsCodeFileTreeChecksInput, G3RsCodeSourceChecksInput,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn require_source_file<'a>(
    inputs: &'a [G3RsCodeSourceChecksInput],
    rel_path: &str,
) -> &'a G3RsCodeSourceChecksInput {
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
    input: &G3RsCodeSourceChecksInput,
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
}

pub fn assert_shared_crate(input: &G3RsCodeSourceChecksInput) {
    assert!(input.is_shared_crate, "{input:#?}");
}

pub fn assert_not_shared_crate(input: &G3RsCodeSourceChecksInput) {
    assert!(!input.is_shared_crate, "{input:#?}");
}

pub fn assert_source_waiver(
    input: &G3RsCodeSourceChecksInput,
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
    let waiver = waiver.unwrap_or_else(|| unreachable!());
    assert_eq!(waiver.reason, reason, "{input:#?}");
}

pub fn assert_code_ast_results(
    lib_results: &[G3CheckResult],
    test_results: &[G3CheckResult],
) {
    assert!(
        lib_results
            .iter()
            .any(|result| result.id() == "RS-CODE-SOURCE-13"),
        "lib input should preserve todo! detection: {lib_results:#?}"
    );
    assert!(
        test_results.is_empty(),
        "test-owned source should preserve current no-findings behavior for the migrated rules: {test_results:#?}"
    );
}

pub fn require_config_file<'a>(
    input: &'a G3RsCodeConfigChecksInput,
    rel_path: &str,
) -> &'a G3RsCodeConfigFile {
    let index = input.files.iter().position(|file| file.rel_path == rel_path);
    assert!(
        index.is_some(),
        "missing config file {rel_path}; input: {input:#?}"
    );
    &input.files[index.unwrap_or(0)]
}

pub fn assert_exception_comments(
    input: &G3RsCodeConfigChecksInput,
    rel_path: &str,
    expected: &[(usize, &str)],
) {
    let comments = input
        .exception_comments
        .iter()
        .filter(|comment| comment.rel_path == rel_path)
        .collect::<Vec<&G3RsCodeExceptionComment>>();

    assert_eq!(comments.len(), expected.len(), "{input:#?}");
    for (comment, (line, text)) in comments.iter().zip(expected.iter()) {
        assert_eq!(comment.line, *line, "{input:#?}");
        assert_eq!(comment.text, *text, "{input:#?}");
    }
}

pub fn assert_parser_backed_config_files_only(input: &G3RsCodeConfigChecksInput) {
    assert!(
        input.files.iter().all(|file| {
            matches!(
                file.kind,
                G3RsCodeConfigFileKind::Guardrail3RsToml { .. }
                    | G3RsCodeConfigFileKind::ClippyToml { .. }
                    | G3RsCodeConfigFileKind::DenyToml { .. }
                    | G3RsCodeConfigFileKind::CargoToml { .. }
                    | G3RsCodeConfigFileKind::RustfmtToml { .. }
                    | G3RsCodeConfigFileKind::RustToolchainToml { .. }
            )
        }),
        "code config inputs must not expose raw Text files: {input:#?}"
    );
}

pub fn assert_workspace_unsafe_code_level(
    input: &G3RsCodeConfigChecksInput,
    rel_path: &str,
    expected: &str,
) {
    let file = require_config_file(input, rel_path);
    assert!(
        matches!(file.kind, G3RsCodeConfigFileKind::CargoToml { .. }),
        "expected Cargo.toml config file in {rel_path}"
    );
    let G3RsCodeConfigFileKind::CargoToml { cargo } = &file.kind else {
        return;
    };
    assert_eq!(
        cargo.workspace
            .as_ref()
            .and_then(|workspace| workspace.lints.as_ref())
            .and_then(|lints| lints.tools.get("rust"))
            .and_then(|tool| tool.get("unsafe_code")),
        Some(&LintValue::Level(expected.to_owned()))
    );
}

pub fn assert_workspace_unsafe_code_detailed_level(
    input: &G3RsCodeConfigChecksInput,
    rel_path: &str,
    expected: &str,
) {
    let file = require_config_file(input, rel_path);
    assert!(
        matches!(file.kind, G3RsCodeConfigFileKind::CargoToml { .. }),
        "expected Cargo.toml config file in {rel_path}"
    );
    let G3RsCodeConfigFileKind::CargoToml { cargo } = &file.kind else {
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

pub fn assert_config_file_paths(input: &G3RsCodeConfigChecksInput, expected: &[&str]) {
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

pub fn assert_single_parse_failed_error(
    err: &g3rs_code_ingestion_runtime::IngestionError,
) {
    assert!(
        matches!(err, g3rs_code_ingestion_runtime::IngestionError::ParseFailed { .. }),
        "unexpected error: {err:?}"
    );
}

pub fn assert_single_unreadable_error(
    err: &g3rs_code_ingestion_runtime::IngestionError,
) {
    assert!(
        matches!(err, g3rs_code_ingestion_runtime::IngestionError::Unreadable { .. }),
        "unexpected error: {err:?}"
    );
}

pub fn assert_root_cargo_paths(input: &G3RsCodeFileTreeChecksInput, expected: &[&str]) {
    let actual = input
        .roots
        .iter()
        .map(|root| root.cargo_rel_path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "{input:#?}");
}

pub fn assert_single_zero_structural_root(
    input: &G3RsCodeFileTreeChecksInput,
    cargo_rel_path: &str,
) {
    assert_eq!(input.roots.len(), 1, "{input:#?}");
    assert_eq!(input.roots[0].cargo_rel_path, cargo_rel_path);
    assert_eq!(input.roots[0].max_module_depth, 0);
    assert_eq!(input.roots[0].max_sibling_dirs, 0);
    assert_eq!(input.roots[0].max_sibling_rs_files, 0);
}

fn findings_by_file(results: &[G3CheckResult]) -> BTreeMap<String, Vec<&G3CheckResult>> {
    let mut by_file = BTreeMap::<String, Vec<&G3CheckResult>>::new();
    for result in results {
        let key = result.file().unwrap_or("<none>").to_owned();
        by_file.entry(key).or_default().push(result);
    }
    by_file
}

pub fn assert_pipeline_reports_expected_findings_on_real_source_files(results: &[G3CheckResult]) {
    assert_file_result_count(results, "src/has_todo.rs", 1);
    assert_has_result_id(results, "src/has_todo.rs", "RS-CODE-SOURCE-13");
    assert_file_result_count(results, "src/direct_std_fs.rs", 1);
    assert_has_result_id(results, "src/direct_std_fs.rs", "RS-CODE-SOURCE-15");
    assert_file_result_count(results, "src/panic_probe.rs", 1);
    assert_has_result_id(results, "src/panic_probe.rs", "RS-CODE-SOURCE-16");
    assert_no_results_for_file(results, "src/clean_file.rs");
}

pub fn assert_config_pipeline_reports_exception_comments_and_unsafe_code_lints(
    results: &[G3CheckResult],
) {
    assert_has_result_id(results, "Cargo.toml", "RS-CODE-CONFIG-07");
    assert_has_result_id_with_severity(results, "Cargo.toml", "RS-CODE-CONFIG-12", G3Severity::Info);
    assert_has_result_id(results, "deny.toml", "RS-CODE-CONFIG-07");
}

pub fn assert_config_pipeline_stays_clean_for_harmless_comments_and_non_workspace_manifests(
    results: &[G3CheckResult],
) {
    assert_result_count(results, 1);
    assert_file_result_count(results, "crates/core/Cargo.toml", 1);
    assert_has_result_id(results, "crates/core/Cargo.toml", "RS-CODE-CONFIG-07");
    assert_no_results_for_file(results, "Cargo.toml");
    assert_no_results_for_file(results, "deny.toml");
}

pub fn assert_config_pipeline_reports_exact_exception_comment_counts(results: &[G3CheckResult]) {
    assert_result_count(results, 3);
    assert_file_result_count(results, "Cargo.toml", 2);
    assert_file_result_count(results, "deny.toml", 1);
}

pub fn assert_config_pipeline_ignores_foreign_nested_repo_findings(results: &[G3CheckResult]) {
    assert_result_count(results, 1);
    assert_file_result_count(results, "Cargo.toml", 1);
    assert_has_result_id(results, "Cargo.toml", "RS-CODE-CONFIG-12");
    assert_no_results_for_file(results, "vendor/foreign/Cargo.toml");
    assert_no_results_for_file(results, "vendor/foreign/deny.toml");
}

pub fn assert_config_pipeline_reports_deny_through_full_lane(results: &[G3CheckResult]) {
    assert_result_count(results, 1);
    assert_file_result_count(results, "Cargo.toml", 1);
    assert_has_result_id_with_severity(results, "Cargo.toml", "RS-CODE-CONFIG-12", G3Severity::Error);
}

pub fn assert_pipeline_reports_new_single_file_ast_rules(results: &[G3CheckResult]) {
    let by_file = findings_by_file(results);

    assert_file_result_count(results, "src/crate_allow.rs", 1);
    assert_has_result_id(results, "src/crate_allow.rs", "RS-CODE-SOURCE-01");
    assert_file_result_count(results, "src/unused_crate_deps.rs", 1);
    assert_has_result_id(results, "src/unused_crate_deps.rs", "RS-CODE-SOURCE-02");
    assert_file_result_count(results, "src/item_allow_missing_reason.rs", 1);
    assert_has_result_id(results, "src/item_allow_missing_reason.rs", "RS-CODE-SOURCE-03");
    assert_file_result_count(results, "src/item_allow_with_reason.rs", 1);
    assert_has_result_id(results, "src/item_allow_with_reason.rs", "RS-CODE-SOURCE-04");
    assert_file_result_count(results, "src/garde_skip.rs", 1);
    assert_has_result_id(results, "src/garde_skip.rs", "RS-CODE-SOURCE-06");
    assert_file_result_count(results, "src/garde_skip_no_comment.rs", 1);
    assert_has_result_id(results, "src/garde_skip_no_comment.rs", "RS-CODE-SOURCE-05");
    assert_file_result_count(results, "src/too_many_lines.rs", 1);
    assert_has_result_id(results, "src/too_many_lines.rs", "RS-CODE-SOURCE-09");
    assert_no_results_for_file(results, "src/raw_string_payload_only.rs");
    assert_file_result_count(results, "src/too_many_uses.rs", 1);
    assert_has_result_id(results, "src/too_many_uses.rs", "RS-CODE-SOURCE-10");
    assert_file_result_count(results, "src/use_error_boundary_clean.rs", 1);
    assert_has_result_id(results, "src/use_error_boundary_clean.rs", "RS-CODE-SOURCE-11");
    assert_file_result_count(results, "src/many_uses.rs", 1);
    assert_has_result_id(results, "src/many_uses.rs", "RS-CODE-SOURCE-11");
    assert_no_results_for_file(results, "src/use_warn_boundary_clean.rs");
    assert_no_results_for_file(results, "tests/use_exempt.rs");
    assert_file_result_count(results, "src/large_struct.rs", 1);
    assert_has_result_id(results, "src/large_struct.rs", "RS-CODE-SOURCE-19");
    assert_file_result_count(results, "src/large_enum.rs", 1);
    assert_has_result_id(results, "src/large_enum.rs", "RS-CODE-SOURCE-19");
    assert_file_result_count(results, "src/path_reason.rs", 1);
    assert_has_result_id(results, "src/path_reason.rs", "RS-CODE-SOURCE-24");
    assert_eq!(by_file["src/path_reason.rs"][0].title(), "#[path] with reason", "{results:#?}");
    assert_file_result_count(results, "src/path_missing_reason.rs", 1);
    assert_has_result_id(results, "src/path_missing_reason.rs", "RS-CODE-SOURCE-24");
    assert_eq!(by_file["src/path_missing_reason.rs"][0].title(), "#[path] without reason", "{results:#?}");
    assert_file_result_count(results, "src/path_weak_reason.rs", 1);
    assert_has_result_id(results, "src/path_weak_reason.rs", "RS-CODE-SOURCE-24");
    assert_eq!(by_file["src/path_weak_reason.rs"][0].title(), "#[path] reason too weak", "{results:#?}");
    assert_file_result_count(results, "src/path_escape.rs", 1);
    assert_has_result_id(results, "src/path_escape.rs", "RS-CODE-SOURCE-24");
    assert_eq!(by_file["src/path_escape.rs"][0].title(), "#[path] escapes parent directory", "{results:#?}");
    assert_file_result_count(results, "src/path_cfg_attr_reason.rs", 1);
    assert_has_result_id(results, "src/path_cfg_attr_reason.rs", "RS-CODE-SOURCE-24");
    assert_no_results_for_file(results, "src/path_cfg_attr_known_false.rs");
    assert_no_results_for_file(results, "src/path_sidecar_exempt.rs");
    assert_file_result_count(results, "src/cfg_attr_unknown.rs", 1);
    assert_has_result_id(results, "src/cfg_attr_unknown.rs", "RS-CODE-SOURCE-08");
    assert_file_result_count(results, "src/deny_without_reason.rs", 1);
    assert_has_result_id(results, "src/deny_without_reason.rs", "RS-CODE-SOURCE-22");
    assert_file_result_count(results, "src/impl_allow.rs", 2);
    assert_has_result_id(results, "src/impl_allow.rs", "RS-CODE-SOURCE-03");
    assert_has_result_id(results, "src/impl_allow.rs", "RS-CODE-SOURCE-17");
    assert_file_result_count(results, "src/cfg_attr.rs", 1);
    assert_has_result_id(results, "src/cfg_attr.rs", "RS-CODE-SOURCE-18");
    assert_file_result_count(results, "src/ffi.rs", 1);
    assert_has_result_id(results, "src/ffi.rs", "RS-CODE-SOURCE-20");
    assert_file_result_count(results, "src/fs_glob.rs", 2);
    assert_has_result_id(results, "src/fs_glob.rs", "RS-CODE-SOURCE-15");
    assert_has_result_id(results, "src/fs_glob.rs", "RS-CODE-SOURCE-21");
    assert_file_result_count(results, "src/include_probe.rs", 1);
    assert_has_result_id(results, "src/include_probe.rs", "RS-CODE-SOURCE-23");
    assert_file_result_count(results, "src/forbid_inventory.rs", 1);
    assert_has_result_id(results, "src/forbid_inventory.rs", "RS-CODE-SOURCE-22");
    assert!(by_file["src/forbid_inventory.rs"][0].inventory(), "{results:#?}");
    assert_file_result_count(results, "tests/expect_probe.rs", 1);
    assert_has_result_id(results, "tests/expect_probe.rs", "RS-CODE-SOURCE-32");
    assert_file_result_count(results, "src/generic_probe.rs", 1);
    assert_has_result_id(results, "src/generic_probe.rs", "RS-CODE-SOURCE-34");
    assert_file_result_count(results, "src/large_trait.rs", 1);
    assert_has_result_id_with_severity(results, "src/large_trait.rs", "RS-CODE-SOURCE-29", G3Severity::Warn);
    assert_file_result_count(results, "src/large_trait_boundary.rs", 1);
    assert_has_result_id_with_severity(results, "src/large_trait_boundary.rs", "RS-CODE-SOURCE-29", G3Severity::Warn);
    assert_no_results_for_file(results, "src/small_trait.rs");
    assert_file_result_count(results, "src/public_field_bag.rs", 1);
    assert_has_result_id_with_severity(results, "src/public_field_bag.rs", "RS-CODE-SOURCE-31", G3Severity::Warn);
    assert_file_result_count(results, "src/public_field_warn_boundary.rs", 1);
    assert_has_result_id_with_severity(results, "src/public_field_warn_boundary.rs", "RS-CODE-SOURCE-31", G3Severity::Warn);
    assert_file_result_count(results, "src/public_field_error_boundary.rs", 1);
    assert_has_result_id_with_severity(results, "src/public_field_error_boundary.rs", "RS-CODE-SOURCE-31", G3Severity::Error);
    assert_no_results_for_file(results, "src/private_field_struct.rs");
    assert_file_result_count(results, "src/public_weak_error.rs", 1);
    assert_has_result_id(results, "src/public_weak_error.rs", "RS-CODE-SOURCE-33");
    assert_file_result_count(results, "src/public_trait_weak_error.rs", 1);
    assert_has_result_id(results, "src/public_trait_weak_error.rs", "RS-CODE-SOURCE-33");
    assert_file_result_count(results, "src/public_impl_weak_error.rs", 1);
    assert_has_result_id(results, "src/public_impl_weak_error.rs", "RS-CODE-SOURCE-33");
    assert_file_result_count(results, "src/public_str_ref_error.rs", 1);
    assert_has_result_id(results, "src/public_str_ref_error.rs", "RS-CODE-SOURCE-33");
    assert_no_results_for_file(results, "src/typed_public_error.rs");
    assert_no_results_for_file(results, "src/private_weak_error.rs");
    assert_file_result_count(results, "src/string_dispatch.rs", 1);
    assert_has_result_id(results, "src/string_dispatch.rs", "RS-CODE-SOURCE-36");
}

pub fn assert_pipeline_reports_effective_line_and_dispatch_boundaries(
    results: &[G3CheckResult],
) {
    assert_no_results_for_file(results, "src/line_cap.rs");
    assert_file_result_count(results, "src/line_over_cap.rs", 1);
    assert_has_result_id(results, "src/line_over_cap.rs", "RS-CODE-SOURCE-09");
    assert_no_results_for_file(results, "src/string_dispatch_clean.rs");
}

pub fn assert_pipeline_reports_trait_and_public_error_boundaries(results: &[G3CheckResult]) {
    assert_no_results_for_file(results, "src/trait_clean.rs");
    assert_file_result_count(results, "src/trait_warn.rs", 1);
    assert_has_result_id(results, "src/trait_warn.rs", "RS-CODE-SOURCE-29");
    assert_file_result_count(results, "src/trait_error.rs", 1);
    assert_has_result_id(results, "src/trait_error.rs", "RS-CODE-SOURCE-29");
    assert_file_result_count(results, "src/public_string_error.rs", 1);
    assert_has_result_id(results, "src/public_string_error.rs", "RS-CODE-SOURCE-33");
    assert_file_result_count(results, "src/public_str_error.rs", 1);
    assert_has_result_id(results, "src/public_str_error.rs", "RS-CODE-SOURCE-33");
    assert_file_result_count(results, "src/public_anyhow_error.rs", 1);
    assert_has_result_id(results, "src/public_anyhow_error.rs", "RS-CODE-SOURCE-33");
    assert_file_result_count(results, "src/public_box_error.rs", 1);
    assert_has_result_id(results, "src/public_box_error.rs", "RS-CODE-SOURCE-33");
    assert_no_results_for_file(results, "src/private_string_error.rs");
    assert_file_result_count(results, "src/public_trait_error.rs", 1);
    assert_has_result_id(results, "src/public_trait_error.rs", "RS-CODE-SOURCE-33");
    assert_file_result_count(results, "src/public_impl_error.rs", 1);
    assert_has_result_id(results, "src/public_impl_error.rs", "RS-CODE-SOURCE-33");
}

pub fn assert_pipeline_reports_include_str_traversal(results: &[G3CheckResult]) {
    let by_file = findings_by_file(results);
    assert_file_result_count(results, "src/include_str_escape.rs", 1);
    assert_has_result_id(results, "src/include_str_escape.rs", "RS-CODE-SOURCE-23");
    assert_eq!(by_file["src/include_str_escape.rs"][0].title(), "include path traversal", "{results:#?}");
}

pub fn assert_pipeline_preserves_current_test_owned_rule_behavior(results: &[G3CheckResult]) {
    assert_result_count(results, 2);
    assert!(
        results.iter().all(|result| result.id() == "RS-CODE-SOURCE-13"),
        "{results:#?}"
    );
}

pub fn assert_pipeline_emits_explicit_input_failure_for_parse_error(results: &[G3CheckResult]) {
    assert_result_count(results, 1);
    let result = &results[0];
    assert_eq!(result.id(), "RS-CODE-SOURCE-30");
    assert_eq!(result.title(), "code-family input failure");
    assert_eq!(result.file(), Some("src/broken.rs"));
    assert!(
        result.message().starts_with("Failed to parse Rust source file:"),
        "unexpected message: {result:#?}"
    );
}

pub fn assert_pipeline_keeps_other_findings_when_one_file_fails_to_parse(
    results: &[G3CheckResult],
) {
    assert_has_result_id(results, "src/broken.rs", "RS-CODE-SOURCE-30");
    assert_has_result_id(results, "src/has_todo.rs", "RS-CODE-SOURCE-13");
}

pub fn assert_pipeline_classifies_custom_target_paths_before_checks_run(
    results: &[G3CheckResult],
) {
    assert_file_result_count(results, "lib/api.rs", 1);
    assert_has_result_id_with_severity(results, "lib/api.rs", "RS-CODE-SOURCE-29", G3Severity::Warn);
    assert_file_result_count(results, "cmd/worker.rs", 1);
    assert_has_result_id_with_severity(results, "cmd/worker.rs", "RS-CODE-SOURCE-29", G3Severity::Error);
}
