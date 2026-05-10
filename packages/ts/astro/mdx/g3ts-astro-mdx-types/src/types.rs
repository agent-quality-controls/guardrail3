use std::collections::BTreeMap;

/// Map of plugin name to its package names; used for `ESLint` plugin maps.
type PluginPackageMap = BTreeMap<String, Vec<String>>;
/// Pair of (script name, script body) extracted from `package.json`.
type ScriptBody = (String, String);

/// Snapshot of the parsed `package.json` surface relevant to Astro MDX checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageSurfaceSnapshot {
    /// Workspace-relative path of the manifest.
    pub rel_path: String,
    /// Optional package name from the manifest.
    pub package_name: Option<String>,
    /// Listed runtime dependencies.
    pub dependencies: Vec<String>,
    /// Listed dev dependencies.
    pub dev_dependencies: Vec<String>,
    /// Names of declared scripts.
    pub script_names: Vec<String>,
    /// Pairs of (script name, raw script body).
    pub script_bodies: Vec<ScriptBody>,
    /// Parsed individual commands from script bodies.
    pub script_commands: Vec<G3TsAstroPackageScriptCommand>,
    /// Tool invocations identified within scripts.
    pub script_tool_invocations: Vec<G3TsAstroPackageScriptToolInvocation>,
    /// Parse blockers encountered while parsing scripts.
    pub script_parse_blockers: Vec<G3TsAstroPackageScriptParseBlocker>,
}

/// One parsed command extracted from a `package.json` script body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageScriptCommand {
    /// Name of the script the command belongs to.
    pub script_name: String,
    /// Raw invocation text.
    pub invocation: String,
    /// Executable component of the invocation.
    pub executable: String,
    /// Argument tokens after the executable.
    pub args: Vec<String>,
    /// Separator preceding this command, if any.
    pub preceded_by: Option<G3TsAstroPackageScriptCommandSeparator>,
}

/// A specific tool invocation identified within a script body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageScriptToolInvocation {
    /// Name of the script the invocation belongs to.
    pub script_name: String,
    /// Index of the invocation among parsed commands.
    pub command_index: usize,
    /// Raw invocation text.
    pub invocation: String,
    /// Executable component of the invocation.
    pub executable: String,
    /// Argument tokens after the executable.
    pub args: Vec<String>,
    /// Separator preceding this invocation, if any.
    pub preceded_by: Option<G3TsAstroPackageScriptCommandSeparator>,
    /// Separator following this invocation, if any.
    pub followed_by: Option<G3TsAstroPackageScriptCommandSeparator>,
}

/// Logical separator between two adjacent shell commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsAstroPackageScriptCommandSeparator {
    /// Sequential `&&` separator.
    And,
    /// Alternative `||` separator.
    Or,
}

/// Information about a script body that could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageScriptParseBlocker {
    /// Name of the affected script.
    pub script_name: String,
    /// Human-readable reason for the parse failure.
    pub reason: String,
}

/// State of the `package.json` surface for an Astro app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroPackageSurfaceState {
    /// Manifest file is missing on disk.
    Missing {
        /// Workspace-relative path that was searched.
        rel_path: String,
    },
    /// Manifest file exists but could not be read.
    Unreadable {
        /// Workspace-relative path of the manifest.
        rel_path: String,
        /// Reason the manifest could not be read.
        reason: String,
    },
    /// Manifest file exists but failed to parse.
    ParseError {
        /// Workspace-relative path of the manifest.
        rel_path: String,
        /// Parse error description.
        reason: String,
    },
    /// Manifest parsed successfully into a snapshot.
    Parsed {
        /// Parsed snapshot of the manifest.
        snapshot: G3TsAstroPackageSurfaceSnapshot,
    },
}

/// Snapshot of the Astro MDX policy section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxPolicySnapshot {
    /// Workspace-relative path of the policy file.
    pub rel_path: String,
    /// Optional content root (where MDX content lives).
    pub content_root: Option<String>,
    /// Configured MDX component map module paths.
    pub mdx_component_maps: Vec<String>,
}

/// State of the Astro MDX policy surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroMdxPolicySurfaceState {
    /// Policy file is missing.
    Missing {
        /// Workspace-relative path searched.
        rel_path: String,
    },
    /// Policy file is present but unreadable.
    Unreadable {
        /// Workspace-relative path searched.
        rel_path: String,
        /// Read failure reason.
        reason: String,
    },
    /// Policy file failed to parse.
    ParseError {
        /// Workspace-relative path searched.
        rel_path: String,
        /// Parse failure reason.
        reason: String,
    },
    /// Policy file parsed but Astro section is missing.
    MissingAstroPolicy {
        /// Workspace-relative path of the policy file.
        rel_path: String,
    },
    /// Policy file parsed successfully into a snapshot.
    Parsed {
        /// Parsed policy snapshot.
        snapshot: G3TsAstroMdxPolicySnapshot,
    },
}

/// Approved MDX source paths derived from policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxApprovedSourcePaths {
    /// MDX component map module paths that exist on disk.
    pub mdx_component_maps: Vec<String>,
    /// MDX component map paths declared but missing on disk.
    pub missing_mdx_component_maps: Vec<String>,
}

/// Input describing a missing MDX component map source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxMissingComponentMapInput {
    /// Workspace-relative path of the policy that declared the source.
    pub policy_rel_path: String,
    /// The configured component map path that is missing.
    pub configured_path: String,
}

