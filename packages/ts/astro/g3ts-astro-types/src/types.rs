#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsAstroContentMode {
    None,
    BuildCollections,
    LiveCollections,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroAppRootInput {
    pub app_root_rel_path: String,
    pub astro_config_rel_path: Option<String>,
    pub content_config_rel_path: Option<String>,
    pub live_config_rel_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroRouteMarkdownPageInput {
    pub rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageSurfaceSnapshot {
    pub rel_path: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub script_names: Vec<String>,
    pub script_bodies: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroPackageSurfaceState {
    Missing { rel_path: String },
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsAstroPackageSurfaceSnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroIntegrationContractInput {
    pub app_root_rel_path: String,
    pub content_mode: G3TsAstroContentMode,
    pub package: G3TsAstroPackageSurfaceState,
    pub requires_source_pipeline_linting: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroEslintSurfaceSnapshot {
    pub rel_path: String,
    pub astro_source_probe_present: bool,
    pub ts_source_probe_present: bool,
    pub tsx_source_probe_present: bool,
    pub astro_source_plugins: Vec<String>,
    pub ts_source_plugins: Vec<String>,
    pub tsx_source_plugins: Vec<String>,
    pub astro_source_error_rules: Vec<String>,
    pub ts_source_error_rules: Vec<String>,
    pub tsx_source_error_rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroEslintSurfaceState {
    Missing { rel_path: String },
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsAstroEslintSurfaceSnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroEslintPluginContractInput {
    pub app_root_rel_path: String,
    pub config: G3TsAstroEslintSurfaceState,
    pub requires_source_pipeline_linting: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroEslintPluginContractInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroFileTreeChecksInput {
    pub app_roots: Vec<G3TsAstroAppRootInput>,
    pub build_collection_roots: Vec<G3TsAstroAppRootInput>,
    pub live_collection_roots: Vec<G3TsAstroAppRootInput>,
    pub route_markdown_pages: Vec<G3TsAstroRouteMarkdownPageInput>,
}
