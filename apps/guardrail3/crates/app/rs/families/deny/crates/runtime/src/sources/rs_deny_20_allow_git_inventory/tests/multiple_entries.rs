use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_config_15_allow_git_inventory as assertions;

use super::helpers::{build_fixture_deny_toml, set_allow_git_sources};

#[test]
fn warns_once_and_inventories_each_allow_git_entry() {
    let results = super::helpers::run_check(&set_allow_git_sources(
        &build_fixture_deny_toml("service"),
        &[
            "https://github.com/example/repo",
            "https://github.com/example/other",
        ],
    ));
    assert!(!results.is_empty());

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
                "`deny.toml` allows git source `https://github.com/example/other`.",
                "deny.toml",
                true,
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
