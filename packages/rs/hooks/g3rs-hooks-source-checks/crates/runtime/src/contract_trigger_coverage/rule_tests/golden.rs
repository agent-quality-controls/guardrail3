use g3rs_hooks_contract_types::{G3HookRequirement, G3HookTriggerPattern};
use g3rs_hooks_source_checks_assertions::contract_trigger_coverage::rule as assertions;

use super::super::run_case;

#[test]
fn fires_missing_pattern_error_when_hook_lacks_rust_relevant_pattern() {
    let results = run_case(
        "#!/bin/sh\nif echo \"$STAGED_FILES\" | grep -qE '(Cargo\\.toml)$'; then g3rs validate --path .; fi\n",
        vec![requirement("guardrail3-rs.toml")],
    );

    assertions::assert_missing_pattern_error(&results);
}

#[test]
fn fires_missing_coverage_when_pattern_omits_an_exact_trigger_path() {
    let results = run_case(
        "#!/bin/sh\nRUST_RELEVANT_PATTERN='((^|/)Cargo\\.toml$)'\n",
        vec![requirement("guardrail3-rs.toml")],
    );

    assertions::assert_missing_coverage_error(&results);
}

#[test]
fn passes_inventory_when_pattern_covers_every_exact_trigger_path() {
    let results = run_case(
        "#!/bin/sh\nRUST_RELEVANT_PATTERN='((^|/)guardrail3-rs\\.toml$|(^|/)Cargo\\.toml$)'\n",
        vec![requirement("guardrail3-rs.toml"), requirement("Cargo.toml")],
    );

    assertions::assert_coverage_proven_inventory(&results);
}

fn requirement(path: &str) -> G3HookRequirement {
    G3HookRequirement {
        id: "test/hook-contract".to_owned(),
        owner_family: "test".to_owned(),
        trigger_patterns: vec![G3HookTriggerPattern::ExactPath(path.to_owned())],
        required_commands: Vec::new(),
        critical_commands: Vec::new(),
    }
}
