use std::collections::BTreeMap;

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
    pub velite_config_rel_path: Option<String>,
    pub velite_output_rel_paths: Vec<String>,
    pub legacy_generated_state_rel_paths: Vec<String>,
    pub forbidden_state_rel_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroRouteMarkdownPageInput {
    pub rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageSurfaceSnapshot {
    pub rel_path: String,
    pub package_name: Option<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub script_names: Vec<String>,
    pub script_bodies: Vec<(String, String)>,
    pub script_commands: Vec<G3TsAstroPackageScriptCommand>,
    pub script_tool_invocations: Vec<G3TsAstroPackageScriptToolInvocation>,
    pub script_parse_blockers: Vec<G3TsAstroPackageScriptParseBlocker>,
    pub safely_runs_astro_check: bool,
    pub safely_runs_astro_build: bool,
    pub safely_runs_syncpack_lint: bool,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSyncpackConfigSnapshot {
    pub rel_path: String,
    pub source_covers_package_manifest: bool,
    pub missing_required_stack_pins: Vec<G3TsAstroSyncpackRequiredPin>,
    pub missing_forbidden_bans: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSyncpackRequiredPin {
    pub dependency: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPolicySnapshot {
    pub rel_path: String,
    pub profile: Option<String>,
    pub content_routes: Vec<String>,
    pub non_content_routes: Vec<String>,
    pub endpoints: Vec<String>,
    pub content_root: Option<String>,
    pub content_adapters: Vec<String>,
    pub required_collections: Vec<String>,
    pub collection_fields: BTreeMap<String, Vec<String>>,
    pub mdx_component_maps: Vec<String>,
    pub metadata_helpers: Vec<String>,
    pub json_ld_helpers: Vec<String>,
    pub forbidden_state: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroPolicySurfaceState {
    Missing { rel_path: String },
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    MissingAstroPolicy { rel_path: String },
    Parsed { snapshot: G3TsAstroPolicySnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroSyncpackConfigState {
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
        snapshot: G3TsAstroSyncpackConfigSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroConfigSurfaceSnapshot {
    pub rel_path: String,
    pub site: Option<String>,
    pub output: Option<G3TsAstroOutputMode>,
    pub integrations: Vec<G3TsAstroIntegrationSnapshot>,
    pub adapter: Option<G3TsAstroIntegrationSnapshot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsAstroOutputMode {
    Static,
    Server,
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
pub struct G3TsAstroContentAdapterSourcePaths {
    pub content_adapter: Vec<String>,
    pub content_adapter_astro_content: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPolicyModuleSourcePaths {
    pub source_paths: Vec<String>,
    pub missing_policy_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxApprovedSourcePaths {
    pub mdx_component_maps: Vec<String>,
    pub missing_mdx_component_maps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoApprovedSourcePaths {
    pub metadata_helpers: Vec<String>,
    pub missing_metadata_helpers: Vec<String>,
    pub json_ld_helpers: Vec<String>,
    pub missing_json_ld_helpers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroEslintSurfaceSnapshot {
    pub rel_path: String,
    pub astro_source_probe_present: bool,
    pub ts_source_probe_present: bool,
    pub tsx_source_probe_present: bool,
    pub mdx_content_probe_present: bool,
    pub astro_source_plugins: Vec<String>,
    pub ts_source_plugins: Vec<String>,
    pub tsx_source_plugins: Vec<String>,
    pub mdx_content_plugins: Vec<String>,
    pub astro_source_plugin_meta_names: BTreeMap<String, String>,
    pub ts_source_plugin_meta_names: BTreeMap<String, String>,
    pub tsx_source_plugin_meta_names: BTreeMap<String, String>,
    pub mdx_content_plugin_meta_names: BTreeMap<String, String>,
    pub astro_source_plugin_package_names: BTreeMap<String, Vec<String>>,
    pub ts_source_plugin_package_names: BTreeMap<String, Vec<String>>,
    pub tsx_source_plugin_package_names: BTreeMap<String, Vec<String>>,
    pub mdx_content_plugin_package_names: BTreeMap<String, Vec<String>>,
    pub astro_source_error_rules: Vec<String>,
    pub ts_source_error_rules: Vec<String>,
    pub tsx_source_error_rules: Vec<String>,
    pub mdx_content_error_rules: Vec<String>,
    pub astro_source_effective_route_scoped_pipeline_rules: Vec<String>,
    pub ts_source_effective_route_scoped_pipeline_rules: Vec<String>,
    pub tsx_source_effective_route_scoped_pipeline_rules: Vec<String>,
    pub astro_source_effective_content_adapter_modules: Vec<String>,
    pub ts_source_effective_content_adapter_modules: Vec<String>,
    pub tsx_source_effective_content_adapter_modules: Vec<String>,
    pub astro_source_route_scoped_pipeline_rule_scopes: Vec<G3TsAstroPipelineRuleScopeSnapshot>,
    pub ts_source_route_scoped_pipeline_rule_scopes: Vec<G3TsAstroPipelineRuleScopeSnapshot>,
    pub tsx_source_route_scoped_pipeline_rule_scopes: Vec<G3TsAstroPipelineRuleScopeSnapshot>,
    pub astro_source_effective_content_data_pipeline_rules: Vec<String>,
    pub ts_source_effective_content_data_pipeline_rules: Vec<String>,
    pub tsx_source_effective_content_data_pipeline_rules: Vec<String>,
    pub astro_source_effective_content_source_pipeline_rules: Vec<String>,
    pub ts_source_effective_content_source_pipeline_rules: Vec<String>,
    pub tsx_source_effective_content_source_pipeline_rules: Vec<String>,
    pub astro_source_effective_inline_public_content_rules: Vec<String>,
    pub ts_source_effective_inline_public_content_rules: Vec<String>,
    pub tsx_source_effective_inline_public_content_rules: Vec<String>,
    pub mdx_content_effective_mdx_component_map_rules: Vec<String>,
    pub astro_source_effective_metadata_helper_rules: Vec<String>,
    pub ts_source_effective_metadata_helper_rules: Vec<String>,
    pub tsx_source_effective_metadata_helper_rules: Vec<String>,
    pub astro_source_effective_json_ld_helper_rules: Vec<String>,
    pub ts_source_effective_json_ld_helper_rules: Vec<String>,
    pub tsx_source_effective_json_ld_helper_rules: Vec<String>,
    pub astro_source_probe_ignored: bool,
    pub ts_source_probe_ignored: bool,
    pub tsx_source_probe_ignored: bool,
    pub mdx_content_probe_ignored: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPipelineRuleScopeSnapshot {
    pub rule_name: String,
    pub route_globs: Vec<String>,
    pub endpoint_globs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroEslintSurfaceState {
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
        snapshot: G3TsAstroEslintSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroEslintPluginContractInput {
    pub app_root_rel_path: String,
    pub config: G3TsAstroEslintSurfaceState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroSetupIntegrationContractInput {
    pub app_root_rel_path: String,
    pub content_mode: G3TsAstroContentMode,
    pub package: G3TsAstroPackageSurfaceState,
    pub syncpack_config: G3TsAstroSyncpackConfigState,
    pub astro_config: G3TsAstroConfigSurfaceState,
    pub required_syncpack_pins: Vec<G3TsAstroSyncpackRequiredPin>,
    pub forbidden_syncpack_deps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroContentIntegrationContractInput {
    pub app_root_rel_path: String,
    pub route_page_paths: Vec<String>,
    pub endpoint_paths: Vec<String>,
    pub content_adapter_sources: G3TsAstroContentAdapterSourcePaths,
    pub package: G3TsAstroPackageSurfaceState,
    pub astro_policy: G3TsAstroPolicySurfaceState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroMdxIntegrationContractInput {
    pub app_root_rel_path: String,
    pub mdx_sources: G3TsAstroMdxApprovedSourcePaths,
    pub package: G3TsAstroPackageSurfaceState,
    pub astro_policy: G3TsAstroPolicySurfaceState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroSeoIntegrationContractInput {
    pub app_root_rel_path: String,
    pub seo_sources: G3TsAstroSeoApprovedSourcePaths,
    pub package: G3TsAstroPackageSurfaceState,
    pub astro_config: G3TsAstroConfigSurfaceState,
    pub astro_policy: G3TsAstroPolicySurfaceState,
    pub llms_txt_rel_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroSetupConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroSetupIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroEslintPluginContractInput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroContentConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroContentIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroEslintPluginContractInput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroMdxConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroMdxIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroEslintPluginContractInput>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroSeoConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroSeoIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroEslintPluginContractInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSetupFileTreeChecksInput {
    pub app_roots: Vec<G3TsAstroAppRootInput>,
    pub live_collection_roots: Vec<G3TsAstroAppRootInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentFileTreeChecksInput {
    pub app_roots: Vec<G3TsAstroAppRootInput>,
    pub build_collection_roots: Vec<G3TsAstroAppRootInput>,
    pub live_collection_roots: Vec<G3TsAstroAppRootInput>,
    pub route_markdown_pages: Vec<G3TsAstroRouteMarkdownPageInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroStateFileTreeChecksInput {
    pub build_collection_roots: Vec<G3TsAstroAppRootInput>,
    pub live_collection_roots: Vec<G3TsAstroAppRootInput>,
}
