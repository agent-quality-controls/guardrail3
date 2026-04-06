use g3rs_deny_config_checks_assertions::rs_deny_config_15_allow_git_inventory as assertions;

use super::helpers::run_check;

#[test]
fn blank_entry_warns() {
    let results = run_check(
        r#"
[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = [""]
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
            assertions::warn(
                "allow-git entry must be non-empty",
                "`deny.toml` has blank `[sources].allow-git` entry.",
                "deny.toml",
                false,
            ),
        ],
    );
}
