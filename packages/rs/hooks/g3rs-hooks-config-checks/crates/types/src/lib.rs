#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksSelectedHookConfigFact {
    pub rel_path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsHooksConfigChecksInput {
    pub selected_hook: Option<G3RsHooksSelectedHookConfigFact>,
    pub installed_tools: Vec<String>,
}
