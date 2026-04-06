/// Typed representation of a `cliff.toml` configuration file.
///
/// Unknown keys are captured in `extra` for forward compatibility.
use std::collections::BTreeMap;

use serde::Deserialize;
use toml::Value;

/// Top-level `cliff.toml` configuration.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CliffToml {
    /// Git configuration section.
    pub git: Option<CliffGitSection>,
    /// Changelog output configuration section.
    pub changelog: Option<CliffChangelogSection>,
    /// Unknown top-level keys.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// The `[git]` section of `cliff.toml`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CliffGitSection {
    /// Whether to parse commits using the Conventional Commits spec.
    pub conventional_commits: Option<bool>,
    /// Whether to filter out commits that are not conventional.
    pub filter_unconventional: Option<bool>,
    /// Rules for parsing and grouping commits.
    pub commit_parsers: Option<Vec<CliffCommitParser>>,
    /// Unknown keys in the `[git]` section.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// The `[changelog]` section of `cliff.toml`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CliffChangelogSection {
    /// Header template for the changelog.
    pub header: Option<String>,
    /// Body template for changelog entries.
    pub body: Option<String>,
    /// Footer template for the changelog.
    pub footer: Option<String>,
    /// Whether to trim leading/trailing whitespace from the rendered output.
    pub trim: Option<bool>,
    /// Unknown keys in the `[changelog]` section.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// A single commit parser rule in the `[[git.commit_parsers]]` array.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CliffCommitParser {
    /// Regex pattern to match against commit messages.
    pub message: Option<String>,
    /// Group name to assign matching commits to.
    pub group: Option<String>,
    /// Whether to skip matching commits entirely.
    pub skip: Option<bool>,
    /// Unknown keys in this commit parser entry.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
