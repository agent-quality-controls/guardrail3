use g3rs_hooks_contract_types::{G3HookRequirement, G3HookTriggerPattern};
use g3rs_hooks_source_checks_assertions::contract_trigger_coverage::rule as assertions;

use super::super::run_case;

#[test]
fn reports_missing_exact_trigger_path() {
    let results = run_case(
        "#!/bin/sh\nif echo \"$STAGED_FILES\" | grep -qE '(Cargo\\.toml)$'; then g3rs validate --path .; fi\n",
        vec![requirement("guardrail3-rs.toml")],
    );

    assertions::assert_not_proven_warning(&results);
}

#[test]
fn warns_even_when_exact_trigger_path_is_present_until_shell_conditions_are_modeled() {
    let results = run_case(
        "#!/bin/sh\nif echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml)$'; then g3rs validate --path .; fi\n",
        vec![requirement("guardrail3-rs.toml")],
    );

    assertions::assert_not_proven_warning(&results);
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
