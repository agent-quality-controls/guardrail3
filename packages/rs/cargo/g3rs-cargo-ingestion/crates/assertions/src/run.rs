use guardrail3_check_types::G3CheckResult;

fn has_result(results: &[G3CheckResult], id: &str, title: &str, file: Option<&str>) -> bool {
    results
        .iter()
        .any(|result| result.id() == id && result.title() == title && result.file() == file)
}

pub fn assert_config_pipeline_old_app_allow_inventory_and_member_rules(results: &[G3CheckResult]) {
    assert!(
        has_result(
            results,
            "g3rs-cargo/approved-allow-inventory",
            "approved allow entry",
            Some("Cargo.toml"),
        ),
        "{results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/no-weakened-overrides",
            "weakened member rust override",
            Some("crates/api/Cargo.toml"),
        ),
        "{results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/member-edition-drift",
            "member edition older than workspace",
            Some("crates/api/Cargo.toml"),
        ),
        "{results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/member-local-allows-forbidden",
            "member-local allow entry missing reason",
            Some("crates/api/Cargo.toml"),
        ),
        "{results:#?}"
    );
}

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
        "{results:#?}"
    );
}

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
        "{results:#?}"
    );
    assert!(
        !results.iter().any(|result| {
            result.id() == "g3rs-cargo/input-failures" && result.file() == Some("guardrail3.toml")
        }),
        "{results:#?}"
    );
}

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
        "{results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/input-failures",
            "failed to read Cargo configuration",
            Some("crates/api/Cargo.toml"),
        ),
        "{results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/input-failures",
            "failed to read Cargo configuration",
            Some("guardrail3-rs.toml"),
        ),
        "{results:#?}"
    );
}

pub fn assert_filetree_pipeline_returns_exact_clean_inventory_results(results: &[G3CheckResult]) {
    assert_eq!(results.len(), 2, "{results:#?}");
    assert!(
        has_result(
            results,
            "g3rs-cargo/missing-member-cargo",
            "all declared workspace members have Cargo.toml",
            Some("Cargo.toml"),
        ),
        "{results:#?}"
    );
    assert!(
        has_result(
            results,
            "g3rs-cargo/input-failures",
            "cargo-family inputs parsed cleanly",
            Some("Cargo.toml"),
        ),
        "{results:#?}"
    );
    assert!(
        results.iter().all(|result| result.inventory()),
        "{results:#?}"
    );
}

pub fn assert_filetree_pipeline_reports_malformed_workspace_members_without_missing_member_reclassification(
    results: &[G3CheckResult],
) {
    assert_eq!(results.len(), 2, "{results:#?}");
    assert!(
        results
            .iter()
            .all(|result| result.id() == "g3rs-cargo/input-failures")
    );
    assert!(
        results
            .iter()
            .all(|result| result.file() == Some("Cargo.toml"))
    );
    assert!(
        results
            .iter()
            .all(|result| result.title() == "failed to read Cargo configuration")
    );
}

pub fn assert_rust_policy_parse_error(state: &g3rs_cargo_types::G3RsCargoRustPolicyState) {
    if let g3rs_cargo_types::G3RsCargoRustPolicyState::ParseError { rel_path, reason } = state {
        assert_eq!(rel_path, "guardrail3-rs.toml");
        assert!(!reason.is_empty());
    } else {
        assert!(false, "expected rust policy parse error, got {state:#?}");
    }
}

pub fn assert_rust_policy_unreadable(state: &g3rs_cargo_types::G3RsCargoRustPolicyState) {
    if let g3rs_cargo_types::G3RsCargoRustPolicyState::Unreadable { rel_path, reason } = state {
        assert_eq!(rel_path, "guardrail3-rs.toml");
        assert_eq!(reason, "file is not readable");
    } else {
        assert!(false, "expected rust policy unreadable, got {state:#?}");
    }
}

pub fn assert_unreadable_error(
    err: &g3rs_cargo_ingestion_runtime::IngestionError,
    expected_suffix: &str,
) {
    if let g3rs_cargo_ingestion_runtime::IngestionError::Unreadable { path, reason } = err {
        assert!(path.ends_with(expected_suffix), "{path:?}");
        assert!(!reason.is_empty());
    } else {
        assert!(false, "expected unreadable error, got {err:#?}");
    }
}

pub fn assert_parse_failed_error(
    err: &g3rs_cargo_ingestion_runtime::IngestionError,
    expected_suffix: &str,
) {
    if let g3rs_cargo_ingestion_runtime::IngestionError::ParseFailed { path, reason } = err {
        assert!(path.ends_with(expected_suffix), "{path:?}");
        assert!(!reason.is_empty());
    } else {
        assert!(false, "expected parse failure, got {err:#?}");
    }
}
