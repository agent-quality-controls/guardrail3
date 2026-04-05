use cargo_toml_parser::{CargoToml, InheritableValue};
use rustfmt_toml_parser::{Edition, RustfmtToml, StyleEdition};

pub(crate) fn rustfmt_table(rustfmt: &RustfmtToml) -> toml::value::Table {
    let value = toml::Value::try_from(rustfmt.clone())
        .expect("typed RustfmtToml should serialize to toml::Value");
    value
        .as_table()
        .cloned()
        .expect("typed RustfmtToml should serialize as a table")
}

pub(crate) fn cargo_edition(cargo: &CargoToml) -> Option<&str> {
    cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.package.as_ref())
        .and_then(|package| package.edition.as_deref())
        .or_else(|| {
            cargo.package
                .as_ref()
                .and_then(|package| inheritable_string(package.edition.as_ref()))
        })
}

pub(crate) fn rustfmt_edition(edition: Option<Edition>) -> Option<&'static str> {
    edition.map(edition_str)
}

pub(crate) fn rustfmt_style_edition(edition: Option<StyleEdition>) -> Option<&'static str> {
    edition.map(style_edition_str)
}

fn inheritable_string(value: Option<&InheritableValue<String>>) -> Option<&str> {
    match value {
        Some(InheritableValue::Value(value)) => Some(value.as_str()),
        Some(InheritableValue::Inherit(_)) | None => None,
    }
}

const fn edition_str(edition: Edition) -> &'static str {
    match edition {
        Edition::Edition2015 => "2015",
        Edition::Edition2018 => "2018",
        Edition::Edition2021 => "2021",
        Edition::Edition2024 => "2024",
    }
}

const fn style_edition_str(edition: StyleEdition) -> &'static str {
    match edition {
        StyleEdition::Edition2015 => "2015",
        StyleEdition::Edition2018 => "2018",
        StyleEdition::Edition2021 => "2021",
        StyleEdition::Edition2024 => "2024",
        StyleEdition::Edition2027 => "2027",
    }
}
