#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HookScriptKind {
    PreCommit,
    Modular,
}
