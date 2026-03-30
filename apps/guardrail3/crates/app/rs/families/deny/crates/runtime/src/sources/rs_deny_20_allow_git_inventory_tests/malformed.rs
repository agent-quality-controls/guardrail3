use guardrail3_app_rs_family_deny_assertions::rs_deny_20_allow_git_inventory as assertions;

use super::super::{build_fixture_deny_toml, set_allow_git_sources};

#[test]
fn errors_for_blank_allow_git_entries_without_hiding_valid_entries() {
    let results = super::super::run_check(&set_allow_git_sources(
        &build_fixture_deny_toml("service"),
        &["", "https://github.com/example/repo"],
    ));

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "allow-git is non-empty",
                "`deny.toml` has non-empty `[sources].allow-git`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "allow-git entry must be non-empty",
                "`deny.toml` has blank `[sources].allow-git` entry.",
                "deny.toml",
                false,
            ),
            assertions::info(
                "allow-git entry",
                "`deny.toml` allows git source `https://github.com/example/repo`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
