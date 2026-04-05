use rustfmt_toml_parser::RustfmtToml;
use cargo_toml_parser::CargoToml;

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
                .and_then(|package| package.edition.as_deref())
        })
}
