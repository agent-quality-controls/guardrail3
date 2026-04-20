use tsconfig_json_parser::types::{TsconfigCompilerOptions, TsconfigDocument};

#[derive(Debug, Clone, PartialEq)]
pub enum G3TsTsconfigExtendsState {
    External {
        specifier: String,
    },
    Missing {
        specifier: String,
        display_path: String,
    },
    Unreadable {
        specifier: String,
        display_path: String,
        reason: String,
    },
    ParseError {
        specifier: String,
        display_path: String,
        reason: String,
    },
    Parsed {
        specifier: String,
        display_path: String,
        document: TsconfigDocument,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum G3TsTsconfigState {
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
        document: TsconfigDocument,
        extends_chain: Vec<G3TsTsconfigExtendsState>,
        effective_compiler_options: TsconfigCompilerOptions,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct G3TsTsconfigChecksInput {
    pub config: G3TsTsconfigState,
}
