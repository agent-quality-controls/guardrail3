use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_29_ignore_accumulation as assertions;

use super::super::ConfigDenyInput;
use super::super::{build_fixture_deny_toml, config_facts};

#[test]
fn warns_when_ignore_count_exceeds_threshold() {
    let deny = config_facts(&build_fixture_deny_toml("service").replace(
        "ignore = []",
        "ignore = [\"A\", \"B\", \"C\", \"D\", \"E\", \"F\"]",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "advisory ignore list is large",
            "`deny.toml` has 6 `[advisories].ignore` entries (threshold: 5).",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn does_not_warn_at_or_below_threshold_even_with_mixed_entry_shapes() {
    let deny = config_facts(&build_fixture_deny_toml("service").replace(
        "ignore = []",
        "ignore = [\"A\", \"B\", \"C\", { id = \"D\", reason = \"good enough reason text\" }, { id = \"E\", reason = \"good enough reason text\" }]",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert!(results.is_empty());
}
