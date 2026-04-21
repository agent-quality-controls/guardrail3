#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmrcDocument {
    pub raw: String,
    pub typed: NpmrcParseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NpmrcParseState {
    Parsed(NpmrcSnapshot),
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmrcSetting {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NpmrcSnapshot {
    pub settings: Vec<NpmrcSetting>,
    pub duplicate_keys: Vec<String>,
}
