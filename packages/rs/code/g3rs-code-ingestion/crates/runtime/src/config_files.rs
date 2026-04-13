use cargo_toml_parser::parse;
use g3rs_code_ingestion_types::{
    G3RsCodeConfigChecksInput, G3RsCodeConfigFile, G3RsCodeConfigFileKind,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use crate::run::IngestionError;

const CONFIG_FILE_NAMES: &[&str] = &[
    "guardrail3.toml",
    "clippy.toml",
    ".clippy.toml",
    "deny.toml",
    ".deny.toml",
    "Cargo.toml",
    "rustfmt.toml",
    "rust-toolchain.toml",
    "rust-toolchain",
];

pub(crate) fn collect_config_files(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCodeConfigChecksInput, IngestionError> {
    let mut files = Vec::new();

    for entry in crate::config_scope::select_owned_config_entries(crawl, CONFIG_FILE_NAMES)? {
        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }

        let content =
            crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;

        let kind = if entry.path.rel_path.ends_with("Cargo.toml") {
            G3RsCodeConfigFileKind::CargoToml {
                cargo: parse(&content).map_err(|err| IngestionError::ParseFailed {
                    path: entry.path.abs_path.clone(),
                    reason: err.to_string(),
                })?,
            }
        } else {
            G3RsCodeConfigFileKind::Text
        };

        files.push(G3RsCodeConfigFile {
            rel_path: entry.path.rel_path.clone(),
            content,
            kind,
        });
    }

    Ok(G3RsCodeConfigChecksInput { files })
}
