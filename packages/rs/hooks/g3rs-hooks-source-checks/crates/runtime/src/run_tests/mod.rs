use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookRequirement, G3HookTriggerPattern,
};
use g3rs_hooks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};
use hook_shell_parser::parse_script;

use super::check_all;

#[test]
fn required_contract_commands_are_checked_across_modular_hook_surface() {
    let inputs = vec![
        input(
            ".githooks/pre-commit",
            G3RsHookScriptKind::PreCommit,
            "#!/bin/sh\nrun-parts .githooks/pre-commit.d\n",
            vec![requirement(G3HookCommandRequirement::G3RsValidatePath)],
        ),
        input(
            ".githooks/pre-commit.d/rust",
            G3RsHookScriptKind::Modular,
            "#!/bin/sh\ng3rs validate --path .\n",
            Vec::new(),
        ),
    ];

    let results = check_all(&inputs);

    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-hooks/required-contract-command-present"
                && result.inventory()
                && result.message().contains("Owner families: test")
        }),
        "contract command in modular hook script should satisfy pre-commit contract surface"
    );
}

fn input(
    rel_path: &str,
    kind: G3RsHookScriptKind,
    content: &str,
    requirements: Vec<G3HookRequirement>,
) -> G3RsHooksSourceChecksInput {
    G3RsHooksSourceChecksInput {
        rel_path: rel_path.to_owned(),
        kind,
        parsed: parse_script(content),
        has_modular_dir: true,
        is_workspace_project: true,
        requirements,
    }
}

fn requirement(command: G3HookCommandRequirement) -> G3HookRequirement {
    G3HookRequirement {
        id: "test/hook-contract".to_owned(),
        owner_family: "test".to_owned(),
        trigger_patterns: vec![G3HookTriggerPattern::Glob("**/*.rs".to_owned())],
        required_commands: vec![command],
        critical_commands: Vec::new(),
    }
}
