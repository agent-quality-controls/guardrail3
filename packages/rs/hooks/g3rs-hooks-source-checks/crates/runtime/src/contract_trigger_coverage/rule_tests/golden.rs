use g3rs_hooks_contract_types::{G3HookRequirement, G3HookTriggerPattern};

use crate::contract_trigger_coverage::rule::run_case;

#[test]
fn reports_missing_exact_trigger_path() {
    let results = run_case(
        "#!/bin/sh\nif echo \"$STAGED_FILES\" | grep -qE '(Cargo\\.toml)$'; then g3rs validate --path .; fi\n",
        vec![requirement("guardrail3-rs.toml")],
    );

    assert!(
        results.iter().any(|result| !result.inventory()),
        "missing exact contract trigger path should be reported"
    );
}

#[test]
fn warns_even_when_exact_trigger_path_is_present_until_shell_conditions_are_modeled() {
    let results = run_case(
        "#!/bin/sh\nif echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml)$'; then g3rs validate --path .; fi\n",
        vec![requirement("guardrail3-rs.toml")],
    );

    assert!(
        results
            .iter()
            .any(|result| !result.inventory() && result.title().contains("not proven")),
        "exact trigger path coverage should not emit success until parsed shell conditions are modeled"
    );
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
