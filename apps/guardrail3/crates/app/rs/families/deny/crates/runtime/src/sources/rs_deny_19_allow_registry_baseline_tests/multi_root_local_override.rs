use guardrail3_app_rs_family_deny_assertions::rs_deny_19_allow_registry_baseline as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, set_allow_registries, write_file};

#[test]
fn local_registry_drift_only_errors_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_allow_registries(
            &build_fixture_deny_toml("service"),
            &["https://github.com/rust-lang/crates.io-index"],
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "canonical crates.io registry not allowed",
                "`apps/devctl/deny.toml` must allow only `sparse+https://index.crates.io/` in `[sources].allow-registry`.",
                "apps/devctl/deny.toml",
                false,
            ),
            assertions::error(
                "unexpected registry allowed",
                "`apps/devctl/deny.toml` allows unexpected registries: https://github.com/rust-lang/crates.io-index.",
                "apps/devctl/deny.toml",
                false,
            ),
        ],
    );
}
