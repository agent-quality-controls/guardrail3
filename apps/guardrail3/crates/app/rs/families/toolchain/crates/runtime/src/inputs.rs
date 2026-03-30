use super::facts::{
    AncestorToolchainFacts, DescendantToolchainFacts, ToolchainFamilyFacts, ToolchainPolicyRootFacts,
    UnownedToolchainFacts,
};

pub struct AncestorToolchainInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) is_legacy: bool,
    pub(crate) parsed: Option<&'a toml::Value>,
    pub(crate) parse_error: Option<&'a str>,
}

pub struct DescendantToolchainInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) is_legacy: bool,
}

pub struct UnownedToolchainInput<'a> {
    pub(crate) rel_path: &'a str,
    pub(crate) is_legacy: bool,
}

pub struct ToolchainPolicyRootInput<'a> {
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
    pub(crate) ancestor_toolchain: Option<AncestorToolchainInput<'a>>,
    pub(crate) descendant_toolchains: Vec<DescendantToolchainInput<'a>>,
}

impl<'a> ToolchainPolicyRootInput<'a> {
    pub fn from_facts(facts: &'a ToolchainPolicyRootFacts) -> Self {
        Self {
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
            ancestor_toolchain: facts
                .ancestor_toolchain
                .as_ref()
                .map(AncestorToolchainInput::from_facts),
            descendant_toolchains: facts
                .descendant_toolchains
                .iter()
                .map(DescendantToolchainInput::from_facts)
                .collect(),
        }
    }
}

impl<'a> AncestorToolchainInput<'a> {
    fn from_facts(facts: &'a AncestorToolchainFacts) -> Self {
        Self {
            rel_path: &facts.rel_path,
            is_legacy: facts.is_legacy,
            parsed: facts.parsed.as_ref(),
            parse_error: facts.parse_error.as_deref(),
        }
    }
}

impl<'a> DescendantToolchainInput<'a> {
    fn from_facts(facts: &'a DescendantToolchainFacts) -> Self {
        Self {
            rel_path: &facts.rel_path,
            is_legacy: facts.is_legacy,
        }
    }
}

impl<'a> UnownedToolchainInput<'a> {
    fn from_facts(facts: &'a UnownedToolchainFacts) -> Self {
        Self {
            rel_path: &facts.rel_path,
            is_legacy: facts.is_legacy,
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

pub fn all_unowned_from_facts(facts: &ToolchainFamilyFacts) -> Vec<UnownedToolchainInput<'_>> {
    facts
        .unowned_toolchains
        .iter()
        .map(UnownedToolchainInput::from_facts)
        .collect()
}
