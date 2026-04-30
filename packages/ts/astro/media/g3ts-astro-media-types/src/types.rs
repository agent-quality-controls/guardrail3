use std::collections::BTreeMap;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMediaPolicySnapshot {
    pub rel_path: String,
    pub favicon: String,
    pub app_icons: Vec<String>,
    pub default_social_image: String,
    pub allow_svg_icons: Option<bool>,
    pub public_source_globs: Vec<String>,
    pub media_helper_modules: Vec<String>,
    pub approved_media_helpers: Vec<String>,
    pub content_image_components: Vec<String>,
    pub content_image_key_props: Vec<String>,
    pub banned_image_source_props: Vec<String>,
    pub banned_image_alt_props: Vec<String>,
    pub allowed_public_image_paths: Vec<String>,
    pub checked_image_extensions: Vec<String>,
    pub metadata_image_property_names: Vec<String>,
    pub extra_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroConfigSurfaceSnapshot {
    pub rel_path: String,
    pub integrations: Vec<G3TsAstroIntegrationSnapshot>,
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
pub enum G3TsAstroMediaPolicySurfaceState {
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
    MissingMediaPolicy {
        rel_path: String,
    },
    Parsed {
        snapshot: G3TsAstroMediaPolicySnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMediaEslintSurfaceSnapshot {
    pub rel_path: String,
    pub public_probe_present: bool,
    pub public_probe_ignored: bool,
    pub public_plugins: Vec<String>,
    pub public_plugin_package_names: BTreeMap<String, Vec<String>>,
    pub public_error_rules: Vec<String>,
    pub public_restricted_disable_patterns: Vec<String>,
    pub public_media_policy_rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroMediaEslintSurfaceState {
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
        snapshot: G3TsAstroMediaEslintSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroMediaIntegrationContractInput {
    pub app_root_rel_path: String,
    pub package: G3TsAstroPackageSurfaceState,
    pub astro_config: G3TsAstroConfigSurfaceState,
    pub astro_policy: G3TsAstroMediaPolicySurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMediaEslintPluginContractInput {
    pub app_root_rel_path: String,
    pub config: G3TsAstroMediaEslintSurfaceState,
    pub astro_policy: G3TsAstroMediaPolicySurfaceState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroMediaConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroMediaIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroMediaEslintPluginContractInput>,
}
