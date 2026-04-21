#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsJscpdRootSnapshot {
    pub rel_path: String,
    pub threshold: Option<i64>,
    pub min_tokens: Option<u64>,
    pub absolute: Option<bool>,
    pub format: Vec<String>,
    pub ignore: Vec<String>,
    pub extra_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsJscpdRootState {
    Missing,
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsJscpdRootSnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsJscpdChecksInput {
    pub root: G3TsJscpdRootState,
}
