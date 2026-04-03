use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_23_skip_hygiene as assertions;

fn skip_toml(skip: &str) -> String {
    format!("[bans]\nskip = {skip}\n")
}

#[test]
fn errors_for_plain_string_skip_entries_without_reasons() {
    let results = super::helpers::run_check(&skip_toml(r#"["plain-crate"]"#));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "skip entry must use table form",
            "`deny.toml` has `[bans.skip]` string entry `plain-crate`; use table form with a `reason`.",
            "deny.toml",
            false,
        )],
    );
}
