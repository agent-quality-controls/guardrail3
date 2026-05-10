/// Pair of (script name, script body) extracted from `package.json`.
type ScriptBody = (String, String);

/// Snapshot of the `package.json` surface used by Astro SEO checks.
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
    /// Listed optional dependencies.
    pub optional_dependencies: Vec<String>,
    /// Listed peer dependencies.
    pub peer_dependencies: Vec<String>,
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

/// One parsed command extracted from a script body.
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

/// Tool invocation parsed from a script body.
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
    /// Reason the script body could not be parsed.
    pub reason: String,
}

/// State of the `package.json` surface for an Astro app.
#[expect(
    clippy::large_enum_variant,
    reason = "Boxing the parsed snapshot would force constructor changes across consumer crates outside this types crate."
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroPackageSurfaceState {
    /// Manifest is missing on disk.
    Missing {
        /// Workspace-relative path searched.
        rel_path: String,
    },
    /// Manifest exists but could not be read.
    Unreadable {
        /// Workspace-relative path of the manifest.
        rel_path: String,
        /// Read failure reason.
        reason: String,
    },
    /// Manifest exists but failed to parse.
    ParseError {
        /// Workspace-relative path of the manifest.
        rel_path: String,
        /// Parse failure reason.
        reason: String,
    },
    /// Manifest parsed successfully.
    Parsed {
        /// Parsed snapshot of the manifest.
        snapshot: G3TsAstroPackageSurfaceSnapshot,
    },
}

/// Snapshot of the parsed Astro config surface.
#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroConfigSurfaceSnapshot {
    /// Workspace-relative path of the Astro config file.
    pub rel_path: String,
    /// Configured `site` URL.
    pub site: Option<String>,
    /// Configured `output` mode.
    pub output: Option<G3TsAstroOutputMode>,
    /// Configured `outDir` value.
    pub out_dir: Option<String>,
    /// Configured `trailingSlash` policy.
    pub trailing_slash: Option<G3TsAstroTrailingSlashPolicy>,
    /// List of integrations configured on Astro.
    pub integrations: Vec<G3TsAstroIntegrationSnapshot>,
    /// Optional adapter integration.
    pub adapter: Option<G3TsAstroIntegrationSnapshot>,
}

/// Astro `output` build mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsAstroOutputMode {
    /// Static-site output.
    Static,
    /// Server-rendered output.
    Server,
}

/// Astro `trailingSlash` policy values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsAstroTrailingSlashPolicy {
    /// Always require a trailing slash.
    Always,
    /// Never require a trailing slash.
    Never,
    /// Ignore trailing-slash differences.
    Ignore,
}

/// One integration entry parsed from the Astro config.
#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroIntegrationSnapshot {
    /// Source module the integration was imported from.
    pub source_module: Option<String>,
    /// Local name bound to the integration.
    pub name: Option<String>,
    /// Imported name from the source module.
    pub imported_name: Option<String>,
    /// Captured call expression for the integration, if any.
    pub call: Option<G3TsAstroCallSnapshot>,
}

/// Captured call expression for a configured integration.
#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroCallSnapshot {
    /// Static value passed as the first argument, when extractable.
    pub first_arg: Option<G3TsAstroStaticValue>,
}

/// Static-evaluable value extracted from the Astro config AST.
#[derive(Debug, Clone, PartialEq)]
pub enum G3TsAstroStaticValue {
    /// Boolean literal.
    Bool(bool),
    /// Numeric literal.
    Number(f64),
    /// String literal.
    String(String),
    /// `null` literal.
    Null,
    /// Array literal of static values.
    Array(Vec<Self>),
    /// Object literal of `(key, static value)` properties.
    Object(Vec<G3TsAstroStaticObjectProperty>),
    /// Imported identifier reference.
    ImportedIdentifier {
        /// Local binding name.
        local_name: String,
        /// Source module name, if known.
        source_module: Option<String>,
        /// Imported name from the source module, if known.
        imported_name: Option<String>,
    },
    /// Expression that could not be statically evaluated.
    UnsupportedExpression {
        /// Reason the expression was not evaluable.
        reason: String,
    },
}

/// One key-value property within a static object literal.
#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroStaticObjectProperty {
    /// Property key.
    pub key: String,
    /// Property value.
    pub value: G3TsAstroStaticValue,
}

