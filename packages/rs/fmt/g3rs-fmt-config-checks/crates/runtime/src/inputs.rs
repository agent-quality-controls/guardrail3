use cargo_toml_parser::{types::CargoToml, types::InheritableValue};
use g3rs_fmt_types::{G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtRustfmtConfigState};
use rustfmt_toml_parser::types::{Edition, RustfmtToml, StyleEdition};

/// Implements `rustfmt`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
pub(crate) const fn rustfmt(input: &G3RsFmtConfigChecksInput) -> Option<&RustfmtToml> {
    match &input.rustfmt_state {
        G3RsFmtRustfmtConfigState::Parsed(rustfmt) => Some(rustfmt),
        G3RsFmtRustfmtConfigState::Unreadable | G3RsFmtRustfmtConfigState::ParseError => None,
    }
}

/// Implements `rustfmt table`.
///
/// Returns the typed `RustfmtToml` re-encoded as a `toml::value::Table` for downstream
/// rule queries. Returns an empty table when serialization unexpectedly fails or the
/// serialized value is not a table; both are unreachable for a valid `RustfmtToml`
/// but the rules tolerate an empty table by emitting their normal "missing key"
/// findings.
pub(crate) fn rustfmt_table(rustfmt: &RustfmtToml) -> toml::value::Table {
    let Ok(value) = toml::Value::try_from(rustfmt.clone()) else {
        return toml::value::Table::new();
    };
    match value {
        toml::Value::Table(table) => table,
        toml::Value::String(_)
        | toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_)
        | toml::Value::Array(_) => toml::value::Table::new(),
    }
}

/// Implements `cargo`.
pub(crate) const fn cargo(input: &G3RsFmtConfigChecksInput) -> Option<&CargoToml> {
    match &input.cargo_state {
        G3RsFmtCargoState::Parsed(cargo) => Some(cargo),
        G3RsFmtCargoState::Missing
        | G3RsFmtCargoState::Unreadable
        | G3RsFmtCargoState::ParseError => None,
    }
}

/// Implements `cargo edition`.
pub(crate) fn cargo_edition(cargo: &CargoToml) -> Option<&str> {
    cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.package.as_ref())
        .and_then(|package| package.edition.as_deref())
        .or_else(|| {
            cargo
                .package
                .as_ref()
                .and_then(|package| inheritable_string(package.edition.as_ref()))
        })
}

/// Implements `rustfmt edition`.
pub(crate) fn rustfmt_edition(edition: Option<Edition>) -> Option<&'static str> {
    edition.map(edition_str)
}

/// Implements `rustfmt style edition`.
pub(crate) fn rustfmt_style_edition(edition: Option<StyleEdition>) -> Option<&'static str> {
    edition.map(style_edition_str)
}

/// Implements `inheritable string`.
fn inheritable_string(value: Option<&InheritableValue<String>>) -> Option<&str> {
    match value {
        Some(InheritableValue::Value(value)) => Some(value.as_str()),
        Some(InheritableValue::Inherit(_)) | None => None,
    }
}

/// Implements `edition str`.
const fn edition_str(edition: Edition) -> &'static str {
    match edition {
        Edition::Edition2015 => "2015",
        Edition::Edition2018 => "2018",
        Edition::Edition2021 => "2021",
        Edition::Edition2024 => "2024",
    }
}

/// Implements `style edition str`.
const fn style_edition_str(edition: StyleEdition) -> &'static str {
    match edition {
        StyleEdition::Edition2015 => "2015",
        StyleEdition::Edition2018 => "2018",
        StyleEdition::Edition2021 => "2021",
        StyleEdition::Edition2024 => "2024",
        StyleEdition::Edition2027 => "2027",
    }
}
