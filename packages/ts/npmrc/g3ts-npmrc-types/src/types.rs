#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsNpmrcSetting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsNpmrcRootSnapshot {
    pub rel_path: String,
    pub settings: Vec<G3TsNpmrcSetting>,
    pub duplicate_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsNpmrcRootState {
    NotPackageManagerRoot,
    Missing,
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsNpmrcRootSnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsNpmrcChecksInput {
    pub root: G3TsNpmrcRootState,
}
