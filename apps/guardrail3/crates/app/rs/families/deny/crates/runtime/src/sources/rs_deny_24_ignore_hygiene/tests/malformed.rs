use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_24_ignore_hygiene as assertions;

fn ignore_toml(ignore: &str) -> String {
    format!("[advisories]\nignore = {ignore}\n")
}

#[test]
fn errors_for_malformed_missing_reason_and_non_string_reason_ignore_entries() {
    let results = super::super::run_check(&ignore_toml(
        r#"[{ reason = "good enough reason text" }, { id = "RUSTSEC-2026-0001" }, { id = "RUSTSEC-2026-0002", reason = 7 }]"#,
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "advisory ignore missing reason",
                "`deny.toml` ignores advisory `RUSTSEC-2026-0001` without a `reason`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "advisory ignore reason must be a string",
                "`deny.toml` has `[advisories].ignore` entry `RUSTSEC-2026-0002` with a non-string `reason`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "malformed advisory ignore entry",
                "`deny.toml` has an `[advisories].ignore` entry without a valid advisory id.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "advisory ignore count",
                "`deny.toml` has 2 advisory ignores (0 documented, 2 missing or invalid reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
