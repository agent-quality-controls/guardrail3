use g3ts_hooks_contract_types::G3TsHookRequirement;
use hook_shell_parser::types::ParsedShellScript;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksSelectedHookConfigFact {
    pub rel_path: String,
    pub parsed: ParsedShellScript,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksConfigChecksInput {
    pub active: bool,
    pub selected_hook: Option<G3TsHooksSelectedHookConfigFact>,
    pub installed_tools: Vec<String>,
    pub requirements: Vec<G3TsHookRequirement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G3TsHookScriptKind {
    PreCommit,
    Modular,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksSourceChecksInput {
    pub rel_path: String,
    pub kind: G3TsHookScriptKind,
    pub parsed: ParsedShellScript,
    pub has_modular_dir: bool,
    pub app_package_roots: Vec<String>,
    pub requirements: Vec<G3TsHookRequirement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksScriptFileFact {
    pub rel_path: String,
    pub line_count: usize,
    pub byte_count: usize,
    pub executable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksFileTreeChecksInput {
    pub active: bool,
    pub pre_commit: Option<G3TsHooksScriptFileFact>,
    pub has_modular_dir: bool,
    pub modular_scripts: Vec<G3TsHooksScriptFileFact>,
    pub local_override_scripts: Vec<String>,
    pub hooks_path: Option<String>,
    pub trust_risks: Vec<String>,
}
