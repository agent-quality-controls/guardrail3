use package_json_parser_types::document::{
    PackageJsonBoolFieldState, PackageJsonDocument, PackageJsonParseState, PackageJsonSnapshot,
};

pub fn typed(document: &PackageJsonDocument) -> Option<&PackageJsonSnapshot> {
    match &document.typed {
        PackageJsonParseState::Parsed(snapshot) => Some(snapshot),
        PackageJsonParseState::Invalid(_) => None,
    }
}

pub fn parse_error_reason(document: &PackageJsonDocument) -> Option<&str> {
    match &document.typed {
        PackageJsonParseState::Parsed(_) => None,
        PackageJsonParseState::Invalid(reason) => Some(reason),
    }
}

pub fn bool_field_state<'a>(
    document: &'a PackageJsonDocument,
    field: &str,
) -> PackageJsonBoolFieldState<'a> {
    let Some(value) = document.raw.get(field) else {
        return PackageJsonBoolFieldState::Missing;
    };
    match value.as_bool() {
        Some(flag) => PackageJsonBoolFieldState::Value(flag),
        None => PackageJsonBoolFieldState::WrongType(value),
    }
}
