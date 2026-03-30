use guardrail3_app_rs_family_deny_assertions::rs_deny_23_skip_hygiene as assertions;

fn skip_toml(skip: &str) -> String {
    format!("[bans]\nskip = {skip}\n")
}

#[test]
fn errors_for_malformed_missing_reason_and_non_string_reason_entries() {
    let results = super::super::run_check(&skip_toml(
        r#"[{ reason = "good enough reason text" }, { crate = "serde@1.0.0" }, { crate = "regex@1.0.0", reason = 7 }]"#,
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "malformed skip entry",
                "`deny.toml` has `[bans.skip]` entry without a valid crate identifier.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "skip entry missing reason",
                "`deny.toml` skips `serde` without a `reason`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "skip reason must be a string",
                "`deny.toml` has `[bans.skip]` entry `regex` with a non-string `reason`.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "skip entry count",
                "`deny.toml` has 2 skip entries (0 documented, 2 missing or invalid reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
