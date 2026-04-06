use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_config_13_unknown_sources_policy as assertions;

use super::helpers::{build_fixture_deny_toml, set_source_policy};

#[test]
fn errors_for_each_weakened_unknown_source_policy_key() {
    let deny = set_source_policy(
        &set_source_policy(
            &build_fixture_deny_toml("service"),
            "unknown-registry",
            "allow",
        ),
        "unknown-git",
        "warn",
    );
    let results = super::helpers::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "sources `unknown-git` has wrong value",
                "`deny.toml` must set `[sources].unknown-git = \"deny\"`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "sources `unknown-registry` has wrong value",
                "`deny.toml` must set `[sources].unknown-registry = \"deny\"`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
