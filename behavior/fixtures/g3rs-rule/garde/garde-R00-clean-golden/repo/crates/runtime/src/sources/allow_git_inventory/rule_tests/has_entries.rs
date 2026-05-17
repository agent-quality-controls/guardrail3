use g3rs_deny_config_checks_assertions::sources::allow_git_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn non_empty_allow_git_warns_and_inventories() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = ["https://github.com/foo/bar"]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "allow-git is non-empty",
                "`deny.toml` has non-empty `[sources].allow-git`.",
                "deny.toml",
                false,
            ),
            assertions::info(
                "allow-git entry",
                "`deny.toml` allows git source `https://github.com/foo/bar`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

#[test]
fn multiple_git_entries_warns_and_inventories_each() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = ["https://github.com/foo/bar", "https://github.com/baz/qux"]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "allow-git is non-empty",
                "`deny.toml` has non-empty `[sources].allow-git`.",
                "deny.toml",
                false,
            ),
            assertions::info(
                "allow-git entry",
                "`deny.toml` allows git source `https://github.com/foo/bar`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "allow-git entry",
                "`deny.toml` allows git source `https://github.com/baz/qux`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
