use guardrail3_app_rs_family_deny::facts::DenyFacts;

pub fn assert_root_profile_name(facts: &DenyFacts, expected: &str) {
    let root = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == "deny.toml")
        .expect("expected root deny.toml facts");
    assert_eq!(root.profile_name.as_deref(), Some(expected));
}
