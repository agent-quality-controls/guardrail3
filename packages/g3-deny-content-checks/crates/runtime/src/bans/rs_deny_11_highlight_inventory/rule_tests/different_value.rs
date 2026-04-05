use g3_deny_content_checks_assertions::rs_deny_11_highlight_inventory as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_highlight_differs_from_baseline() {
    let results = run_check(
        r#"
[bans]
highlight = "simplest"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "highlight differs from baseline",
            "`deny.toml` sets `[bans].highlight = simplest`.",
            "deny.toml",
            true,
        )],
    );
}
