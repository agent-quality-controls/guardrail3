use g3rs_deny_config_checks_assertions::sources::allow_registry_baseline::rule as assertions;

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
