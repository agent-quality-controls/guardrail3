use cargo_toml_parser_types::cargo_toml::{CargoToml, LintValue, ToolLints};
use cargo_toml_parser_types::document::{
    CargoBoolFieldState, CargoLintTableState, CargoStringFieldState, CargoTomlDocument,
    CargoTomlDocumentKind, CargoTomlParseState,
};
use toml::Value;

#[must_use]
pub fn typed(document: &CargoTomlDocument) -> Option<&CargoToml> {
    match &document.typed {
        CargoTomlParseState::Parsed(cargo) => Some(cargo.as_ref()),
        CargoTomlParseState::Invalid(_) => None,
    }
}

#[must_use]
pub fn parse_error_reason(document: &CargoTomlDocument) -> Option<&str> {
    match &document.typed {
        CargoTomlParseState::Parsed(_) => None,
        CargoTomlParseState::Invalid(reason) => Some(reason),
    }
}

#[must_use]
pub fn kind(document: &CargoTomlDocument) -> CargoTomlDocumentKind {
    if document.raw.get("workspace").is_some() {
        CargoTomlDocumentKind::WorkspaceRoot
    } else if document.raw.get("package").is_some() {
        CargoTomlDocumentKind::PackageRoot
    } else {
        CargoTomlDocumentKind::Other
    }
}

pub fn package_name(document: &CargoTomlDocument) -> Option<&str> {
    document
        .raw
        .get("package")
        .and_then(|value| value.get("name"))
        .and_then(Value::as_str)
}

#[must_use]
pub fn root_package_string_field<'a>(
    document: &'a CargoTomlDocument,
    field: &str,
) -> CargoStringFieldState<'a> {
    if kind(document) == CargoTomlDocumentKind::WorkspaceRoot {
        let workspace_package = string_field(
            document
                .raw
                .get("workspace")
                .and_then(|value| value.get("package")),
            field,
        );
        if matches!(workspace_package, CargoStringFieldState::Missing) {
            package_string_field(document, field)
        } else {
            workspace_package
        }
    } else {
        package_string_field(document, field)
    }
}

#[must_use]
pub fn package_string_field<'a>(
    document: &'a CargoTomlDocument,
    field: &str,
) -> CargoStringFieldState<'a> {
    string_field(document.raw.get("package"), field)
}

#[must_use]
pub fn lints_workspace_state(document: &CargoTomlDocument) -> CargoBoolFieldState<'_> {
    document
        .raw
        .get("lints")
        .and_then(|value| value.get("workspace"))
        .map_or(CargoBoolFieldState::Missing, |value| {
            value.as_bool().map_or(
                CargoBoolFieldState::WrongType(value),
                CargoBoolFieldState::Value,
            )
        })
}

#[must_use]
pub fn policy_lints<'a>(document: &'a CargoTomlDocument, family: &str) -> Option<&'a ToolLints> {
    let cargo = typed(document)?;
    match kind(document) {
        CargoTomlDocumentKind::WorkspaceRoot => cargo
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.lints.as_ref())
            .and_then(|lints| lints.tools.get(family))
            .or_else(|| {
                cargo
                    .lints
                    .as_ref()
                    .and_then(|lints| lints.tools.get(family))
            }),
        CargoTomlDocumentKind::PackageRoot => cargo
            .lints
            .as_ref()
            .and_then(|lints| lints.tools.get(family)),
        CargoTomlDocumentKind::Other => None,
    }
}

#[must_use]
pub fn policy_lints_state<'a>(
    document: &'a CargoTomlDocument,
    family: &str,
) -> CargoLintTableState<'a> {
    let raw_lints = match kind(document) {
        CargoTomlDocumentKind::WorkspaceRoot => document
            .raw
            .get("workspace")
            .and_then(|value| value.get("lints"))
            .or_else(|| document.raw.get("lints")),
        CargoTomlDocumentKind::PackageRoot => document.raw.get("lints"),
        CargoTomlDocumentKind::Other => None,
    };
    lint_table_state(raw_lints, family, policy_lints(document, family))
}

