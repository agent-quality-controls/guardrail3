use guardrail3_app_rs_family_deny_assertions::rs_deny_09_ban_baseline_complete as assertions;

use super::super::{build_fixture_deny_toml, remove_deny_ban};

#[test]
fn errors_for_each_missing_canonical_service_ban() {
    let results = super::super::run_check(&remove_deny_ban(
        &remove_deny_ban(&build_fixture_deny_toml("service"), "actix-web"),
        "lazy_static",
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "missing canonical ban",
                "`deny.toml` is missing deny ban `actix-web`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "missing canonical ban",
                "`deny.toml` is missing deny ban `lazy_static`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
