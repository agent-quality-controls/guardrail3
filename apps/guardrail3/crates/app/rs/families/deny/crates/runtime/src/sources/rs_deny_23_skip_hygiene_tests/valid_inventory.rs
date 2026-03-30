use guardrail3_app_rs_family_deny_assertions::rs_deny_23_skip_hygiene as assertions;

fn skip_toml(skip: &str) -> String {
    format!("[bans]\nskip = {skip}\n")
}

#[test]
fn inventories_supported_skip_entry_shapes() {
    let results = super::super::run_check(&skip_toml(
        r#"["plain-crate", { crate = "serde@1.0.0", reason = "good enough reason text" }, { name = "windows-sys", version = "0.60.2", reason = "good enough reason text" }]"#,
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::info(
                "skip entry",
                "`deny.toml` has skip entry `plain-crate`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "skip entry",
                "`deny.toml` has skip entry `serde`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "skip entry",
                "`deny.toml` has skip entry `windows-sys`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
