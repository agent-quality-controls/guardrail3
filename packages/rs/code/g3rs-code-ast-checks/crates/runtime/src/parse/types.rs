#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ForbiddenMacroInfo {
    pub(crate) line: usize,
    pub(crate) macro_name: String,
    pub(crate) in_test_context: bool,
}
