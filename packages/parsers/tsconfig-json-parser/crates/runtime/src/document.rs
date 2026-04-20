use tsconfig_json_parser_types::document::{
    TsconfigBoolFieldState, TsconfigCompilerOptions, TsconfigDocument, TsconfigParseState,
    TsconfigSnapshot,
};

pub fn typed(document: &TsconfigDocument) -> Option<&TsconfigSnapshot> {
    match &document.typed {
        TsconfigParseState::Parsed(snapshot) => Some(snapshot),
        TsconfigParseState::Invalid(_) => None,
    }
}

pub fn parse_error_reason(document: &TsconfigDocument) -> Option<&str> {
    match &document.typed {
        TsconfigParseState::Parsed(_) => None,
        TsconfigParseState::Invalid(reason) => Some(reason),
    }
}

pub fn extends_entries(document: &TsconfigDocument) -> &[String] {
    typed(document)
        .map(|snapshot| snapshot.extends.as_slice())
        .unwrap_or(&[])
}

pub fn compiler_options(document: &TsconfigDocument) -> Option<&TsconfigCompilerOptions> {
    typed(document).map(|snapshot| &snapshot.compiler_options)
}

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
    match value.as_bool() {
        Some(flag) => TsconfigBoolFieldState::Value(flag),
        None => TsconfigBoolFieldState::WrongType(value),
    }
}
