use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStylePolicySnapshot {
    pub rel_path: String,
    pub source_globs: Vec<String>,
    pub stylelint_css_globs: Vec<String>,
    pub extra_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsStylePolicySurfaceState {
    Missing { rel_path: String },
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    MissingTsPolicy { rel_path: String },
    MissingStylePolicy { rel_path: String },
    Parsed { snapshot: G3TsStylePolicySnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStylePackageSurfaceSnapshot {
    pub rel_path: String,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub script_names: Vec<String>,
    pub script_tool_invocations: Vec<G3TsStylePackageScriptToolInvocation>,
    pub script_parse_blockers: Vec<G3TsStylePackageScriptParseBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStylePackageScriptToolInvocation {
    pub script_name: String,
    pub executable: String,
    pub args: Vec<String>,
    pub preceded_by: Option<G3TsStylePackageScriptCommandSeparator>,
    pub followed_by: Option<G3TsStylePackageScriptCommandSeparator>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsStylePackageScriptCommandSeparator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStylePackageScriptParseBlocker {
    pub script_name: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsStylePackageSurfaceState {
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
        snapshot: G3TsStylePackageSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStylelintConfigSnapshot {
    pub rel_path: String,
    pub raw_extends: Vec<String>,
    pub raw_plugins: Vec<String>,
    pub resolved_extends: Vec<String>,
    pub resolved_plugins: Vec<String>,
    pub resolved_rule_names: Vec<String>,
    pub probe_present: bool,
    pub probe_ignored: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsStylelintConfigSurfaceState {
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
        snapshot: G3TsStylelintConfigSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Snapshot mirrors the discrete ESLint-config surface flags consumed by style \
              checks; each boolean models a separately observable rule state that must remain \
              independently inspectable"
)]
pub struct G3TsStyleEslintSurfaceSnapshot {
    pub rel_path: String,
    pub source_probe_present: bool,
    pub source_probe_ignored: bool,
    pub source_plugins: Vec<String>,
    #[expect(
        clippy::type_complexity,
        reason = "BTreeMap<String, Vec<String>> models package-name -> rule-list lookups; \
                  introducing a named alias here would not add clarity beyond the field name"
    )]
    pub source_plugin_package_names: BTreeMap<String, Vec<String>>,
    pub style_policy_plugin_effective: bool,
    pub style_policy_rule_effective: bool,
    pub source_warn_or_error_rules: Vec<String>,
    pub source_restricted_disable_patterns: Vec<String>,
    pub source_probe_disable_policies: Vec<G3TsStyleEslintProbeDisablePolicySnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleEslintProbeDisablePolicySnapshot {
    pub rel_path: String,
    pub ignored: bool,
    pub warn_or_error_rules: Vec<String>,
    pub restricted_disable_patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsStyleEslintSurfaceState {
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
        snapshot: G3TsStyleEslintSurfaceSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleEslintDirectiveInput {
    pub rel_path: String,
    pub directive_kind: String,
    pub disabled_rules: Vec<String>,
    pub all_rules: bool,
    pub line: u32,
    pub target_line: Option<u32>,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleSyncpackRequiredPin {
    pub dependency: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleSyncpackSnapshot {
    pub rel_path: String,
    pub source: Vec<String>,
    pub version_groups: Vec<G3TsStyleSyncpackVersionGroupSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleSyncpackVersionGroupSnapshot {
    pub dependencies: Vec<String>,
    pub dependency_types: Vec<String>,
    pub packages: Option<Vec<String>>,
    pub specifier_types: Option<Vec<String>>,
    pub is_ignored: Option<bool>,
    pub is_banned: Option<bool>,
    pub pin_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsStyleSyncpackSurfaceState {
    Missing { rel_path: String },
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsStyleSyncpackSnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleContractInput {
    pub app_root_rel_path: String,
    pub policy: G3TsStylePolicySurfaceState,
    pub package: G3TsStylePackageSurfaceState,
    pub stylelint_config: G3TsStylelintConfigSurfaceState,
    pub eslint_config: G3TsStyleEslintSurfaceState,
    pub syncpack_config: G3TsStyleSyncpackSurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleConfigChecksInput {
    pub contracts: Vec<G3TsStyleContractInput>,
    pub eslint_directives: Vec<G3TsStyleEslintDirectiveInput>,
}
