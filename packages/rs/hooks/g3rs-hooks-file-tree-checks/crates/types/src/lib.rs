#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksScriptFileFact {
    pub rel_path: String,
    pub line_count: usize,
    pub byte_count: usize,
    pub executable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksFileTreeChecksInput {
    pub pre_commit: Option<G3RsHooksScriptFileFact>,
    pub has_modular_dir: bool,
    pub modular_scripts: Vec<G3RsHooksScriptFileFact>,
    pub local_override_scripts: Vec<String>,
    pub hooks_path: Option<String>,
    pub trust_risks: Vec<String>,
}
