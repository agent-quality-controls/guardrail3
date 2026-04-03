use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_19_allow_registry_baseline as assertions;

use super::helpers::{build_fixture_deny_toml, set_allow_registries};

#[test]
fn local_registry_drift_only_errors_for_the_owned_local_root() {
    let results = super::helpers::run_check(&set_allow_registries(
        &build_fixture_deny_toml("service"),
        &["https://github.com/rust-lang/crates.io-index"],
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "canonical crates.io registry not allowed",
                "`deny.toml` must allow only `sparse+https://index.crates.io/` in `[sources].allow-registry`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "unexpected registry allowed",
                "`deny.toml` allows unexpected registries: https://github.com/rust-lang/crates.io-index.",
                "deny.toml",
                false,
            ),
        ],
    );
}
