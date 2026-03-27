use super::facts::{CargoEditionState, RustfmtConfigKind, RustfmtFacts, ToolchainChannelState};

pub struct RustfmtRootInput {
    pub config_rel: Option<String>,
    pub parsed: Option<toml::Value>,
    pub cargo_edition: CargoEditionState,
    pub toolchain_channel: ToolchainChannelState,
}

pub struct RustfmtExtraConfigInput {
    pub config_rel: String,
    pub config_kind: RustfmtConfigKind,
}

pub struct RustfmtDualConflictInput {
    pub dir_rel: String,
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
