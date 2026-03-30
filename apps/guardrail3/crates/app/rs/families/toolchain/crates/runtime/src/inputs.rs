use super::facts::{PolicyRootKind, ToolchainFamilyFacts, ToolchainPolicyRootFacts};

pub struct ToolchainPolicyRootInput<'a> {
    pub(crate) kind: PolicyRootKind,
    #[allow(dead_code)]
    pub(crate) rel_dir: &'a str,
    #[allow(dead_code)]
    pub(crate) cargo_rel_path: &'a str,
    pub(crate) cargo_toml_rel: Option<&'a str>,
    pub(crate) toolchain_toml_rel: Option<&'a str>,
    pub(crate) legacy_toolchain_rel: Option<&'a str>,
    pub(crate) parsed: Option<&'a toml::Value>,
    pub(crate) parse_error: Option<&'a str>,
    pub(crate) cargo_rust_version: Option<&'a str>,
    pub(crate) cargo_rust_version_invalid: bool,
    pub(crate) cargo_parse_error: Option<&'a str>,
}

impl<'a> ToolchainPolicyRootInput<'a> {
    pub fn from_facts(facts: &'a ToolchainPolicyRootFacts) -> Self {
        Self {
            kind: facts.kind,
            rel_dir: &facts.rel_dir,
            cargo_rel_path: &facts.cargo_rel_path,
            cargo_toml_rel: Some(&facts.cargo_rel_path),
            toolchain_toml_rel: facts.toolchain_toml_rel.as_deref(),
            legacy_toolchain_rel: facts.legacy_toolchain_rel.as_deref(),
            parsed: facts.parsed.as_ref(),
            parse_error: facts.parse_error.as_deref(),
            cargo_rust_version: facts.cargo_rust_version.as_deref(),
            cargo_rust_version_invalid: facts.cargo_rust_version_invalid,
            cargo_parse_error: facts.cargo_parse_error.as_deref(),
        }
    }
}

pub type ToolchainRootInput<'a> = ToolchainPolicyRootInput<'a>;

pub fn all_from_facts(facts: &ToolchainFamilyFacts) -> Vec<ToolchainPolicyRootInput<'_>> {
    facts.policy_roots
        .iter()
        .map(ToolchainPolicyRootInput::from_facts)
        .collect()
}
