use g3rs_clippy_types::{
    G3RsClippyCargoConfigOverride, G3RsClippyConfigChecksInput, G3RsClippyConfigState,
    G3RsClippyFileTreeChecksInput, G3RsClippyPolicyContextState, G3RsClippyShadowedConfig,
};

pub(crate) fn assemble_config_input(
    clippy_rel_path: String,
    clippy: G3RsClippyConfigState,
    policy_context: G3RsClippyPolicyContextState,
    published_library_policy: bool,
    cargo_config_overrides: Vec<G3RsClippyCargoConfigOverride>,
) -> G3RsClippyConfigChecksInput {
    G3RsClippyConfigChecksInput {
        clippy_rel_path,
        clippy,
        policy_context,
        published_library_policy,
        cargo_config_overrides,
    }
}

pub(crate) fn assemble_filetree_input(
    preferred_root_config_rel_path: Option<String>,
    shadowed_same_root_configs: Vec<G3RsClippyShadowedConfig>,
) -> G3RsClippyFileTreeChecksInput {
    G3RsClippyFileTreeChecksInput {
        preferred_root_config_rel_path,
        shadowed_same_root_configs,
    }
}