/// Snapshot of the `ESLint` surface relevant to MDX checks.
#[expect(
    clippy::struct_excessive_bools,
    reason = "Each bool flags an independent ESLint config dimension required by downstream contracts."
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxEslintSurfaceSnapshot {
    /// Workspace-relative path of the `ESLint` config file.
    pub rel_path: String,
    /// Whether the MDX content probe is present.
    pub mdx_content_probe_present: bool,
    /// Plugins active for the MDX content probe.
    pub mdx_content_plugins: Vec<String>,
    /// Plugin-to-package map for the MDX content probe.
    pub mdx_content_plugin_package_names: PluginPackageMap,
    /// Error-severity rules active for the MDX content probe.
    pub mdx_content_error_rules: Vec<String>,
    /// Warn-or-error severity rules active for the MDX content probe.
    pub mdx_content_warn_or_error_rules: Vec<String>,
    /// Restricted disable patterns for the MDX content probe.
    pub mdx_content_restricted_disable_patterns: Vec<String>,
    /// Whether unused-disable directives fail closed for the MDX content probe.
    pub mdx_content_unused_disable_fail_closed: bool,
    /// Effective MDX component map rules at the MDX content probe.
    pub mdx_content_effective_mdx_component_map_rules: Vec<String>,
    /// Effective named-component-import rules at the MDX content probe.
    pub mdx_content_effective_named_component_import_rules: Vec<String>,
    /// Effective no-raw-image rules at the MDX content probe.
    pub mdx_content_effective_no_raw_image_rules: Vec<String>,
    /// Whether the component map probe is present.
    pub component_map_probe_present: bool,
    /// Plugins active for the component map probe.
    pub component_map_plugins: Vec<String>,
    /// Plugin-to-package map for the component map probe.
    pub component_map_plugin_package_names: PluginPackageMap,
    /// Error-severity rules active for the component map probe.
    pub component_map_error_rules: Vec<String>,
    /// Warn-or-error severity rules active for the component map probe.
    pub component_map_warn_or_error_rules: Vec<String>,
    /// Restricted disable patterns for the component map probe.
    pub component_map_restricted_disable_patterns: Vec<String>,
    /// Whether unused-disable directives fail closed for the component map probe.
    pub component_map_unused_disable_fail_closed: bool,
    /// Effective no-raw-ui-export rules at the component map probe.
    pub component_map_effective_no_raw_ui_export_rules: Vec<String>,
    /// Effective wrapper-zod-parse rules at the component map probe.
    pub component_map_effective_wrapper_zod_parse_rules: Vec<String>,
    /// Whether the component map probe is ignored by `ESLint`.
    pub component_map_probe_ignored: bool,
    /// Whether the MDX content probe is ignored by `ESLint`.
    pub mdx_content_probe_ignored: bool,
}

/// State of the `ESLint` surface relevant to MDX checks.
#[expect(
    clippy::large_enum_variant,
    reason = "Boxing the snapshot would force constructor changes across crates that consume this public type; the variant size is acceptable for surface-state values that flow through the configured-checks pipeline."
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroMdxEslintSurfaceState {
    /// `ESLint` config file is missing.
    Missing {
        /// Workspace-relative path searched.
        rel_path: String,
    },
    /// `ESLint` config file is unreadable.
    Unreadable {
        /// Workspace-relative path of the config file.
        rel_path: String,
        /// Read failure reason.
        reason: String,
    },
    /// `ESLint` config file failed to parse.
    ParseError {
        /// Workspace-relative path of the config file.
        rel_path: String,
        /// Parse failure reason.
        reason: String,
    },
    /// `ESLint` config file parsed successfully into a snapshot.
    Parsed {
        /// Parsed snapshot of the `ESLint` surface.
        snapshot: G3TsAstroMdxEslintSurfaceSnapshot,
    },
}

/// Per-app input for the `ESLint` MDX plugin contract check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxEslintPluginContractInput {
    /// Workspace-relative path to the Astro app root.
    pub app_root_rel_path: String,
    /// `ESLint` config surface for the app.
    pub config: G3TsAstroMdxEslintSurfaceState,
}

/// One `ESLint` disable directive encountered in MDX sources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxEslintDirectiveInput {
    /// Workspace-relative path of the file containing the directive.
    pub rel_path: String,
    /// Kind of directive (eslint-disable, eslint-disable-next-line, etc.).
    pub directive_kind: String,
    /// Rules explicitly disabled by the directive.
    pub disabled_rules: Vec<String>,
    /// Whether the directive disables all rules.
    pub all_rules: bool,
    /// Source line of the directive itself.
    pub line: u32,
    /// Source line targeted by the directive, when applicable.
    pub target_line: Option<u32>,
    /// Parse failure reason for malformed directives.
    pub parse_error: Option<String>,
}

/// Per-app input for the integration contract check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxIntegrationContractInput {
    /// Workspace-relative path to the Astro app root.
    pub app_root_rel_path: String,
    /// Approved MDX source paths derived from policy.
    pub mdx_sources: G3TsAstroMdxApprovedSourcePaths,
    /// `package.json` surface state for the app.
    pub package: G3TsAstroPackageSurfaceState,
    /// MDX policy state for the app.
    pub astro_policy: G3TsAstroMdxPolicySurfaceState,
}

/// Aggregated input for all MDX config-checks runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroMdxConfigChecksInput {
    /// Per-app integration contract inputs.
    pub integration_contracts: Vec<G3TsAstroMdxIntegrationContractInput>,
    /// Per-app `ESLint` plugin contract inputs.
    pub eslint_contracts: Vec<G3TsAstroMdxEslintPluginContractInput>,
    /// Inputs for missing component map sources, repo-wide.
    pub missing_component_map_sources: Vec<G3TsAstroMdxMissingComponentMapInput>,
    /// Inputs for `ESLint` directives encountered in MDX sources.
    pub eslint_directives: Vec<G3TsAstroMdxEslintDirectiveInput>,
}
