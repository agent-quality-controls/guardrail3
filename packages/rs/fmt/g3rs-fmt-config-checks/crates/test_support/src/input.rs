use cargo_toml_parser::types::{CargoToml, InheritableValue};
use g3rs_fmt_types::{
    G3RsFmtCargoFacts, G3RsFmtCargoState, G3RsFmtConfigChecksInput, G3RsFmtRustPolicyState,
    G3RsFmtRustfmtConfigState, G3RsFmtRustfmtFacts, G3RsFmtToolchainFacts, G3RsFmtToolchainState,
    G3RsFmtWaiver,
};
use rust_toolchain_toml_parser::types::RustToolchainToml;
use rustfmt_toml_parser::types::{Edition, RustfmtToml, StyleEdition};

pub fn parsed_rustfmt(rustfmt_toml: &str) -> G3RsFmtRustfmtConfigState {
    let rustfmt =
        rustfmt_toml_parser::parse(rustfmt_toml).expect("rustfmt test fixture should parse");
    let keys = explicit_keys(rustfmt_toml);
    G3RsFmtRustfmtConfigState::Parsed(rustfmt_facts(&rustfmt, &keys))
}

pub fn parsed_cargo(cargo_toml: &str) -> G3RsFmtCargoState {
    let cargo = cargo_toml_parser::parse(cargo_toml).expect("cargo test fixture should parse");
    G3RsFmtCargoState::Parsed(G3RsFmtCargoFacts {
        edition: cargo_edition(&cargo).map(str::to_owned),
    })
}

pub fn parsed_toolchain(toolchain_toml: &str) -> G3RsFmtToolchainState {
    let toolchain = rust_toolchain_toml_parser::parse(toolchain_toml)
        .expect("toolchain test fixture should parse");
    G3RsFmtToolchainState::Parsed(G3RsFmtToolchainFacts {
        channel: toolchain_channel(&toolchain).map(str::to_owned),
    })
}

pub fn explicit_keys(rustfmt_toml: &str) -> Vec<String> {
    toml::from_str::<toml::Value>(rustfmt_toml)
        .expect("rustfmt test fixture should parse as toml")
        .as_table()
        .expect("rustfmt test fixture should be a toml table")
        .keys()
        .cloned()
        .collect()
}

pub fn rustfmt_input(
    rustfmt_state: G3RsFmtRustfmtConfigState,
    cargo_state: G3RsFmtCargoState,
    toolchain_state: G3RsFmtToolchainState,
    rust_policy: G3RsFmtRustPolicyState,
) -> G3RsFmtConfigChecksInput {
    G3RsFmtConfigChecksInput {
        rustfmt_rel_path: "rustfmt.toml".to_owned(),
        rustfmt_state,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state,
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_state,
        rust_policy,
    }
}

pub fn waiver(reason: &str) -> G3RsFmtWaiver {
    G3RsFmtWaiver {
        rule: "RS-FMT-CONFIG-07".to_owned(),
        file: "rustfmt.toml".to_owned(),
        selector: "ignore".to_owned(),
        reason: reason.to_owned(),
    }
}

fn rustfmt_facts(rustfmt: &RustfmtToml, explicit_keys: &[String]) -> G3RsFmtRustfmtFacts {
    G3RsFmtRustfmtFacts {
        edition: rustfmt.edition.map(edition_str).map(str::to_owned),
        style_edition: rustfmt
            .style_edition
            .map(style_edition_str)
            .map(str::to_owned),
        max_width: rustfmt.max_width.map(i64::from),
        tab_spaces: rustfmt.tab_spaces.map(i64::from),
        use_field_init_shorthand: rustfmt.use_field_init_shorthand,
        use_try_shorthand: rustfmt.use_try_shorthand,
        reorder_imports: rustfmt.reorder_imports,
        reorder_modules: rustfmt.reorder_modules,
        explicit_keys: explicit_keys.to_vec(),
        nightly_keys: explicit_keys
            .iter()
            .filter(|key| NIGHTLY_KEYS.contains(&key.as_str()))
            .cloned()
            .collect(),
        ignore: rustfmt.ignore.clone(),
    }
}

fn cargo_edition(cargo: &CargoToml) -> Option<&str> {
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

fn inheritable_string(value: Option<&InheritableValue<String>>) -> Option<&str> {
    match value {
        Some(InheritableValue::Value(value)) => Some(value.as_str()),
        Some(InheritableValue::Inherit(_)) | None => None,
    }
}

fn toolchain_channel(toolchain: &RustToolchainToml) -> Option<&str> {
    toolchain
        .toolchain
        .as_ref()
        .and_then(|entry| entry.channel.as_deref())
}

const NIGHTLY_KEYS: &[&str] = &[
    "group_imports",
    "imports_granularity",
    "format_code_in_doc_comments",
    "format_strings",
    "overflow_delimited_expr",
    "normalize_comments",
    "normalize_doc_attributes",
    "wrap_comments",
    "format_macro_matchers",
    "format_macro_bodies",
    "condense_wildcard_suffixes",
];

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
