use cargo_toml_parser::types::{CargoToml, InheritableValue};
/// Public ingestion entry point.
use g3rs_fmt_types as fmt_types;
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use rust_toolchain_toml_parser::types::RustToolchainToml;
use rustfmt_toml_parser::types::{Edition, RustfmtToml, StyleEdition};

/// Re-export of `G3RsFmtIngestionError` so the facade can reach it.
pub use g3rs_fmt_ingestion_types::G3RsFmtIngestionError as IngestionError;

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<fmt_types::G3RsFmtConfigChecksInput, IngestionError> {
    let rustfmt_entry = crate::select::select_active_rustfmt_config(crawl)
        .ok_or(IngestionError::RustfmtTomlNotFound)?;
    let rustfmt_state = ingest_rustfmt_config(rustfmt_entry)?;
    let cargo_state = ingest_cargo_config(crawl);
    let toolchain_state = ingest_toolchain_config(crawl);

    Ok(fmt_types::G3RsFmtConfigChecksInput {
        rustfmt_rel_path: rustfmt_entry.path.rel_path.clone(),
        rustfmt_state,
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo_state,
        toolchain_rel_path: "rust-toolchain.toml".to_owned(),
        toolchain_state,
        rust_policy: ingest_rust_policy(crawl),
    })
}

pub fn ingest_for_source_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<fmt_types::G3RsFmtSourceChecksInput, IngestionError> {
    Err(IngestionError::SourceIngestionNotImplemented)
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<fmt_types::G3RsFmtFileTreeChecksInput, IngestionError> {
    Ok(fmt_types::G3RsFmtFileTreeChecksInput {
        root_rustfmt_toml_rel_path: crate::select::select_root_rustfmt_toml(crawl)
            .map(|entry| entry.path.rel_path.clone()),
        root_dot_rustfmt_toml_rel_path: crate::select::select_root_dot_rustfmt_toml(crawl)
            .map(|entry| entry.path.rel_path.clone()),
        nested_config_files: crate::select::collect_nested_config_files(crawl),
        dual_conflict_dirs: crate::select::collect_dual_conflict_dirs(crawl),
    })
}

fn ingest_rustfmt_config(
    entry: &g3rs_workspace_crawl::G3RsWorkspaceEntry,
) -> Result<fmt_types::G3RsFmtRustfmtConfigState, IngestionError> {
    if !entry.readable {
        return Ok(fmt_types::G3RsFmtRustfmtConfigState::Unreadable);
    }
    match crate::parse::parse_rustfmt_toml(&entry.path.abs_path) {
        Ok((rustfmt, explicit_keys)) => Ok(fmt_types::G3RsFmtRustfmtConfigState::Parsed(
            rustfmt_facts(&rustfmt, &explicit_keys),
        )),
        Err(IngestionError::Unreadable { .. }) => {
            Ok(fmt_types::G3RsFmtRustfmtConfigState::Unreadable)
        }
        Err(_) => Ok(fmt_types::G3RsFmtRustfmtConfigState::ParseError),
    }
}

fn ingest_cargo_config(crawl: &G3RsWorkspaceCrawl) -> fmt_types::G3RsFmtCargoState {
    let Some(entry) = crate::select::select_cargo_toml(crawl) else {
        return fmt_types::G3RsFmtCargoState::Missing;
    };
    if !entry.readable {
        return fmt_types::G3RsFmtCargoState::Unreadable;
    }
    match crate::parse::parse_cargo_toml(&entry.path.abs_path) {
        Ok(cargo) => fmt_types::G3RsFmtCargoState::Parsed(fmt_types::G3RsFmtCargoFacts {
            edition: cargo_edition(&cargo).map(str::to_owned),
        }),
        Err(IngestionError::ParseFailed { .. }) => fmt_types::G3RsFmtCargoState::ParseError,
        Err(IngestionError::Unreadable { .. }) => fmt_types::G3RsFmtCargoState::Unreadable,
        Err(_) => fmt_types::G3RsFmtCargoState::ParseError,
    }
}

fn ingest_toolchain_config(crawl: &G3RsWorkspaceCrawl) -> fmt_types::G3RsFmtToolchainState {
    let Some(entry) = crate::select::select_toolchain_toml(crawl) else {
        return fmt_types::G3RsFmtToolchainState::Missing;
    };
    if !entry.readable {
        return fmt_types::G3RsFmtToolchainState::Unreadable;
    }
    match crate::parse::parse_toolchain_toml(&entry.path.abs_path) {
        Ok(toolchain) => {
            fmt_types::G3RsFmtToolchainState::Parsed(fmt_types::G3RsFmtToolchainFacts {
                channel: toolchain_channel(&toolchain).map(str::to_owned),
            })
        }
        Err(IngestionError::ParseFailed { .. }) => fmt_types::G3RsFmtToolchainState::ParseError,
        Err(IngestionError::Unreadable { .. }) => fmt_types::G3RsFmtToolchainState::Unreadable,
        Err(_) => fmt_types::G3RsFmtToolchainState::ParseError,
    }
}

fn rustfmt_facts(
    rustfmt: &RustfmtToml,
    explicit_keys: &[String],
) -> fmt_types::G3RsFmtRustfmtFacts {
    fmt_types::G3RsFmtRustfmtFacts {
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

fn ingest_rust_policy(crawl: &G3RsWorkspaceCrawl) -> fmt_types::G3RsFmtRustPolicyState {
    let Some(entry) = crate::select::select_rust_policy_toml(crawl) else {
        return fmt_types::G3RsFmtRustPolicyState::Missing;
    };
    if !entry.readable {
        return fmt_types::G3RsFmtRustPolicyState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "file is not readable".to_owned(),
        };
    }
    let Ok(content) = crate::fs::read_to_string(&entry.path.abs_path) else {
        return fmt_types::G3RsFmtRustPolicyState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "file is not readable".to_owned(),
        };
    };
    let parsed = match guardrail3_rs_toml_parser::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return fmt_types::G3RsFmtRustPolicyState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };
    fmt_types::G3RsFmtRustPolicyState::Parsed {
        rel_path: entry.path.rel_path.clone(),
        waivers: parsed
            .waivers
            .into_iter()
            .map(|waiver| fmt_types::G3RsFmtWaiver {
                rule: waiver.rule,
                file: waiver.file,
                selector: waiver.selector,
                reason: waiver.reason,
            })
            .collect(),
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
