use tsconfig_json_parser::types::TsconfigCompilerOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G3TsTsconfigBoolState {
    Missing,
    Value(bool),
    WrongType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTsconfigInlineStrictFlags {
    pub strict: G3TsTsconfigBoolState,
    pub no_implicit_returns: G3TsTsconfigBoolState,
    pub no_unused_locals: G3TsTsconfigBoolState,
    pub no_unused_parameters: G3TsTsconfigBoolState,
    pub no_unchecked_indexed_access: G3TsTsconfigBoolState,
    pub exact_optional_property_types: G3TsTsconfigBoolState,
    pub no_property_access_from_index_signature: G3TsTsconfigBoolState,
    pub no_implicit_override: G3TsTsconfigBoolState,
    pub no_fallthrough_cases_in_switch: G3TsTsconfigBoolState,
    pub force_consistent_casing_in_file_names: G3TsTsconfigBoolState,
    pub allow_unreachable_code: G3TsTsconfigBoolState,
    pub allow_unused_labels: G3TsTsconfigBoolState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Resolved {
        specifier: String,
        display_path: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
        uses_extends: bool,
        extends_chain: Vec<G3TsTsconfigExtendsState>,
        inline_strict_flags: G3TsTsconfigInlineStrictFlags,
        effective_compiler_options: TsconfigCompilerOptions,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsTsconfigChecksInput {
    pub config: G3TsTsconfigState,
}
