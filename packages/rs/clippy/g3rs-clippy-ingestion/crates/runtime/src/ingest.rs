use g3_guardrail_toml_types::WaiverConfig;
use g3rs_clippy_types::{
    G3RsClippyCargoConfigState, G3RsClippyCargoMemberState, G3RsClippyCargoRootState,
    G3RsClippyConfigChecksInput, G3RsClippyConfigState, G3RsClippyFileTreeChecksInput,
    G3RsClippyRustPolicyState, G3RsClippyShadowedConfig,
};

/// assemble config input fn.
pub(crate) const fn assemble_config_input(
    clippy_rel_path: String,
    clippy: G3RsClippyConfigState,
    rust_policy: G3RsClippyRustPolicyState,
    cargo_root: G3RsClippyCargoRootState,
    cargo_workspace_members: Vec<G3RsClippyCargoMemberState>,
    cargo_configs: Vec<G3RsClippyCargoConfigState>,
    waivers: Vec<WaiverConfig>,
) -> G3RsClippyConfigChecksInput {
    G3RsClippyConfigChecksInput {
        clippy_rel_path,
        clippy,
        rust_policy,
        cargo_root,
        cargo_workspace_members,
        cargo_configs,
        waivers,
    }
}

/// assemble filetree input fn.
pub(crate) const fn assemble_filetree_input(
    preferred_root_config_rel_path: Option<String>,
    shadowed_same_root_configs: Vec<G3RsClippyShadowedConfig>,
) -> G3RsClippyFileTreeChecksInput {
    G3RsClippyFileTreeChecksInput {
        preferred_root_config_rel_path,
        shadowed_same_root_configs,
    }
}
