use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_05_ancestor_shadow_drift::{
    ExpectedRuleResult, Severity, assert_rule_results,
};

use super::{AncestorFixture, check, test_input_with_ancestor};

const LOCAL_TOOLCHAIN_REL: &str = "apps/guardrail3/rust-toolchain.toml";
const STABLE_TOOLCHAIN: &str =
    "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]";

#[test]
fn emits_no_result_when_no_ancestor_toolchain_exists() {
    let input = test_input_with_ancestor(LOCAL_TOOLCHAIN_REL, STABLE_TOOLCHAIN, None);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(&results, &[]);
}

#[test]
fn emits_no_result_when_ancestor_toolchain_matches_local_policy() {
    let input = test_input_with_ancestor(
        LOCAL_TOOLCHAIN_REL,
        STABLE_TOOLCHAIN,
        Some(AncestorFixture::modern(
            "rust-toolchain.toml",
            STABLE_TOOLCHAIN,
        )),
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(&results, &[]);
}

#[test]
fn warns_when_ancestor_toolchain_differs_from_local_policy() {
    let input = test_input_with_ancestor(
        LOCAL_TOOLCHAIN_REL,
        STABLE_TOOLCHAIN,
        Some(AncestorFixture::modern(
            "rust-toolchain.toml",
            "[toolchain]\nchannel = \"beta\"\ncomponents = [\"clippy\", \"rustfmt\"]",
        )),
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Warn,
            inventory: false,
            title: "ancestor toolchain drifts from local policy root",
            message: "Ancestor `rust-toolchain.toml` differs from local toolchain policy at `apps/guardrail3/rust-toolchain.toml`. Running from the ancestor directory can use a different toolchain contract.",
            file: Some("rust-toolchain.toml"),
        }],
    );
}

#[test]
fn warns_when_ancestor_legacy_toolchain_can_shadow_local_policy() {
    let input = test_input_with_ancestor(
        LOCAL_TOOLCHAIN_REL,
        STABLE_TOOLCHAIN,
        Some(AncestorFixture::legacy("rust-toolchain")),
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Warn,
            inventory: false,
            title: "ancestor legacy toolchain shadows local policy root",
            message: "Ancestor `rust-toolchain` can shadow local toolchain policy at `apps/guardrail3/rust-toolchain.toml`. Remove or migrate the ancestor file so routed-root toolchain behavior stays stable.",
            file: Some("rust-toolchain"),
        }],
    );
}

#[test]
fn warns_when_ancestor_toolchain_is_malformed() {
    let input = test_input_with_ancestor(
        LOCAL_TOOLCHAIN_REL,
        STABLE_TOOLCHAIN,
        Some(AncestorFixture::malformed(
            "rust-toolchain.toml",
            "expected a right bracket",
        )),
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Warn,
            inventory: false,
            title: "ancestor toolchain parse error risks shadow drift",
            message: "Ancestor `rust-toolchain.toml` at `rust-toolchain.toml` is invalid: expected a right bracket. Commands run above `apps/guardrail3/rust-toolchain.toml` may resolve a different toolchain surface.",
            file: Some("rust-toolchain.toml"),
        }],
    );
}
