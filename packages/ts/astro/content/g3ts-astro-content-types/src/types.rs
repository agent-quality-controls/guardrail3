use std::collections::BTreeMap;

/// Pair of `(script_name, raw_script_body)` extracted from `package.json` scripts.
pub type G3TsAstroPackageScriptBody = (String, String);

/// Mapping of collection name to the field names defined for that collection.
pub type G3TsAstroContentCollectionFields = BTreeMap<String, Vec<String>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPackageSurfaceSnapshot {
    pub rel_path: String,
    pub package_name: Option<String>,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub script_names: Vec<String>,
    pub script_bodies: Vec<G3TsAstroPackageScriptBody>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsAstroContentMode {
    None,
    BuildCollections,
    LiveCollections,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::struct_field_names,
    reason = "Each `*_rel_path` field names a distinct config artifact relative to the app \
              root; renaming would obscure the artifact each path refers to"
)]
pub struct G3TsAstroContentAppRootInput {
    pub app_root_rel_path: String,
    pub content_config_rel_path: Option<String>,
    pub live_config_rel_path: Option<String>,
    pub velite_config_rel_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroRouteMarkdownPageInput {
    pub rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentVeliteOutputInput {
    pub app_root_rel_path: String,
    pub rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentPolicySnapshot {
    pub rel_path: String,
    pub profile: Option<String>,
    pub content_routes: Vec<String>,
    pub non_content_routes: Vec<String>,
    pub endpoints: Vec<String>,
    pub content_root: Option<String>,
    pub content_adapters: Vec<String>,
    pub required_collections: Vec<String>,
    pub collection_fields: G3TsAstroContentCollectionFields,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroContentPolicySurfaceState {
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
        snapshot: G3TsAstroContentPolicySnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentAdapterSourcePaths {
    pub content_adapter: Vec<String>,
    pub content_adapter_astro_content: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroPipelineRuleScopeSnapshot {
    pub rule_name: String,
    pub route_globs: Vec<String>,
    pub endpoint_globs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentAdapterRootInput {
    pub policy_rel_path: String,
    pub configured_adapter: String,
    pub source_exists: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentAdapterSourceInput {
    pub policy_rel_path: String,
    pub source_rel_path: String,
    pub imports_astro_content: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Each bool corresponds to a distinct named ESLint probe (astro/ts/tsx \
              source presence and ignored state); merging into bitflags would obscure \
              the field-level surface consumed by checks across multiple crates"
)]
pub struct G3TsAstroContentEslintSurfaceSnapshot {
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
    pub astro_source_effective_content_adapter_modules: Vec<String>,
    pub ts_source_effective_content_adapter_modules: Vec<String>,
    pub tsx_source_effective_content_adapter_modules: Vec<String>,
    pub astro_source_route_scoped_pipeline_rule_scopes: Vec<G3TsAstroPipelineRuleScopeSnapshot>,
    pub ts_source_route_scoped_pipeline_rule_scopes: Vec<G3TsAstroPipelineRuleScopeSnapshot>,
    pub tsx_source_route_scoped_pipeline_rule_scopes: Vec<G3TsAstroPipelineRuleScopeSnapshot>,
    pub astro_source_effective_inline_public_content_rules: Vec<String>,
    pub ts_source_effective_inline_public_content_rules: Vec<String>,
    pub tsx_source_effective_inline_public_content_rules: Vec<String>,
    pub astro_source_warn_or_error_rules: Vec<String>,
    pub ts_source_warn_or_error_rules: Vec<String>,
    pub tsx_source_warn_or_error_rules: Vec<String>,
    pub astro_source_restricted_disable_patterns: Vec<String>,
    pub ts_source_restricted_disable_patterns: Vec<String>,
    pub tsx_source_restricted_disable_patterns: Vec<String>,
    pub astro_source_probe_ignored: bool,
    pub ts_source_probe_ignored: bool,
    pub tsx_source_probe_ignored: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::large_enum_variant,
    reason = "Parsed snapshot is the dominant runtime variant; boxing would force \
              construction-site changes across consumer crates outside this workspace"
)]
pub enum G3TsAstroContentEslintSurfaceState {
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
        snapshot: G3TsAstroContentEslintSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentEslintPluginContractInput {
    pub app_root_rel_path: String,
    pub config: G3TsAstroContentEslintSurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentEslintDirectiveInput {
    /// Relative path of the source file containing the directive.
    rel_path: String,
    /// Disable directive kind, for example `disable`, `disable-next-line`, or
    /// `disable-line`.
    directive_kind: String,
    /// Names of the rules disabled by this directive; empty when `all_rules`
    /// is true.
    disabled_rules: Vec<String>,
    /// True when the directive disables all rules (no rule list specified).
    all_rules: bool,
    /// 1-based line number where the directive comment appears.
    line: u32,
    /// 1-based line number the directive targets, when applicable.
    target_line: Option<u32>,
    /// Parse error encountered while scanning the directive, if any.
    parse_error: Option<String>,
}

impl G3TsAstroContentEslintDirectiveInput {
    #[must_use]
    pub const fn new(
        rel_path: String,
        directive_kind: String,
        disabled_rules: Vec<String>,
        all_rules: bool,
        line: u32,
        target_line: Option<u32>,
        parse_error: Option<String>,
    ) -> Self {
        Self {
            rel_path,
            directive_kind,
            disabled_rules,
            all_rules,
            line,
            target_line,
            parse_error,
        }
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub fn directive_kind(&self) -> &str {
        self.directive_kind.as_str()
    }

    #[must_use]
    pub fn disabled_rules(&self) -> &[String] {
        &self.disabled_rules
    }

    #[must_use]
    pub const fn all_rules(&self) -> bool {
        self.all_rules
    }

    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    #[must_use]
    pub const fn target_line(&self) -> Option<u32> {
        self.target_line
    }

    #[must_use]
    pub fn parse_error(&self) -> Option<&str> {
        self.parse_error.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentPolicyEslintContractInput {
    pub app_root_rel_path: String,
    pub route_page_paths: Vec<String>,
    pub endpoint_paths: Vec<String>,
    pub astro_policy: G3TsAstroContentPolicySurfaceState,
    pub eslint_config: G3TsAstroContentEslintSurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentIntegrationContractInput {
    pub app_root_rel_path: String,
    pub route_page_paths: Vec<String>,
    pub endpoint_paths: Vec<String>,
    pub content_adapter_sources: G3TsAstroContentAdapterSourcePaths,
    pub package: G3TsAstroPackageSurfaceState,
    pub astro_policy: G3TsAstroContentPolicySurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentConfigChecksInput {
    pub integration_contracts: Vec<G3TsAstroContentIntegrationContractInput>,
    pub eslint_contracts: Vec<G3TsAstroContentEslintPluginContractInput>,
    pub policy_eslint_contracts: Vec<G3TsAstroContentPolicyEslintContractInput>,
    pub eslint_directives: Vec<G3TsAstroContentEslintDirectiveInput>,
    pub adapter_root_contracts: Vec<G3TsAstroContentAdapterRootInput>,
    pub adapter_source_contracts: Vec<G3TsAstroContentAdapterSourceInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroContentFileTreeChecksInput {
    pub app_roots: Vec<G3TsAstroContentAppRootInput>,
    pub build_collection_roots: Vec<G3TsAstroContentAppRootInput>,
    pub live_collection_roots: Vec<G3TsAstroContentAppRootInput>,
    pub route_markdown_pages: Vec<G3TsAstroRouteMarkdownPageInput>,
    pub velite_output_paths: Vec<G3TsAstroContentVeliteOutputInput>,
}
