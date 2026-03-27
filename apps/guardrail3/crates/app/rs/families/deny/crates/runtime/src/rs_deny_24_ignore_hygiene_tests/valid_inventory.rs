use guardrail3_app_rs_family_deny_assertions::rs_deny_24_ignore_hygiene as assertions;



fn ignore_toml(ignore: &str) -> String {
    format!("[advisories]\nignore = {ignore}\n")
}

#[test]
fn inventories_supported_ignore_entry_shapes() {
    let results = super::super::run_check(&ignore_toml(
        r#"["RUSTSEC-2026-0000", { id = "RUSTSEC-2026-0001", reason = "good enough reason text" }]"#,
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::info(
                "advisory ignore entry",
                "`deny.toml` ignores advisory `RUSTSEC-2026-0000`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "advisory ignore entry",
                "`deny.toml` ignores advisory `RUSTSEC-2026-0001`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
