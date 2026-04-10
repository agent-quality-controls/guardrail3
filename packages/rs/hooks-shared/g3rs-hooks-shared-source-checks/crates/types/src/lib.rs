#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G3RsHookScriptKind {
    PreCommit,
    Modular,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksSharedSourceChecksInput {
    pub rel_path: String,
    pub kind: G3RsHookScriptKind,
    pub content: String,
    pub has_modular_dir: bool,
}
