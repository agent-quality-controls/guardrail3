use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_24_ignore_hygiene as assertions;

fn ignore_toml(ignore: &str) -> String {
    format!("[advisories]\nignore = {ignore}\n")
}

#[test]
fn warns_for_documented_ignore_entries() {
    let results = super::super::run_check(&ignore_toml(
        r#"[{ id = "RUSTSEC-2026-0001", reason = "good enough reason text" }]"#,
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "advisory ignore entry",
                "`deny.toml` has documented advisory ignore `RUSTSEC-2026-0001`.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "advisory ignore count",
                "`deny.toml` has 1 advisory ignores (1 documented, 0 missing or invalid reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
