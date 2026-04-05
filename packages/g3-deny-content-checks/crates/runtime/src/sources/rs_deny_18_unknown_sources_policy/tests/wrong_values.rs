use g3_deny_content_checks_assertions::rs_deny_18_unknown_sources_policy as assertions;

use super::helpers::run_check;

#[test]
fn wrong_unknown_registry_errors() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "warn"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "sources `unknown-registry` has wrong value",
            "`deny.toml` must set `[sources].unknown-registry = \"deny\"`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn wrong_unknown_git_errors() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "allow"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "sources `unknown-git` has wrong value",
            "`deny.toml` must set `[sources].unknown-git = \"deny\"`.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn both_wrong_errors_twice() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "allow"
unknown-git = "warn"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "sources `unknown-registry` has wrong value",
                "`deny.toml` must set `[sources].unknown-registry = \"deny\"`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "sources `unknown-git` has wrong value",
                "`deny.toml` must set `[sources].unknown-git = \"deny\"`.",
                "deny.toml",
                false,
            ),
        ],
    );
}

#[test]
fn missing_values_errors() {
    let results = run_check(
        r#"
[sources]
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "sources `unknown-registry` has wrong value",
                "`deny.toml` must set `[sources].unknown-registry = \"deny\"`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "sources `unknown-git` has wrong value",
                "`deny.toml` must set `[sources].unknown-git = \"deny\"`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
