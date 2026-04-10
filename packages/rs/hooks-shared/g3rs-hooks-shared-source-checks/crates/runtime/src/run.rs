use g3rs_hooks_shared_source_checks_types::{G3RsHookScriptKind, G3RsHooksSharedSourceChecksInput};
use hook_shell_parser::parse_script;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsHooksSharedSourceChecksInput) -> Vec<G3CheckResult> {
    let parsed = parse_script(&input.content);
    let kind = match input.kind {
        G3RsHookScriptKind::PreCommit => crate::facts::HookScriptKind::PreCommit,
        G3RsHookScriptKind::Modular => crate::facts::HookScriptKind::Modular,
    };
    let executable_input = crate::inputs::ExecutableCommandContextInput {
        rel_path: &input.rel_path,
        kind,
        content: &input.content,
        parsed: &parsed,
    };
    let dispatcher_input = crate::inputs::DispatcherSyntaxInput {
        rel_path: &input.rel_path,
        has_modular_dir: input.has_modular_dir,
        parsed: &parsed,
    };
    let fail_open_input = crate::inputs::FailOpenWrapperInput {
        rel_path: &input.rel_path,
        executable_lines: parsed.executable_lines(),
    };
    let mut results = Vec::new();

    if input.kind == G3RsHookScriptKind::PreCommit {
        crate::bootstrap::hook_shared_04_dispatcher_pattern::check(&dispatcher_input, &mut results);
        crate::shell_safety::hook_shared_19_real_dispatcher_syntax_only::check(
            &dispatcher_input,
            &mut results,
        );
    }

    crate::shell_safety::hook_shared_10_shell_error_handling::check(&executable_input, &mut results);
    crate::shell_safety::hook_shared_11_valid_shebang::check(&executable_input, &mut results);
    crate::shell_safety::hook_shared_13_no_unconditional_exit_zero::check(
        &executable_input,
        &mut results,
    );
    crate::shell_safety::hook_shared_14_no_bypass_instructions::check(
        &executable_input,
        &mut results,
    );
    crate::workflow::hook_shared_15_merge_conflict_step_present::check(
        &executable_input,
        &mut results,
    );
    crate::workflow::hook_shared_16_file_size_step_present::check(&executable_input, &mut results);
    crate::shell_safety::hook_shared_18_executable_command_context_only::check(
        &executable_input,
        &mut results,
    );
    crate::shell_safety::hook_shared_20_concrete_lockfile_command::check(
        &executable_input,
        &mut results,
    );
    crate::shell_safety::hook_shared_21_no_fail_open_wrappers::check(
        &fail_open_input,
        &mut results,
    );

    crate::compat::finish(results)
}
