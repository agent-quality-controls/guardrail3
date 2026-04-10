use g3rs_hooks_rs_source_checks_types::G3RsHooksRsSourceChecksInput;
use hook_shell_parser::parse_script;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsHooksRsSourceChecksInput) -> Vec<G3CheckResult> {
    let parsed = parse_script(&input.content);
    let rule_input = crate::inputs::RustHookCommandInput {
        rel_path: &input.rel_path,
        parsed: &parsed,
    };
    let mut results = Vec::new();

    crate::hook_rs_01_fmt_step_present::check(&rule_input, &mut results);
    crate::hook_rs_02_clippy_step_present::check(&rule_input, &mut results);
    crate::hook_rs_03_cargo_deny_step_present::check(&rule_input, &mut results);
    crate::hook_rs_04_test_step_present::check(&rule_input, &mut results);
    crate::hook_rs_05_cargo_machete_step_present::check(&rule_input, &mut results);
    crate::hook_rs_07_duplication_tool_is_cargo_dupes::check(&rule_input, &mut results);
    crate::hook_rs_08_guardrail_validate_staged_present::check(&rule_input, &mut results);
    crate::hook_rs_09_clippy_denies_warnings::check(&rule_input, &mut results);
    crate::hook_rs_10_test_uses_workspace::check(&rule_input, &mut results);
    crate::hook_rs_11_gitleaks_step_present::check(&rule_input, &mut results);
    crate::hook_rs_12_cargo_dupes_step_present::check(&rule_input, &mut results);
    crate::hook_rs_13_cargo_dupes_excludes::check(&rule_input, &mut results);
    crate::hook_rs_16_config_changes_trigger_validation::check(&input.content, &rule_input, &mut results);

    crate::compat::finish(results)
}
