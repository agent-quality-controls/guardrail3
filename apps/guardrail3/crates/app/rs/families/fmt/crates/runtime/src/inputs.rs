use super::facts::{CargoEditionState, RustfmtConfigKind, RustfmtFacts, ToolchainChannelState};

pub struct RustfmtRootInput {
    pub(crate) config_rel: Option<String>,
    pub(crate) parsed: Option<toml::Value>,
    pub(crate) cargo_edition: CargoEditionState,
    pub(crate) toolchain_channel: ToolchainChannelState,
}

pub struct RustfmtExtraConfigInput {
    pub(crate) config_rel: String,
    pub(crate) config_kind: RustfmtConfigKind,
}

pub struct RustfmtDualConflictInput {
    pub(crate) dir_rel: String,
}

impl RustfmtRootInput {
    pub fn from_facts(facts: &RustfmtFacts) -> Self {
        Self {
            config_rel: facts.root_config_rel.clone(),
            parsed: facts.root_parsed.clone(),
            cargo_edition: facts.cargo_edition.clone(),
            toolchain_channel: facts.toolchain_channel.clone(),
        }
    }
}
