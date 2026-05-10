use guardrail3_check_types::G3CheckResult;

/// Return true when the result set contains a finding with the given identity tuple.
fn has_result(results: &[G3CheckResult], id: &str, title: &str, file: Option<&str>) -> bool {
    results
        .iter()
        .any(|result| result.id() == id && result.title() == title && result.file() == file)
}

/// Assert the cargo-family config pipeline emits the expected allow-inventory and member findings.
///
/// # Panics
///
/// Panics when any expected finding is missing.
pub fn assert_config_pipeline_old_app_allow_inventory_and_member_rules(results: &[G3CheckResult]) {
    assert!(
        has_result(
            results,
            "g3rs-cargo/approved-allow-inventory",
            "approved allow entry",
            Some("Cargo.toml"),
        ),
        "missing approved-allow-inventory finding: {results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/no-weakened-overrides",
            "weakened member rust override",
            Some("crates/api/Cargo.toml"),
        ),
        "missing no-weakened-overrides finding: {results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/member-edition-drift",
            "member edition older than workspace",
            Some("crates/api/Cargo.toml"),
        ),
        "missing member-edition-drift finding: {results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/member-local-allows-forbidden",
            "member-local allow entry missing reason",
            Some("crates/api/Cargo.toml"),
        ),
        "missing member-local-allows-forbidden finding: {results:#?}"
    );
}

/// Assert the config pipeline suppresses allow-related findings when guardrail3-rs.toml is invalid.
///
/// # Panics
///
/// Panics when any allow-related finding fires for the invalid-policy fixture.
pub fn assert_config_pipeline_stands_down_allow_rules_when_guardrail3_rs_is_invalid(
    results: &[G3CheckResult],
) {
    assert!(
        !results.iter().any(|result| {
            matches!(
                result.id(),
                "g3rs-cargo/approved-allow-inventory"
                    | "g3rs-cargo/unapproved-allow-entries"
                    | "g3rs-cargo/member-local-allows-forbidden"
            )
        }),
        "unexpected allow-related finding fired: {results:#?}"
    );
}

/// Assert the file-tree pipeline surfaces guardrail3-rs.toml parse failures and ignores legacy file.
///
/// # Panics
///
/// Panics when the expected input-failure is missing or the legacy file is referenced.
pub fn assert_filetree_pipeline_reports_guardrail3_rs_parse_failures_and_ignores_legacy_guardrail3_toml(
    results: &[G3CheckResult],
) {
    assert!(
        has_result(
            results,
            "g3rs-cargo/input-failures",
            "failed to read Cargo configuration",
            Some("guardrail3-rs.toml"),
        ),
        "missing guardrail3-rs.toml input-failure: {results:#?}"
    );
    assert!(
        !results.iter().any(|result| {
            result.id() == "g3rs-cargo/input-failures" && result.file() == Some("guardrail3.toml")
        }),
        "unexpected legacy guardrail3.toml input-failure: {results:#?}"
    );
}

/// Assert the file-tree pipeline emits missing-member and input-failure findings.
///
/// # Panics
///
/// Panics when any expected finding is missing.
pub fn assert_filetree_pipeline_reports_missing_member_and_input_failures(
    results: &[G3CheckResult],
) {
    assert!(
        has_result(
            results,
            "g3rs-cargo/missing-member-cargo",
            "declared workspace member missing Cargo.toml",
            Some("Cargo.toml"),
        ),
        "missing missing-member-cargo finding: {results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/input-failures",
            "failed to read Cargo configuration",
            Some("crates/api/Cargo.toml"),
        ),
        "missing member input-failure: {results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/input-failures",
            "failed to read Cargo configuration",
            Some("guardrail3-rs.toml"),
        ),
        "missing guardrail3-rs.toml input-failure: {results:#?}"
    );
}

/// Assert the file-tree pipeline emits exactly the clean-inventory result set.
///
/// # Panics
///
/// Panics when the result count is not 2 or any expected inventory is missing.
pub fn assert_filetree_pipeline_returns_exact_clean_inventory_results(results: &[G3CheckResult]) {
    assert_eq!(results.len(), 2, "{results:#?}");
    assert!(
        has_result(
            results,
            "g3rs-cargo/missing-member-cargo",
            "all declared workspace members have Cargo.toml",
            Some("Cargo.toml"),
        ),
        "missing clean missing-member-cargo inventory: {results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/input-failures",
            "cargo-family inputs parsed cleanly",
            Some("Cargo.toml"),
        ),
        "missing clean input-failures inventory: {results:#?}"
    );
    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "non-inventory result in clean output: {results:#?}"
    );
}