#[must_use]
pub fn member_lints<'a>(document: &'a CargoTomlDocument, family: &str) -> Option<&'a ToolLints> {
    typed(document)?
        .lints
        .as_ref()
        .and_then(|lints| lints.tools.get(family))
}

#[must_use]
pub fn member_lints_state<'a>(
    document: &'a CargoTomlDocument,
    family: &str,
) -> CargoLintTableState<'a> {
    lint_table_state(
        document.raw.get("lints"),
        family,
        member_lints(document, family),
    )
}

#[must_use]
pub fn policy_allow_entries(document: &CargoTomlDocument, family: &str) -> Vec<String> {
    explicit_allow_entries(policy_lints(document, family))
}

#[must_use]
pub fn member_allow_entries(document: &CargoTomlDocument, family: &str) -> Vec<String> {
    explicit_allow_entries(member_lints(document, family))
}

#[must_use]
pub fn policy_lint_level<'a>(
    document: &'a CargoTomlDocument,
    family: &str,
    name: &str,
) -> Option<&'a str> {
    lint_level(policy_lints(document, family)?.get(name))
}

#[must_use]
pub fn member_lint_level<'a>(
    document: &'a CargoTomlDocument,
    family: &str,
    name: &str,
) -> Option<&'a str> {
    lint_level(member_lints(document, family)?.get(name))
}

/// Resolves a single `[package].field` (or workspace inherit form) into the typed `CargoStringFieldState`.
fn string_field<'a>(table: Option<&'a Value>, field: &str) -> CargoStringFieldState<'a> {
    let Some(value) = table.and_then(|table| table.get(field)) else {
        return CargoStringFieldState::Missing;
    };

    match value {
        Value::String(field_value) => CargoStringFieldState::Value(field_value),
        Value::Table(field_table)
            if field_table.len() == 1
                && field_table.get("workspace").and_then(Value::as_bool) == Some(true) =>
        {
            CargoStringFieldState::Inherit
        }
        Value::Integer(_)
        | Value::Float(_)
        | Value::Boolean(_)
        | Value::Datetime(_)
        | Value::Array(_)
        | Value::Table(_) => CargoStringFieldState::WrongType(value),
    }
}

/// Returns sorted lint names whose level resolves to `"allow"` in the given `ToolLints` block.
fn explicit_allow_entries(lints: Option<&ToolLints>) -> Vec<String> {
    let Some(lints) = lints else {
        return Vec::new();
    };
    let mut entries = lints
        .iter()
        .filter(|&(_name, value)| lint_level(Some(value)) == Some("allow"))
        .map(|(name, _value)| name.clone())
        .collect::<Vec<_>>();
    entries.sort();
    entries
}

/// Combines raw and typed lint-table views into a single `CargoLintTableState`.
fn lint_table_state<'a>(
    raw_lints: Option<&'a Value>,
    family: &str,
    typed_lints: Option<&'a ToolLints>,
) -> CargoLintTableState<'a> {
    let Some(raw_lints) = raw_lints else {
        return CargoLintTableState::Missing;
    };
    let Value::Table(raw_lints_table) = raw_lints else {
        return CargoLintTableState::WrongType(raw_lints);
    };
    let Some(raw_family_value) = raw_lints_table.get(family) else {
        return CargoLintTableState::Missing;
    };
    if let Some(typed_lints) = typed_lints {
        return CargoLintTableState::Parsed(typed_lints);
    }
    CargoLintTableState::WrongType(raw_family_value)
}

/// Extracts the lint level string from a `LintValue` regardless of compact or detailed form.
fn lint_level(value: Option<&LintValue>) -> Option<&str> {
    match value? {
        LintValue::Level(level) => Some(level.as_str()),
        LintValue::Detailed(detail) => Some(detail.level.as_str()),
    }
}
