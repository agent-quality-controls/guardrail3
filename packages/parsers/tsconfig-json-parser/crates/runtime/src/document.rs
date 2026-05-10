use tsconfig_json_parser_types::document::{
    TsconfigBoolFieldState, TsconfigCompilerOptions, TsconfigDocument, TsconfigParseState,
    TsconfigSnapshot,
};

#[must_use]
pub const fn typed(document: &TsconfigDocument) -> Option<&TsconfigSnapshot> {
    match &document.typed {
        TsconfigParseState::Parsed(snapshot) => Some(snapshot),
        TsconfigParseState::Invalid(_) => None,
    }
}

#[must_use]
pub fn parse_error_reason(document: &TsconfigDocument) -> Option<&str> {
    match &document.typed {
        TsconfigParseState::Parsed(_) => None,
        TsconfigParseState::Invalid(reason) => Some(reason),
    }
}

#[must_use]
pub fn extends_entries(document: &TsconfigDocument) -> &[String] {
    typed(document).map_or(&[], |snapshot| snapshot.extends.as_slice())
}

#[must_use]
pub fn compiler_options(document: &TsconfigDocument) -> Option<&TsconfigCompilerOptions> {
    typed(document).map(|snapshot| &snapshot.compiler_options)
}

#[must_use]
pub fn bool_field_state<'a>(
    document: &'a TsconfigDocument,
    field: &str,
) -> TsconfigBoolFieldState<'a> {
    let Some(raw_compiler_options) = document.raw.get("compilerOptions") else {
        return TsconfigBoolFieldState::Missing;
    };
    let Some(value) = raw_compiler_options.get(field) else {
        return TsconfigBoolFieldState::Missing;
    };
    value.as_bool().map_or_else(
        || TsconfigBoolFieldState::WrongType(value),
        TsconfigBoolFieldState::Value,
    )
}
