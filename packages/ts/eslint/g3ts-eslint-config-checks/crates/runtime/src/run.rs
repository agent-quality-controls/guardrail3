use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

#[must_use]
pub fn check(input: &G3TsEslintConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    crate::exists::check(input, &mut results);
    crate::parseable::check(input, &mut results);
    crate::ts_plugin_present::check(input, &mut results);
    crate::project_service_enabled::check(input, &mut results);
    crate::no_explicit_any_error::check(input, &mut results);
    crate::no_console_error::check(input, &mut results);
    crate::thresholds::check(input, &mut results);
    crate::core_baseline_rules::check(input, &mut results);
    crate::type_safety_rules::check(input, &mut results);
    crate::hygiene_rules::check(input, &mut results);
    crate::unicorn_rules::check(input, &mut results);
    crate::regexp_rules::check(input, &mut results);
    crate::sonarjs_rules::check(input, &mut results);
    crate::test_relaxations::check(input, &mut results);
    crate::js_carveout::check(input, &mut results);
    crate::plugin_stack::check(input, &mut results);
    crate::tsx_source_parity::check(input, &mut results);
    results
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod run_tests;
