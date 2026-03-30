#[derive(Debug, Clone)]
pub struct AncestorToolchainFacts {
    pub(crate) rel_path: String,
    pub(crate) is_legacy: bool,
    pub(crate) parsed: Option<toml::Value>,
    pub(crate) parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DescendantToolchainFacts {
    pub(crate) rel_path: String,
    pub(crate) is_legacy: bool,
}

#[derive(Debug, Clone)]
pub struct UnownedToolchainFacts {
    pub(crate) rel_path: String,
    pub(crate) is_legacy: bool,
}

#[derive(Debug, Clone)]
pub struct ToolchainPolicyRootFacts {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) toolchain_toml_rel: Option<String>,
    pub(crate) legacy_toolchain_rel: Option<String>,
    pub(crate) parsed: Option<toml::Value>,
    pub(crate) parse_error: Option<String>,
    pub(crate) cargo_rust_version: Option<String>,
    pub(crate) cargo_rust_version_invalid: bool,
    pub(crate) cargo_parse_error: Option<String>,
    pub(crate) ancestor_toolchain: Option<AncestorToolchainFacts>,
    pub(crate) descendant_toolchains: Vec<DescendantToolchainFacts>,
}

#[derive(Debug, Clone, Default)]
pub struct ToolchainFamilyFacts {
    pub(crate) policy_roots: Vec<ToolchainPolicyRootFacts>,
    pub(crate) unowned_toolchains: Vec<UnownedToolchainFacts>,
}
