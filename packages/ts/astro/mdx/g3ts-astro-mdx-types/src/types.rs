use std::collections::BTreeMap;

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
pub struct G3TsAstroMdxPolicySnapshot {
    pub rel_path: String,
    pub content_root: Option<String>,
    pub mdx_component_maps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroMdxPolicySurfaceState {
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
        snapshot: G3TsAstroMdxPolicySnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxApprovedSourcePaths {
    pub mdx_component_maps: Vec<String>,
    pub missing_mdx_component_maps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxMissingComponentMapInput {
    pub policy_rel_path: String,
    pub configured_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxEslintSurfaceSnapshot {
    pub rel_path: String,
    pub mdx_content_probe_present: bool,
    pub mdx_content_plugins: Vec<String>,
    pub mdx_content_plugin_package_names: BTreeMap<String, Vec<String>>,
    pub mdx_content_error_rules: Vec<String>,
    pub mdx_content_warn_or_error_rules: Vec<String>,
    pub mdx_content_restricted_disable_patterns: Vec<String>,
    pub mdx_content_unused_disable_fail_closed: bool,
    pub mdx_content_effective_mdx_component_map_rules: Vec<String>,
    pub mdx_content_effective_named_component_import_rules: Vec<String>,
    pub mdx_content_effective_no_raw_image_rules: Vec<String>,
    pub component_map_probe_present: bool,
    pub component_map_plugins: Vec<String>,
    pub component_map_plugin_package_names: BTreeMap<String, Vec<String>>,
    pub component_map_error_rules: Vec<String>,
    pub component_map_warn_or_error_rules: Vec<String>,
    pub component_map_restricted_disable_patterns: Vec<String>,
    pub component_map_unused_disable_fail_closed: bool,
    pub component_map_effective_no_raw_ui_export_rules: Vec<String>,
    pub component_map_effective_wrapper_zod_parse_rules: Vec<String>,
    pub component_map_probe_ignored: bool,
    pub mdx_content_probe_ignored: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroMdxEslintSurfaceState {
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
        snapshot: G3TsAstroMdxEslintSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxEslintPluginContractInput {
    pub app_root_rel_path: String,
    pub config: G3TsAstroMdxEslintSurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxEslintDirectiveInput {
    pub rel_path: String,
    pub directive_kind: String,
    pub disabled_rules: Vec<String>,
    pub all_rules: bool,
    pub line: u32,
    pub target_line: Option<u32>,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroMdxIntegrationContractInput {
    pub app_root_rel_path: String,
    pub mdx_sources: G3TsAstroMdxApprovedSourcePaths,
    pub package: G3TsAstroPackageSurfaceState,
    pub astro_policy: G3TsAstroMdxPolicySurfaceState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroMdxConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroMdxIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroMdxEslintPluginContractInput>,
    pub missing_component_map_sources: Vec<G3TsAstroMdxMissingComponentMapInput>,
    pub eslint_directives: Vec<G3TsAstroMdxEslintDirectiveInput>,
}
