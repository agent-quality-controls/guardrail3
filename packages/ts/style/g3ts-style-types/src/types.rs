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
pub struct G3TsStyleEslintSurfaceSnapshot {
    pub rel_path: String,
    pub source_probe_present: bool,
    pub source_probe_ignored: bool,
    pub source_plugins: Vec<String>,
    pub source_plugin_package_names: BTreeMap<String, Vec<String>>,
    pub style_policy_plugin_effective: bool,
    pub style_policy_rule_effective: bool,
    pub source_warn_or_error_rules: Vec<String>,
    pub source_restricted_disable_patterns: Vec<String>,
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
    rel_path: String,
    directive_kind: String,
    disabled_rules: Vec<String>,
    all_rules: bool,
    line: u32,
    target_line: Option<u32>,
    parse_error: Option<String>,
}

impl G3TsStyleEslintDirectiveInput {
    #[must_use]
    pub fn parsed(
        rel_path: String,
        directive_kind: String,
        disabled_rules: Vec<String>,
        all_rules: bool,
        line: u32,
        target_line: Option<u32>,
    ) -> Self {
        Self {
            rel_path,
            directive_kind,
            disabled_rules,
            all_rules,
            line,
            target_line,
            parse_error: None,
        }
    }

    #[must_use]
    pub fn parse_error(rel_path: String, reason: String) -> Self {
        Self {
            rel_path,
            directive_kind: String::new(),
            disabled_rules: Vec::new(),
            all_rules: false,
            line: 0,
            target_line: None,
            parse_error: Some(reason),
        }
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub fn directive_kind(&self) -> &str {
        &self.directive_kind
    }

    #[must_use]
    pub fn disabled_rules(&self) -> &[String] {
        &self.disabled_rules
    }

    #[must_use]
    pub fn all_rules(&self) -> bool {
        self.all_rules
    }

    #[must_use]
    pub fn line(&self) -> u32 {
        self.line
    }

    #[must_use]
    pub fn target_line(&self) -> Option<u32> {
        self.target_line
    }

    #[must_use]
    pub fn parse_error_reason(&self) -> Option<&str> {
        self.parse_error.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleSyncpackRequiredPin {
    pub dependency: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleSyncpackSnapshot {
    pub rel_path: String,
    pub source_covers_package_manifest: bool,
    pub missing_required_pins: Vec<G3TsStyleSyncpackRequiredPin>,
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
