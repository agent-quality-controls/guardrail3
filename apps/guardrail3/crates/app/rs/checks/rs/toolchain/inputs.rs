use super::facts::ToolchainFacts;

pub struct ToolchainRootInput<'a> {
    pub toolchain_toml_rel: Option<&'a str>,
    pub legacy_toolchain_rel: Option<&'a str>,
    pub parsed: Option<&'a toml::Value>,
    pub parse_error: Option<&'a str>,
    pub cargo_rust_version: Option<&'a str>,
}

impl<'a> ToolchainRootInput<'a> {
    pub fn from_facts(facts: &'a ToolchainFacts) -> Self {
        Self {
            toolchain_toml_rel: facts.toolchain_toml_rel.as_deref(),
            legacy_toolchain_rel: facts.legacy_toolchain_rel.as_deref(),
            parsed: facts.parsed.as_ref(),
            parse_error: facts.parse_error.as_deref(),
            cargo_rust_version: facts.cargo_rust_version.as_deref(),
        }
    }
}