/// State of the Astro config surface.
#[derive(Debug, Clone, PartialEq)]
pub enum G3TsAstroConfigSurfaceState {
    /// Astro config file is missing.
    Missing {
        /// Workspace-relative path searched.
        rel_path: String,
    },
    /// Astro config file is unreadable.
    Unreadable {
        /// Workspace-relative path of the config file.
        rel_path: String,
        /// Read failure reason.
        reason: String,
    },
    /// Astro config file failed to parse.
    ParseError {
        /// Workspace-relative path of the config file.
        rel_path: String,
        /// Parse failure reason.
        reason: String,
    },
    /// Astro config file parsed successfully.
    Parsed {
        /// Parsed snapshot of the Astro config.
        snapshot: G3TsAstroConfigSurfaceSnapshot,
    },
}

/// Snapshot of the SEO policy section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoPolicySnapshot {
    /// Workspace-relative path of the policy file.
    pub rel_path: String,
    /// Configured metadata helper module paths.
    pub metadata_helpers: Vec<String>,
    /// Configured `JSON-LD` helper module paths.
    pub json_ld_helpers: Vec<String>,
    /// Whether strict AI-readable rules are required.
    pub strict_ai_readable: bool,
    /// Required sections in the LLMs document.
    pub llms_required_sections: Vec<String>,
    /// Required links in the LLMs document.
    pub llms_required_links: Vec<String>,
}

/// State of the SEO policy surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroSeoPolicySurfaceState {
    /// Policy file is missing.
    Missing {
        /// Workspace-relative path searched.
        rel_path: String,
    },
    /// Policy file is present but unreadable.
    Unreadable {
        /// Workspace-relative path of the policy file.
        rel_path: String,
        /// Read failure reason.
        reason: String,
    },
    /// Policy file failed to parse.
    ParseError {
        /// Workspace-relative path of the policy file.
        rel_path: String,
        /// Parse failure reason.
        reason: String,
    },
    /// Policy file parsed but the Astro section is missing.
    MissingAstroPolicy {
        /// Workspace-relative path of the policy file.
        rel_path: String,
    },
    /// Policy file parsed successfully.
    Parsed {
        /// Parsed SEO policy snapshot.
        snapshot: G3TsAstroSeoPolicySnapshot,
    },
}

/// Approved SEO source paths derived from policy.
#[expect(
    clippy::struct_field_names,
    reason = "The `_helpers` postfix names domain concepts that are part of the public field contract and consumed by downstream checks."
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoApprovedSourcePaths {
    /// Metadata helper module paths that exist on disk.
    pub metadata_helpers: Vec<String>,
    /// Metadata helper module paths declared but missing on disk.
    pub missing_metadata_helpers: Vec<String>,
    /// `JSON-LD` helper module paths that exist on disk.
    pub json_ld_helpers: Vec<String>,
    /// `JSON-LD` helper module paths declared but missing on disk.
    pub missing_json_ld_helpers: Vec<String>,
}

/// Input describing a missing metadata helper source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoMissingMetadataHelperInput {
    /// Workspace-relative path of the policy that declared the source.
    pub policy_rel_path: String,
    /// Configured metadata helper path that is missing.
    pub configured_path: String,
}

/// Input describing a missing `JSON-LD` helper source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoMissingJsonLdHelperInput {
    /// Workspace-relative path of the policy that declared the source.
    pub policy_rel_path: String,
    /// Configured `JSON-LD` helper path that is missing.
    pub configured_path: String,
}

/// Snapshot of the `ESLint` surface relevant to SEO checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoEslintSurfaceSnapshot {
    /// Workspace-relative path of the `ESLint` config file.
    pub rel_path: String,
    /// Whether the Astro source probe is present.
    pub astro_source_probe_present: bool,
    /// Whether the TypeScript source probe is present.
    pub ts_source_probe_present: bool,
    /// Whether the TSX source probe is present.
    pub tsx_source_probe_present: bool,
    /// Effective metadata-helper rules at the Astro source probe.
    pub astro_source_effective_metadata_helper_rules: Vec<String>,
    /// Effective metadata-helper rules at the TS source probe.
    pub ts_source_effective_metadata_helper_rules: Vec<String>,
    /// Effective metadata-helper rules at the TSX source probe.
    pub tsx_source_effective_metadata_helper_rules: Vec<String>,
    /// Effective `JSON-LD` helper rules at the Astro source probe.
    pub astro_source_effective_json_ld_helper_rules: Vec<String>,
    /// Effective `JSON-LD` helper rules at the TS source probe.
    pub ts_source_effective_json_ld_helper_rules: Vec<String>,
    /// Effective `JSON-LD` helper rules at the TSX source probe.
    pub tsx_source_effective_json_ld_helper_rules: Vec<String>,
    /// Warn-or-error rules at the Astro source probe.
    pub astro_source_warn_or_error_rules: Vec<String>,
    /// Warn-or-error rules at the TS source probe.
    pub ts_source_warn_or_error_rules: Vec<String>,
    /// Warn-or-error rules at the TSX source probe.
    pub tsx_source_warn_or_error_rules: Vec<String>,
    /// Restricted disable patterns at the Astro source probe.
    pub astro_source_restricted_disable_patterns: Vec<String>,
    /// Restricted disable patterns at the TS source probe.
    pub ts_source_restricted_disable_patterns: Vec<String>,
    /// Restricted disable patterns at the TSX source probe.
    pub tsx_source_restricted_disable_patterns: Vec<String>,
}

