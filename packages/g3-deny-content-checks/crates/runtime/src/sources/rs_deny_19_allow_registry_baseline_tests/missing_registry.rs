use g3_deny_content_checks_assertions::rs_deny_19_allow_registry_baseline as assertions;

use super::helpers::run_check;

#[test]
fn missing_canonical_registry_errors() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://example.com/registry"]
allow-git = []
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "canonical registry missing",
                "`deny.toml` must allow registry `sparse+https://index.crates.io/`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "unexpected registry allowed",
                "`deny.toml` allows unexpected registry `https://example.com/registry`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
