#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageSurfaceSnapshot {
    pub rel_path: String,
    pub package_name: Option<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub peer_dependencies: Vec<String>,
    pub script_names: Vec<String>,
    pub script_bodies: Vec<(String, String)>,
    pub script_commands: Vec<G3TsAstroPackageScriptCommand>,
    pub script_tool_invocations: Vec<G3TsAstroPackageScriptToolInvocation>,
    pub script_parse_blockers: Vec<G3TsAstroPackageScriptParseBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageScriptCommand {
    pub script_name: String,
    pub invocation: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<G3TsAstroPackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageScriptToolInvocation {
    pub script_name: String,
    pub command_index: usize,
    pub invocation: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<G3TsAstroPackageScriptCommandSeparator>,
    pub followed_by: Option<G3TsAstroPackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsAstroPackageScriptCommandSeparator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageScriptParseBlocker {
    pub script_name: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroPackageSurfaceState {
    Missing {
        rel_path: String,
    },
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroConfigSurfaceSnapshot {
    pub rel_path: String,
    pub site: Option<String>,
    pub output: Option<G3TsAstroOutputMode>,
    pub out_dir: Option<String>,
    pub trailing_slash: Option<G3TsAstroTrailingSlashPolicy>,
    pub integrations: Vec<G3TsAstroIntegrationSnapshot>,
    pub adapter: Option<G3TsAstroIntegrationSnapshot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsAstroOutputMode {
    Static,
    Server,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsAstroTrailingSlashPolicy {
    Always,
    Never,
    Ignore,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroIntegrationSnapshot {
    pub source_module: Option<String>,
    pub name: Option<String>,
    pub imported_name: Option<String>,
    pub call: Option<G3TsAstroCallSnapshot>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroCallSnapshot {
    pub first_arg: Option<G3TsAstroStaticValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum G3TsAstroStaticValue {
    Bool(bool),
    Number(f64),
    String(String),
    Null,
    Array(Vec<G3TsAstroStaticValue>),
    Object(Vec<G3TsAstroStaticObjectProperty>),
    ImportedIdentifier {
        local_name: String,
        source_module: Option<String>,
        imported_name: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroStaticObjectProperty {
    pub key: String,
    pub value: G3TsAstroStaticValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum G3TsAstroConfigSurfaceState {
    Missing {
        rel_path: String,
    },
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        snapshot: G3TsAstroConfigSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoPolicySnapshot {
    pub rel_path: String,
    pub metadata_helpers: Vec<String>,
    pub json_ld_helpers: Vec<String>,
    pub strict_ai_readable: bool,
    pub llms_required_sections: Vec<String>,
    pub llms_required_links: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroSeoPolicySurfaceState {
    Missing {
        rel_path: String,
    },
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    MissingAstroPolicy {
        rel_path: String,
    },
    Parsed {
        snapshot: G3TsAstroSeoPolicySnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoApprovedSourcePaths {
    pub metadata_helpers: Vec<String>,
    pub missing_metadata_helpers: Vec<String>,
    pub json_ld_helpers: Vec<String>,
    pub missing_json_ld_helpers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoMissingMetadataHelperInput {
    pub policy_rel_path: String,
    pub configured_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoMissingJsonLdHelperInput {
    pub policy_rel_path: String,
    pub configured_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoEslintSurfaceSnapshot {
    pub rel_path: String,
    pub astro_source_probe_present: bool,
    pub ts_source_probe_present: bool,
    pub tsx_source_probe_present: bool,
    pub astro_source_effective_metadata_helper_rules: Vec<String>,
    pub ts_source_effective_metadata_helper_rules: Vec<String>,
    pub tsx_source_effective_metadata_helper_rules: Vec<String>,
    pub astro_source_effective_json_ld_helper_rules: Vec<String>,
    pub ts_source_effective_json_ld_helper_rules: Vec<String>,
    pub tsx_source_effective_json_ld_helper_rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroSeoEslintSurfaceState {
    Missing {
        rel_path: String,
    },
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        snapshot: G3TsAstroSeoEslintSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoEslintPluginContractInput {
    pub app_root_rel_path: String,
    pub config: G3TsAstroSeoEslintSurfaceState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroSeoIntegrationContractInput {
    pub app_root_rel_path: String,
    pub seo_sources: G3TsAstroSeoApprovedSourcePaths,
    pub package: G3TsAstroPackageSurfaceState,
    pub astro_config: G3TsAstroConfigSurfaceState,
    pub astro_policy: G3TsAstroSeoPolicySurfaceState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroSeoConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroSeoIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroSeoEslintPluginContractInput>,
    pub missing_metadata_helper_sources: Vec<G3TsAstroSeoMissingMetadataHelperInput>,
    pub missing_json_ld_helper_sources: Vec<G3TsAstroSeoMissingJsonLdHelperInput>,
}