/// State of the `ESLint` surface relevant to SEO checks.
#[expect(
    clippy::large_enum_variant,
    reason = "Boxing the parsed snapshot would force constructor changes across consumer crates outside this types crate."
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsAstroSeoEslintSurfaceState {
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
    /// `ESLint` config file parsed successfully.
    Parsed {
        /// Parsed snapshot of the `ESLint` surface.
        snapshot: G3TsAstroSeoEslintSurfaceSnapshot,
    },
}

/// Per-app input for the `ESLint` SEO plugin contract check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoEslintPluginContractInput {
    /// Workspace-relative path to the Astro app root.
    pub app_root_rel_path: String,
    /// `ESLint` surface state for the app.
    pub config: G3TsAstroSeoEslintSurfaceState,
}

/// One `ESLint` disable directive observed in SEO sources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsAstroSeoEslintDirectiveInput {
    /// Workspace-relative path of the file containing the directive.
    rel_path: String,
    /// Kind of directive (eslint-disable, eslint-disable-next-line, etc.).
    directive_kind: String,
    /// Rules explicitly disabled by the directive.
    disabled_rules: Vec<String>,
    /// Whether the directive disables all rules.
    all_rules: bool,
    /// Source line of the directive.
    line: u32,
    /// Source line targeted by the directive, when applicable.
    target_line: Option<u32>,
    /// Parse failure reason for malformed directives.
    parse_error: Option<String>,
}

impl G3TsAstroSeoEslintDirectiveInput {
    /// Builds a new directive input.
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

    /// Workspace-relative path of the file containing the directive.
    #[must_use]
    pub fn rel_path(&self) -> &str {
        self.rel_path.as_str()
    }

    /// Kind of directive (e.g. `eslint-disable`).
    #[must_use]
    pub fn directive_kind(&self) -> &str {
        &self.directive_kind
    }

    /// Rules explicitly disabled by the directive.
    #[must_use]
    pub fn disabled_rules(&self) -> &[String] {
        &self.disabled_rules
    }

    /// Whether the directive disables all rules.
    #[must_use]
    pub const fn all_rules(&self) -> bool {
        self.all_rules
    }

    /// Source line of the directive.
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// Source line targeted by the directive, when applicable.
    #[must_use]
    pub const fn target_line(&self) -> Option<u32> {
        self.target_line
    }

    /// Parse failure reason for malformed directives.
    #[must_use]
    pub fn parse_error(&self) -> Option<&str> {
        self.parse_error.as_deref()
    }
}

/// Per-app input for the integration contract check.
#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroSeoIntegrationContractInput {
    /// Workspace-relative path to the Astro app root.
    pub app_root_rel_path: String,
    /// Approved SEO source paths.
    pub seo_sources: G3TsAstroSeoApprovedSourcePaths,
    /// `package.json` surface state.
    pub package: G3TsAstroPackageSurfaceState,
    /// Astro config surface state.
    pub astro_config: G3TsAstroConfigSurfaceState,
    /// SEO policy surface state.
    pub astro_policy: G3TsAstroSeoPolicySurfaceState,
}

/// Aggregated input for all SEO config-checks runs.
#[derive(Debug, Clone, PartialEq)]
pub struct G3TsAstroSeoConfigChecksInput {
    /// Per-app integration contract inputs.
    pub integration_contracts: Vec<G3TsAstroSeoIntegrationContractInput>,
    /// Per-app `ESLint` plugin contract inputs.
    pub eslint_contracts: Vec<G3TsAstroSeoEslintPluginContractInput>,
    /// Inputs for missing metadata helper sources.
    pub missing_metadata_helper_sources: Vec<G3TsAstroSeoMissingMetadataHelperInput>,
    /// Inputs for missing `JSON-LD` helper sources.
    pub missing_json_ld_helper_sources: Vec<G3TsAstroSeoMissingJsonLdHelperInput>,
    /// Inputs for `ESLint` directives encountered in SEO sources.
    pub eslint_directives: Vec<G3TsAstroSeoEslintDirectiveInput>,
}
