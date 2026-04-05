use g3_deny_content_checks_assertions::rs_deny_19_allow_registry_baseline as assertions;

use super::helpers::run_check;

#[test]
fn extra_registry_errors() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/", "https://example.com/registry"]
allow-git = []
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "unexpected registry allowed",
                "`deny.toml` allows unexpected registry `https://example.com/registry`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "allow-registry entry count differs from baseline",
                "`deny.toml` must contain exactly 1 `[sources].allow-registry` entry.",
                "deny.toml",
                false,
            ),
        ],
    );
}
