use cargo_toml_parser::CargoToml;

#[derive(Debug, Clone)]
pub struct ToolchainPolicyRootFacts {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) toolchain_toml_rel: Option<String>,
    pub(crate) legacy_toolchain_rel: Option<String>,
    pub(crate) parsed: Option<rust_toolchain_toml_parser::RustToolchainToml>,
    pub(crate) parse_error: Option<String>,
    pub(crate) cargo_parsed: Option<CargoToml>,
    pub(crate) cargo_parse_error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ToolchainFamilyFacts {
    pub(crate) policy_roots: Vec<ToolchainPolicyRootFacts>,
}
