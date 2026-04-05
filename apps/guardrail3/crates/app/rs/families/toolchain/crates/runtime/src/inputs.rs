use cargo_toml_parser::CargoToml;

use super::facts::{ToolchainFamilyFacts, ToolchainPolicyRootFacts};

pub struct ToolchainPolicyRootInput<'a> {
    #[allow(dead_code)]
    pub(crate) rel_dir: &'a str,
    #[allow(dead_code)]
    pub(crate) cargo_rel_path: &'a str,
    pub(crate) toolchain_toml_rel: Option<&'a str>,
    pub(crate) legacy_toolchain_rel: Option<&'a str>,
    pub(crate) parsed: Option<&'a rust_toolchain_toml_parser::RustToolchainToml>,
    pub(crate) parse_error: Option<&'a str>,
    pub(crate) cargo: Option<&'a CargoToml>,
    pub(crate) cargo_parse_error: Option<&'a str>,
}

impl<'a> ToolchainPolicyRootInput<'a> {
    pub fn from_facts(facts: &'a ToolchainPolicyRootFacts) -> Self {
        Self {
            rel_dir: &facts.rel_dir,
            cargo_rel_path: &facts.cargo_rel_path,
            toolchain_toml_rel: facts.toolchain_toml_rel.as_deref(),
            legacy_toolchain_rel: facts.legacy_toolchain_rel.as_deref(),
            parsed: facts.parsed.as_ref(),
            parse_error: facts.parse_error.as_deref(),
            cargo: facts.cargo_parsed.as_ref(),
            cargo_parse_error: facts.cargo_parse_error.as_deref(),
        }
    }
}

pub type ToolchainRootInput<'a> = ToolchainPolicyRootInput<'a>;

pub fn all_from_facts(facts: &ToolchainFamilyFacts) -> Vec<ToolchainPolicyRootInput<'_>> {
    facts
        .policy_roots
        .iter()
        .map(ToolchainPolicyRootInput::from_facts)
        .collect()
}
