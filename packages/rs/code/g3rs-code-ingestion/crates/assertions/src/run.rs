#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

pub use crate::run_config::{
    assert_config_file_paths, assert_config_pipeline_ignores_foreign_nested_repo_findings,
    assert_config_pipeline_reports_deny_through_full_lane,
    assert_config_pipeline_reports_exact_exception_comment_counts,
    assert_config_pipeline_reports_exception_comments_and_unsafe_code_lints,
    assert_config_pipeline_stays_clean_for_harmless_comments_and_non_workspace_manifests,
    assert_exception_comments, assert_parser_backed_config_files_only,
    assert_workspace_unsafe_code_detailed_level, assert_workspace_unsafe_code_level,
    require_config_file,
};
pub use crate::run_file_tree::{assert_root_cargo_paths, assert_single_zero_structural_root};
pub use crate::run_pipeline::{
    assert_pipeline_classifies_custom_target_paths_before_checks_run,
    assert_pipeline_emits_explicit_input_failure_for_parse_error,
    assert_pipeline_keeps_other_findings_when_one_file_fails_to_parse,
    assert_pipeline_preserves_current_test_owned_rule_behavior,
    assert_pipeline_reports_effective_line_and_dispatch_boundaries,
    assert_pipeline_reports_expected_findings_on_real_source_files,
    assert_pipeline_reports_include_str_traversal,
    assert_pipeline_reports_new_single_file_ast_rules,
    assert_pipeline_reports_trait_and_public_error_boundaries,
};
pub use crate::run_results::{
    assert_code_ast_results, assert_file_result_count, assert_has_result_id,
    assert_has_result_id_with_severity, assert_no_results_for_file, assert_result_count,
    assert_results_empty, assert_single_parse_failed_error, assert_single_unreadable_error,
};
pub use crate::run_source::{
    assert_not_shared_crate, assert_shared_crate, assert_source_file, assert_source_parse_failure,
    assert_source_waiver, require_source_file,
};
