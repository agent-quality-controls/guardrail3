use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsHooksSourceChecksInput) -> Vec<G3CheckResult> {
    let kind = match input.kind {
        G3RsHookScriptKind::PreCommit => crate::facts::HookScriptKind::PreCommit,
        G3RsHookScriptKind::Modular => crate::facts::HookScriptKind::Modular,
    };
    let rust_input = crate::inputs::RustHookCommandInput {
        rel_path: &input.rel_path,
        parsed: &input.parsed,
        is_workspace_project: input.is_workspace_project,
    };
    let executable_input = crate::inputs::ExecutableCommandContextInput {
        rel_path: &input.rel_path,
        kind,
        parsed: &input.parsed,
    };
    let dispatcher_input = crate::inputs::DispatcherSyntaxInput {
        rel_path: &input.rel_path,
        has_modular_dir: input.has_modular_dir,
        parsed: &input.parsed,
    };
    let fail_open_input = crate::inputs::FailOpenWrapperInput {
        rel_path: &input.rel_path,
        parsed: &input.parsed,
    };
    let mut results = Vec::new();

    if input.kind == G3RsHookScriptKind::PreCommit {
        crate::bootstrap::dispatcher_pattern::check(&dispatcher_input, &mut results);
        crate::shell_safety::real_dispatcher_syntax_only::check(&dispatcher_input, &mut results);

        crate::fmt_step_present::check(&rust_input, &mut results);
        crate::clippy_step_present::check(&rust_input, &mut results);
        crate::cargo_deny_step_present::check(&rust_input, &mut results);
        crate::test_step_present::check(&rust_input, &mut results);
        crate::cargo_machete_step_present::check(&rust_input, &mut results);
        crate::duplication_tool_is_cargo_dupes::check(&rust_input, &mut results);
        crate::guardrail_validate_staged_present::check(&rust_input, &mut results);
        crate::clippy_denies_warnings::check(&rust_input, &mut results);
        crate::test_uses_workspace::check(&rust_input, &mut results);
        crate::gitleaks_step_present::check(&rust_input, &mut results);
        crate::cargo_dupes_step_present::check(&rust_input, &mut results);
        crate::cargo_dupes_excludes::check(&rust_input, &mut results);
        crate::config_changes_trigger_validation::check(&rust_input, &mut results);
        crate::shared_target_dir_present::check(&rust_input, &mut results);
    }

    crate::shell_safety::shell_error_handling::check(&executable_input, &mut results);
    crate::shell_safety::valid_shebang::check(&executable_input, &mut results);
    crate::shell_safety::no_unconditional_exit_zero::check(&executable_input, &mut results);
    crate::shell_safety::no_bypass_instructions::check(&executable_input, &mut results);
    crate::workflow::merge_conflict_step_present::check(&executable_input, &mut results);
    crate::workflow::file_size_step_present::check(&executable_input, &mut results);
    crate::shell_safety::executable_command_context_only::check(&executable_input, &mut results);
    crate::shell_safety::concrete_lockfile_command::check(&executable_input, &mut results);
    crate::shell_safety::no_fail_open_wrappers::check(&fail_open_input, &mut results);

    crate::compat::finish(results)
}
