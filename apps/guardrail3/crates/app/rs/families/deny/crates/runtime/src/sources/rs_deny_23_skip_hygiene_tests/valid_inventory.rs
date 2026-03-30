use guardrail3_app_rs_family_deny_assertions::rs_deny_23_skip_hygiene as assertions;

fn skip_toml(skip: &str) -> String {
    format!("[bans]\nskip = {skip}\n")
}

#[test]
fn warns_for_documented_skip_entries() {
    let results = super::super::run_check(&skip_toml(
        r#"[{ crate = "serde@1.0.0", reason = "good enough reason text" }, { name = "windows-sys", version = "0.60.2", reason = "good enough reason text" }]"#,
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "skip entry",
                "`deny.toml` has documented skip entry `serde`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "skip entry",
                "`deny.toml` has documented skip entry `windows-sys`.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "skip entry count",
                "`deny.toml` has 2 skip entries (2 documented, 0 missing or invalid reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
