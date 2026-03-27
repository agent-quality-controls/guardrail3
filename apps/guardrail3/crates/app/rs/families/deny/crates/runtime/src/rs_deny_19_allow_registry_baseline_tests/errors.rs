use guardrail3_app_rs_family_deny_assertions::rs_deny_19_allow_registry_baseline as assertions;

use super::super::build_fixture_deny_toml;

#[test]
fn errors_when_sources_section_is_missing_or_crates_io_is_not_allowed() {
    let mut results = Vec::new();
    results.extend(super::super::run_check(
        &build_fixture_deny_toml("service").replace("[sources]\n", "[removed]\n"),
    ));
    results.extend(super::super::run_check(
        &build_fixture_deny_toml("service").replace(
            "allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]",
            "allow-registry = []",
        ),
    ));
    results.extend(super::super::run_check(
        &build_fixture_deny_toml("service").replace(
            "allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]",
            "allow-registry = [\"https://github.com/rust-lang/crates.io-index\", \"https://example.com/index\"]",
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
                "crates.io registry not allowed",
                "`deny.toml` must include crates.io in `[sources].allow-registry`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "unexpected registry allowed",
                "`deny.toml` allows unexpected registries: https://example.com/index.",
                "deny.toml",
                false,
            ),
        ],
    );
}
