use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

pub(super) fn hook() -> g3rs_hooks_types::G3RsHooksSelectedHookConfigFact {
    g3rs_hooks_types::G3RsHooksSelectedHookConfigFact {
        rel_path: ".githooks/pre-commit".to_owned(),
        parsed: hook_shell_parser::parse_script("#!/usr/bin/env bash\n"),
    }
}

pub(super) fn requirement() -> G3HookRequirement {
    G3HookRequirement {
        id: "g3rs-fmt/hook-contract".to_owned(),
        owner_family: "fmt".to_owned(),
        trigger_patterns: vec![G3HookTriggerPattern::Glob("**/*.rs".to_owned())],
        required_commands: vec![G3HookCommandRequirement::CargoFmtCheck],
        critical_commands: vec![G3HookCriticalCommand::CargoSubcommand("fmt".to_owned())],
    }
}
