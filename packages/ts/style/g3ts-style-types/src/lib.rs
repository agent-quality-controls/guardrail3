use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStylePolicySnapshot {
    pub rel_path: String,
    pub source_globs: Vec<String>,
    pub tailwind_denylist: Vec<String>,
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
    pub tailwind_rule_effective: bool,
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
pub struct G3TsStyleContractInput {
    pub app_root_rel_path: String,
    pub policy: G3TsStylePolicySurfaceState,
    pub package: G3TsStylePackageSurfaceState,
    pub stylelint_config: G3TsStylelintConfigSurfaceState,
    pub eslint_config: G3TsStyleEslintSurfaceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsStyleConfigChecksInput {
    pub contracts: Vec<G3TsStyleContractInput>,
}
