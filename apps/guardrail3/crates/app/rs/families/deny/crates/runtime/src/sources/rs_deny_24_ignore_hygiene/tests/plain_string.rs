use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_config_19_ignore_hygiene as assertions;

fn ignore_toml(ignore: &str) -> String {
    format!("[advisories]\nignore = {ignore}\n")
}

#[test]
fn errors_for_plain_string_ignore_entries_without_reasons() {
    let results = super::helpers::run_check(&ignore_toml(r#"["RUSTSEC-2026-0000"]"#));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "advisory ignore must use table form",
            "`deny.toml` has `[advisories].ignore` string entry `RUSTSEC-2026-0000`; use table form with a `reason`.",
            "deny.toml",
            false,
        )],
    );
}
