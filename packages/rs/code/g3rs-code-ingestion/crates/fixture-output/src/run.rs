//! Command implementation for serialized `g3rs-code-ingestion` fixture output.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use g3_workspace_crawl::G3WorkspaceCrawlError;
use g3rs_code_ingestion_types::G3RsCodeIngestionError;
use g3rs_code_types::{
    G3RsCodeConfigChecksInput, G3RsCodeFileTreeChecksInput, G3RsCodeSourceChecksInput,
};
use serde::Serialize;

/// Serialized config-ingestion lane result.
type ConfigChecksResult = Result<G3RsCodeConfigChecksInput, G3RsCodeIngestionError>;
/// Serialized source-ingestion lane result.
type SourceChecksResult = Result<Vec<G3RsCodeSourceChecksInput>, G3RsCodeIngestionError>;
/// Serialized file-tree-ingestion lane result.
type FileTreeChecksResult = Result<G3RsCodeFileTreeChecksInput, G3RsCodeIngestionError>;

/// Fixture-output command failure.
#[derive(Debug)]
pub enum FixtureOutputError {
    /// Command-line arguments were not the supported `--path <workspace>` form.
    Usage(String),
    /// Workspace crawl failed before family ingestion could run.
    Crawl(G3WorkspaceCrawlError),
    /// JSON serialization failed.
    Json(serde_json::Error),
}

impl std::fmt::Display for FixtureOutputError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Usage(message) => write!(formatter, "{message}"),
            Self::Crawl(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for FixtureOutputError {}

impl From<G3WorkspaceCrawlError> for FixtureOutputError {
    fn from(error: G3WorkspaceCrawlError) -> Self {
        Self::Crawl(error)
    }
}

impl From<serde_json::Error> for FixtureOutputError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

/// Serialized output for one code-ingestion fixture run.
#[derive(Debug, Serialize)]
struct CodeIngestionFixtureOutput {
    /// Stable output schema identifier.
    schema_version: &'static str,
    /// Result from `ingest_for_config_checks`.
    config_checks: ConfigChecksResult,
    /// Result from `ingest_for_source_checks`.
    source_checks: SourceChecksResult,
    /// Result from `ingest_for_file_tree_checks`.
    file_tree_checks: FileTreeChecksResult,
}

/// Render fixture output for command-line arguments.
///
/// # Errors
///
/// Returns an error when crawling or JSON serialization fails.
pub fn run_from_env() -> Result<String, FixtureOutputError> {
    let path = parse_path_arg(std::env::args_os().skip(1))?;
    render_path(&path)
}

/// Render fixture output for one workspace path.
///
/// # Errors
///
/// Returns an error when crawling or JSON serialization fails.
pub fn render_path(workspace_path: &Path) -> Result<String, FixtureOutputError> {
    let crawl = g3_workspace_crawl::crawl(workspace_path)?;
    let output = CodeIngestionFixtureOutput {
        schema_version: "g3rs-code-ingestion-fixture-output-v1",
        config_checks: g3rs_code_ingestion::ingest_for_config_checks(&crawl),
        source_checks: g3rs_code_ingestion::ingest_for_source_checks(&crawl),
        file_tree_checks: g3rs_code_ingestion::ingest_for_file_tree_checks(&crawl),
    };
    Ok(serde_json::to_string_pretty(&output)?)
}

/// Parse the only supported command shape.
fn parse_path_arg(mut args: impl Iterator<Item = OsString>) -> Result<PathBuf, FixtureOutputError> {
    let Some(flag) = args.next() else {
        return Err(usage_error());
    };
    if flag != "--path" {
        return Err(usage_error());
    }
    let Some(path) = args.next() else {
        return Err(usage_error());
    };
    if args.next().is_some() {
        return Err(usage_error());
    }
    Ok(PathBuf::from(path))
}

/// Create the stable usage error.
fn usage_error() -> FixtureOutputError {
    FixtureOutputError::Usage(
        "usage: g3rs-code-ingestion-fixture-output --path <workspace-root>".to_owned(),
    )
}
