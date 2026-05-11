use cargo_toml_parser::types::CargoToml;
use g3_workspace_crawl::G3WorkspaceCrawl;
/// Public ingestion entry point.
use g3rs_garde_source_checks_types::G3RsGardeSourceChecksInput;
use g3rs_garde_types::{
    G3RsGardeApplicability, G3RsGardeClippyInput, G3RsGardeConfigChecksInput,
    G3RsGardeFileTreeChecksInput, G3RsGardeRustPolicyInput, G3RsGardeWaiver, G3RsSourceFile,
};
use g3rs_toml_parser::types::Guardrail3RsToml;

/// Re-export of `G3RsGardeIngestionError` so the facade can reach it.
pub use g3rs_garde_ingestion_types::G3RsGardeIngestionError as IngestionError;

/// Ingest garde config from a workspace crawl into a config checks input.
///
/// Cargo.toml is required. Clippy config is optional — if absent,
/// the clippy state will be `Missing` in the result and the config lane
/// will emit its own "cannot verify" warnings when garde is present.
///
/// # Errors
///
/// Returns an error if Cargo.toml is missing, unreadable, or unparseable.
pub fn ingest_for_config_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsGardeConfigChecksInput, IngestionError> {
    let cargo_entry =
        crate::select::select_cargo_toml(crawl).ok_or(IngestionError::CargoTomlNotFound)?;
    if !cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let cargo = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path)?;
    let garde_dependency_present = has_garde_dependency(&cargo);
    let rust_policy = parse_rust_policy(crawl);
    let applicability = if garde_dependency_present || rust_policy_enables_garde(&rust_policy) {
        G3RsGardeApplicability::Active
    } else {
        G3RsGardeApplicability::Inactive
    };

    let clippy_input = if applicability == G3RsGardeApplicability::Inactive {
        G3RsGardeClippyInput::Missing
    } else if let Some(entry) = crate::select::select_clippy_toml(crawl) {
        if entry.readable {
            match crate::parse::parse_clippy_toml(&entry.path.abs_path) {
                Ok(parsed) => G3RsGardeClippyInput::Parsed {
                    rel_path: entry.path.rel_path.clone(),
                    clippy: parsed,
                },
                Err(IngestionError::Unreadable { reason, .. }) => G3RsGardeClippyInput::Invalid {
                    rel_path: entry.path.rel_path.clone(),
                    message: format!(
                        "Failed to read `{}` for garde clippy-ban validation: {reason}",
                        entry.path.rel_path
                    ),
                },
                Err(IngestionError::ParseFailed { reason, .. }) => G3RsGardeClippyInput::Invalid {
                    rel_path: entry.path.rel_path.clone(),
                    message: format!(
                        "Failed to parse `{}` for garde clippy-ban validation: {reason}",
                        entry.path.rel_path
                    ),
                },
                Err(other) => return Err(other),
            }
        } else {
            G3RsGardeClippyInput::Invalid {
                rel_path: entry.path.rel_path.clone(),
                message: format!(
                    "Failed to read `{}` for garde clippy-ban validation: file is not readable",
                    entry.path.rel_path
                ),
            }
        }
    } else {
        G3RsGardeClippyInput::Missing
    };

    Ok(crate::ingest::assemble(
        applicability,
        cargo_entry.path.rel_path.clone(),
        cargo,
        clippy_input,
    ))
}

