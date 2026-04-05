use cargo_toml_parser::CargoToml;
use guardrail3_domain_config::types::EscapeHatchConfig;
use rust_toolchain_toml_parser::RustToolchainToml;
use rustfmt_toml_parser::RustfmtToml;

use super::facts::{RustfmtConfigKind, RustfmtFacts};

pub struct RustfmtRootInput {
    pub(crate) config_rel: Option<String>,
    pub(crate) parsed: Option<RustfmtToml>,
    pub(crate) parse_error: Option<String>,
    pub(crate) escape_hatches: Vec<EscapeHatchConfig>,
    pub(crate) cargo_rel_path: String,
    pub(crate) cargo: Option<CargoToml>,
    pub(crate) cargo_parse_error: Option<String>,
    pub(crate) toolchain_rel_path: String,
    pub(crate) toolchain: Option<RustToolchainToml>,
    pub(crate) toolchain_parse_error: Option<String>,
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
            parse_error: facts.root_parse_error.clone(),
            escape_hatches: facts.escape_hatches.clone(),
            cargo_rel_path: facts.cargo_rel_path.clone(),
            cargo: facts.cargo_parsed.clone(),
            cargo_parse_error: facts.cargo_parse_error.clone(),
            toolchain_rel_path: facts.toolchain_rel_path.clone(),
            toolchain: facts.toolchain_parsed.clone(),
            toolchain_parse_error: facts.toolchain_parse_error.clone(),
        }
    }
}
