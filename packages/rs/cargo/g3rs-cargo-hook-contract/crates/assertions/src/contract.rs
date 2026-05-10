use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};

/// Internal.
///
/// # Panics
///
/// See body for assertions.
pub fn assert_contract_matches_expected_policy() {
    let actual = g3rs_cargo_hook_contract_runtime::hook_contract();
    let expected = vec![G3HookRequirement {
        id: "g3rs-cargo/hook-contract".to_owned(),
        owner_family: "cargo".to_owned(),
        trigger_patterns: vec![
            G3HookTriggerPattern::ExactPath("Cargo.toml".to_owned()),
            G3HookTriggerPattern::ExactPath("Cargo.lock".to_owned()),
            G3HookTriggerPattern::ExactPath(".cargo/config.toml".to_owned()),
            G3HookTriggerPattern::ExactPath(".cargo/config".to_owned()),
        ],
        required_commands: vec![G3HookCommandRequirement::ConcreteLockfileCommand],
        critical_commands: vec![G3HookCriticalCommand::Binary("cargo".to_owned())],
    }];
    assert_eq!(actual, expected, "cargo hook contract drift: {actual:#?}");
}