/// Ingest garde source input from a workspace crawl.
///
/// # Errors
/// Returns an error when the underlying operation fails.
pub fn ingest_for_source_checks(
    crawl: &G3WorkspaceCrawl,
) -> Result<G3RsGardeSourceChecksInput, IngestionError> {
    let cargo_entry =
        crate::select::select_cargo_toml(crawl).ok_or(IngestionError::CargoTomlNotFound)?;
    if !cargo_entry.readable {
        return Err(IngestionError::Unreadable {
            path: cargo_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let cargo = crate::parse::parse_cargo_toml(&cargo_entry.path.abs_path)?;
    let garde_dependency_present = has_garde_dependency(&cargo);
    let rust_policy = parse_rust_policy(crawl);
    let applicability = if garde_dependency_present || rust_policy_enables_garde(&rust_policy) {
        G3RsGardeApplicability::Active
    } else {
        G3RsGardeApplicability::Inactive
    };
    let source_files = crate::select::select_ast_source_files(crawl)
        .into_iter()
        .map(|entry| G3RsSourceFile {
            rel_path: entry.path.rel_path.clone(),
            abs_path: entry.path.abs_path.clone(),
        })
        .collect::<Vec<_>>();
    let analysis = crate::source_analysis::analyze_source_files(&source_files, &rust_policy);

    Ok(G3RsGardeSourceChecksInput {
        applicability,
        garde_dependency_present,
        rust_policy,
        input_failures: analysis.input_failures,
        struct_targets: analysis.struct_targets,
        enum_targets: analysis.enum_targets,
        manual_deserialize_impls: analysis.manual_deserialize_impls,
        boundary_fields: analysis.boundary_fields,
        query_as_macros: analysis.query_as_macros,
    })
}

/// Implements `has garde dependency`.
fn has_garde_dependency(cargo: &CargoToml) -> bool {
    cargo.dependencies.contains_key("garde")
        || cargo
            .workspace
            .as_ref()
            .is_some_and(|workspace| workspace.dependencies.contains_key("garde"))
}

/// Implements `parse rust policy`.
fn parse_rust_policy(crawl: &G3WorkspaceCrawl) -> G3RsGardeRustPolicyInput {
    let Some(entry) = crate::select::select_guardrail3_rs_toml(crawl) else {
        return G3RsGardeRustPolicyInput::Missing;
    };

    if !entry.readable {
        return G3RsGardeRustPolicyInput::Invalid {
            rel_path: entry.path.rel_path.clone(),
            message: format!(
                "Failed to read `{}` for garde Rust policy resolution: file is not readable",
                entry.path.rel_path
            ),
        };
    }

    match crate::fs::read_to_string(&entry.path.abs_path) {
        Ok(content) => match g3rs_toml_parser::parse(&content) {
            Ok(parsed) => G3RsGardeRustPolicyInput::Parsed {
                rel_path: entry.path.rel_path.clone(),
                garde_enabled: parsed
                    .checks
                    .as_ref()
                    .and_then(|checks| checks.garde)
                    .unwrap_or(false),
                waivers: collect_waivers(&parsed),
            },
            Err(err) => G3RsGardeRustPolicyInput::Invalid {
                rel_path: entry.path.rel_path.clone(),
                message: format!(
                    "Failed to parse `{}` for garde Rust policy resolution: {err}",
                    entry.path.rel_path
                ),
            },
        },
        Err(err) => G3RsGardeRustPolicyInput::Invalid {
            rel_path: entry.path.rel_path.clone(),
            message: format!(
                "Failed to read `{}` for garde Rust policy resolution: {err}",
                entry.path.rel_path
            ),
        },
    }
}

/// Implements `rust policy enables garde`.
const fn rust_policy_enables_garde(policy: &G3RsGardeRustPolicyInput) -> bool {
    match policy {
        G3RsGardeRustPolicyInput::Parsed { garde_enabled, .. } => *garde_enabled,
        G3RsGardeRustPolicyInput::Missing | G3RsGardeRustPolicyInput::Invalid { .. } => false,
    }
}

/// Implements `collect waivers`.
fn collect_waivers(parsed: &Guardrail3RsToml) -> Vec<G3RsGardeWaiver> {
    parsed
        .waivers
        .iter()
        .map(|waiver| G3RsGardeWaiver {
            rule: waiver.rule.clone(),
            file: waiver.file.clone(),
            selector: waiver.selector.clone(),
            reason: waiver.reason.clone(),
        })
        .collect()
}

/// Stub file-tree ingestion entry point for the garde family.
///
/// # Errors
/// Returns an error when the underlying operation fails.
pub const fn ingest_for_file_tree_checks(
    _crawl: &G3WorkspaceCrawl,
) -> Result<G3RsGardeFileTreeChecksInput, IngestionError> {
    Err(IngestionError::FileTreeIngestionNotImplemented)
}

#[cfg(test)]
#[path = "run_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod run_tests;
