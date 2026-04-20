use eslint_config_parser::types::EslintConfigDocument;

#[derive(Debug, Clone, PartialEq)]
pub enum G3TsEslintConfigState {
    Missing,
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        rel_path: String,
        document: EslintConfigDocument,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsEslintConfigChecksInput {
    pub config: G3TsEslintConfigState,
}
