use hook_shell_parser::ParsedShellScript;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksSelectedHookConfigFact {
    pub rel_path: String,
    pub parsed: ParsedShellScript,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksConfigChecksInput {
    pub active: bool,
    pub selected_hook: Option<G3RsHooksSelectedHookConfigFact>,
    pub installed_tools: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G3RsHookScriptKind {
    PreCommit,
    Modular,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksSourceChecksInput {
    pub rel_path: String,
    pub kind: G3RsHookScriptKind,
    pub parsed: ParsedShellScript,
    pub has_modular_dir: bool,
    pub is_workspace_project: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksScriptFileFact {
    pub rel_path: String,
    pub line_count: usize,
    pub byte_count: usize,
    pub executable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksFileTreeChecksInput {
    pub active: bool,
    pub pre_commit: Option<G3RsHooksScriptFileFact>,
    pub has_modular_dir: bool,
    pub modular_scripts: Vec<G3RsHooksScriptFileFact>,
    pub local_override_scripts: Vec<String>,
    pub hooks_path: Option<String>,
    pub trust_risks: Vec<String>,
}
