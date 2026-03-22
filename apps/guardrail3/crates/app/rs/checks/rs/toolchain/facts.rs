#[derive(Debug, Clone)]
pub struct ToolchainFacts {
    pub toolchain_toml_rel: Option<String>,
    pub legacy_toolchain_rel: Option<String>,
    pub parsed: Option<toml::Value>,
    pub parse_error: Option<String>,
    pub cargo_rust_version: Option<String>,
}
