#![recursion_limit = "1024"]

mod facts;
mod hook_rs_01_fmt_step_present;
mod hook_rs_02_clippy_step_present;
mod hook_rs_03_cargo_deny_step_present;
mod hook_rs_04_test_step_present;
mod hook_rs_05_cargo_machete_step_present;
mod hook_rs_06_required_tools_installed;
mod hook_rs_07_duplication_tool_is_cargo_dupes;
mod hook_rs_08_guardrail_validate_staged_present;
mod hook_rs_09_clippy_denies_warnings;
mod hook_rs_10_test_uses_workspace;
mod hook_rs_11_gitleaks_step_present;
mod hook_rs_12_cargo_dupes_step_present;
mod hook_rs_13_cargo_dupes_excludes_tests;
mod hook_rs_14_guardrail_binary_available;
mod hook_rs_15_cargo_dupes_installed;
mod hook_rs_16_config_changes_trigger_validation;
mod inputs;
#[cfg(test)]
mod test_support;

use guardrail3_app_rs_family_hooks_shared::hook_shell::parse_script;
use guardrail3_domain_report::CheckResult;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::ToolChecker;

use self::facts::collect;
use self::inputs::RustHookCommandInput;

pub fn check(tree: &ProjectTree, tc: &dyn ToolChecker) -> Vec<CheckResult> {
    let facts = collect(tree);
    let mut results = Vec::new();

    let (Some(rel_path), Some(content)) = (
        facts.pre_commit_rel_path.as_deref(),
        facts.pre_commit_content.as_deref(),
    ) else {
        return results;
    };

    let parsed = parse_script(content);
    let input = RustHookCommandInput {
        rel_path,
        parsed: &parsed,
    };

    hook_rs_01_fmt_step_present::check(&input, &mut results);
    hook_rs_02_clippy_step_present::check(&input, &mut results);
    hook_rs_03_cargo_deny_step_present::check(&input, &mut results);
    hook_rs_04_test_step_present::check(&input, &mut results);
    hook_rs_05_cargo_machete_step_present::check(&input, &mut results);
    hook_rs_06_required_tools_installed::check(rel_path, tc, &mut results);
    hook_rs_07_duplication_tool_is_cargo_dupes::check(&input, &mut results);
    hook_rs_08_guardrail_validate_staged_present::check(&input, &mut results);
    hook_rs_09_clippy_denies_warnings::check(&input, &mut results);
    hook_rs_10_test_uses_workspace::check(&input, &mut results);
    hook_rs_11_gitleaks_step_present::check(&input, &mut results);
    hook_rs_12_cargo_dupes_step_present::check(&input, &mut results);
    hook_rs_13_cargo_dupes_excludes_tests::check(&input, &mut results);
    let guardrail_validation_expected =
        hook_rs_08_guardrail_validate_staged_present::script_contains_guardrail_step(input.parsed);
    let guardrail_validation_path_qualified =
        hook_rs_08_guardrail_validate_staged_present::script_contains_path_qualified_guardrail_step(
            input.parsed,
        );
    hook_rs_14_guardrail_binary_available::check(
        rel_path,
        guardrail_validation_expected,
        guardrail_validation_path_qualified,
        tc,
        &mut results,
    );
    hook_rs_15_cargo_dupes_installed::check(
        rel_path,
        hook_rs_12_cargo_dupes_step_present::script_contains_cargo_dupes(input.parsed),
        hook_rs_12_cargo_dupes_step_present::script_contains_path_qualified_cargo_dupes(
            input.parsed,
        ),
        tc,
        &mut results,
    );
    hook_rs_16_config_changes_trigger_validation::check(content, &input, &mut results);

    results
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