/// Assert the file-tree pipeline surfaces malformed-workspace-members errors as input-failures only.
///
/// # Panics
///
/// Panics when the result set does not exactly match the expected shape.
pub fn assert_filetree_pipeline_reports_malformed_workspace_members_without_missing_member_reclassification(
    results: &[G3CheckResult],
) {
    assert_eq!(results.len(), 2, "{results:#?}");
    assert!(
        results
            .iter()
            .all(|result| result.id() == "g3rs-cargo/input-failures"),
        "non-input-failure id present: {results:#?}"
    );
    assert!(
        results
            .iter()
            .all(|result| result.file() == Some("Cargo.toml")),
        "non-Cargo.toml file present: {results:#?}"
    );
    assert!(
        results
            .iter()
            .all(|result| result.title() == "failed to read Cargo configuration"),
        "unexpected title present: {results:#?}"
    );
}

/// Assert that the rust-policy state is `ParseError` with the expected `rel_path`.
///
/// # Panics
///
/// Panics when the variant is not `ParseError` or the `rel_path` mismatches.
pub fn assert_rust_policy_parse_error(state: &g3rs_cargo_types::G3RsCargoRustPolicyState) {
    let g3rs_cargo_types::G3RsCargoRustPolicyState::ParseError { rel_path, reason } = state else {
        assert!(
            matches!(
                state,
                g3rs_cargo_types::G3RsCargoRustPolicyState::ParseError { .. }
            ),
            "expected rust policy parse error, got {state:?}"
        );
        return;
    };
    assert_eq!(rel_path, "guardrail3-rs.toml", "rel_path mismatch");
    assert!(!reason.is_empty(), "reason should be non-empty");
}

/// Assert that the rust-policy state is `Unreadable` with the expected `rel_path`/`reason`.
///
/// # Panics
///
/// Panics when the variant is not `Unreadable` or fields mismatch.
pub fn assert_rust_policy_unreadable(state: &g3rs_cargo_types::G3RsCargoRustPolicyState) {
    let g3rs_cargo_types::G3RsCargoRustPolicyState::Unreadable { rel_path, reason } = state else {
        assert!(
            matches!(
                state,
                g3rs_cargo_types::G3RsCargoRustPolicyState::Unreadable { .. }
            ),
            "expected rust policy unreadable, got {state:?}"
        );
        return;
    };
    assert_eq!(rel_path, "guardrail3-rs.toml", "rel_path mismatch");
    assert_eq!(reason, "file is not readable", "reason mismatch");
}

/// Assert that the ingestion error is `Unreadable` and the path ends with `expected_suffix`.
///
/// # Panics
///
/// Panics when the variant is not `Unreadable` or the path suffix mismatches.
pub fn assert_unreadable_error(
    err: &g3rs_cargo_ingestion_runtime::IngestionError,
    expected_suffix: &str,
) {
    let g3rs_cargo_ingestion_runtime::IngestionError::Unreadable { path, reason } = err else {
        assert!(
            matches!(
                err,
                g3rs_cargo_ingestion_runtime::IngestionError::Unreadable { .. }
            ),
            "expected unreadable error, got {err:?}"
        );
        return;
    };
    assert!(
        path.ends_with(expected_suffix),
        "path suffix mismatch: {}",
        path.display()
    );
    assert!(!reason.is_empty(), "reason should be non-empty");
}

/// Assert that the ingestion error is `ParseFailed` and the path ends with `expected_suffix`.
///
/// # Panics
///
/// Panics when the variant is not `ParseFailed` or the path suffix mismatches.
pub fn assert_parse_failed_error(
    err: &g3rs_cargo_ingestion_runtime::IngestionError,
    expected_suffix: &str,
) {
    let g3rs_cargo_ingestion_runtime::IngestionError::ParseFailed { path, reason } = err else {
        assert!(
            matches!(
                err,
                g3rs_cargo_ingestion_runtime::IngestionError::ParseFailed { .. }
            ),
            "expected parse failure, got {err:?}"
        );
        return;
    };
    assert!(
        path.ends_with(expected_suffix),
        "path suffix mismatch: {}",
        path.display()
    );
    assert!(!reason.is_empty(), "reason should be non-empty");
}
