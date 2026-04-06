use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_config_14_allow_registry_baseline as assertions;

use super::helpers::build_fixture_deny_toml;

#[test]
fn errors_when_sources_section_is_missing_or_registry_policy_is_not_exact() {
    let mut results = Vec::new();
    results.extend(super::helpers::run_check(
        &build_fixture_deny_toml("service").replace("[sources]\n", "[removed]\n"),
    ));
    results.extend(super::helpers::run_check(
        &build_fixture_deny_toml("service").replace(
            "allow-registry = [\"sparse+https://index.crates.io/\"]",
            "allow-registry = []",
        ),
    ));
    results.extend(super::helpers::run_check(
        &build_fixture_deny_toml("service").replace(
            "allow-registry = [\"sparse+https://index.crates.io/\"]",
            "allow-registry = [\"sparse+https://index.crates.io/\", \"https://example.com/index\"]",
        ),
    ));
    results.extend(super::helpers::run_check(
        &build_fixture_deny_toml("service").replace(
            "allow-registry = [\"sparse+https://index.crates.io/\"]",
            "allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]",
        ),
    ));
    results.extend(super::helpers::run_check(
        &build_fixture_deny_toml("service").replace(
            "allow-registry = [\"sparse+https://index.crates.io/\"]",
            "allow-registry = [\"sparse+https://index.crates.io/\", \"https://github.com/rust-lang/crates.io-index\"]",
        ),
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "[sources] allow-registry missing",
                "`deny.toml` has no valid crates.io registry allow-list.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "allow-registry must contain exactly one entry",
                "`deny.toml` must contain exactly one `[sources].allow-registry` entry: `sparse+https://index.crates.io/`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "allow-registry must contain exactly one entry",
                "`deny.toml` must contain exactly one `[sources].allow-registry` entry: `sparse+https://index.crates.io/`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "unexpected registry allowed",
                "`deny.toml` allows unexpected registries: https://example.com/index.",
                "deny.toml",
                false,
            ),
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
            assertions::error(
                "allow-registry must contain exactly one entry",
                "`deny.toml` must contain exactly one `[sources].allow-registry` entry: `sparse+https://index.crates.io/`.",
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
