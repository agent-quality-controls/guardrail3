use std::collections::BTreeMap;

/// Pair of (script name, script body) extracted from `package.json`.
type ScriptBody = (String, String);
/// Map of plugin name to its package names.
type PluginPackageMap = BTreeMap<String, Vec<String>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageSurfaceSnapshot {
    pub rel_path: String,
    pub package_name: Option<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub peer_dependencies: Vec<String>,
    pub script_names: Vec<String>,
    pub script_bodies: Vec<ScriptBody>,
    pub script_commands: Vec<G3TsAstroPackageScriptCommand>,
    pub script_tool_invocations: Vec<G3TsAstroPackageScriptToolInvocation>,
    pub script_all_tool_invocations: Vec<G3TsAstroPackageScriptToolInvocation>,
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

#[expect(
    clippy::large_enum_variant,
    reason = "Boxing the parsed snapshot would force constructor changes across consumer crates outside this types crate."
)]
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
    Array(Vec<Self>),
    Object(Vec<G3TsAstroStaticObjectProperty>),
    ImportedIdentifier {
        local_name: String,
        source_module: Option<String>,
        imported_name: Option<String>,
    },
    UnsupportedExpression {
        reason: String,
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
pub struct G3TsAstroSetupAppRootInput {
    pub app_root_rel_path: String,
    pub astro_config_rel_path: Option<String>,
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "Each bool flags an independent ESLint config dimension required by downstream contracts."
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSetupEslintSurfaceSnapshot {
    pub rel_path: String,
    pub astro_source_probe_present: bool,
    pub ts_source_probe_present: bool,
    pub tsx_source_probe_present: bool,
    pub astro_source_plugins: Vec<String>,
    pub ts_source_plugins: Vec<String>,
    pub tsx_source_plugins: Vec<String>,
    pub astro_source_plugin_meta_names: BTreeMap<String, String>,
    pub ts_source_plugin_meta_names: BTreeMap<String, String>,
    pub tsx_source_plugin_meta_names: BTreeMap<String, String>,
    pub astro_source_plugin_package_names: PluginPackageMap,
    pub ts_source_plugin_package_names: PluginPackageMap,
    pub tsx_source_plugin_package_names: PluginPackageMap,
    pub astro_source_error_rules: Vec<String>,
    pub ts_source_error_rules: Vec<String>,
    pub tsx_source_error_rules: Vec<String>,
    pub astro_source_warn_or_error_rules: Vec<String>,
    pub ts_source_warn_or_error_rules: Vec<String>,
    pub tsx_source_warn_or_error_rules: Vec<String>,
    pub astro_source_restricted_disable_patterns: Vec<String>,
    pub ts_source_restricted_disable_patterns: Vec<String>,
    pub tsx_source_restricted_disable_patterns: Vec<String>,
    pub astro_source_unused_disable_fail_closed: bool,
    pub ts_source_unused_disable_fail_closed: bool,
    pub tsx_source_unused_disable_fail_closed: bool,
    pub astro_source_probe_ignored: bool,
    pub ts_source_probe_ignored: bool,
    pub tsx_source_probe_ignored: bool,
}

#[expect(
    clippy::large_enum_variant,
    reason = "Boxing the parsed snapshot would force constructor changes across consumer crates outside this types crate."
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroSetupEslintSurfaceState {
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
        snapshot: G3TsAstroSetupEslintSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSetupEslintPluginContractInput {
    pub app_root_rel_path: String,
    pub config: G3TsAstroSetupEslintSurfaceState,
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
pub struct G3TsAstroSetupIntegrationContractInput {
    pub app_root_rel_path: String,
    pub package: G3TsAstroPackageSurfaceState,
    pub syncpack_config: G3TsAstroSyncpackConfigState,
    pub astro_config: G3TsAstroConfigSurfaceState,
    pub required_syncpack_pins: Vec<G3TsAstroSyncpackRequiredPin>,
    pub forbidden_syncpack_deps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroSetupConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroSetupIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroSetupEslintPluginContractInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSetupFileTreeChecksInput {
    pub app_roots: Vec<G3TsAstroSetupAppRootInput>,
}
