//! Release-family input types.

use std::collections::{BTreeMap, BTreeSet};

use cargo_toml_parser::{types::CargoToml, types::WorkspacePackageSection};
use cliff_toml_parser::types::CliffToml;
use release_plz_toml_parser::types::ReleasePlzToml;

/// Mapping from a workflow matrix axis name to its declared values.
pub(crate) type G3RsReleaseMatrixAxes = BTreeMap<String, Vec<String>>;

/// Outcome of a release dry-run for a publishable crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsReleaseDryRunOutcome {
    /// Dry run succeeded.
    Passed,
    /// Dry run failed with the captured reason.
    Failed(String),
}

/// Whether a path-dependency target lives inside or outside the workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsReleasePathTargetKind {
    /// Target lives inside the same workspace.
    InWorkspace,
    /// Target lives outside the workspace.
    OutsideWorkspace,
}

/// A failure encountered while ingesting a release input file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseInputFailure {
    /// Repo-relative path of the offending input.
    pub rel_path: String,
    /// Human-readable failure message.
    pub message: String,
}

/// A crate participating in release-family checks.
#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigCrate {
    /// Crate name as declared in the manifest.
    pub name: String,
    /// Repo-relative path to the crate's `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Parsed `Cargo.toml`.
    pub cargo: CargoToml,
    /// Resolved `[workspace.package]` section, when applicable.
    pub workspace_package: Option<WorkspacePackageSection>,
    /// Whether the crate produces a binary target.
    pub is_binary: bool,
    /// Whether the crate produces a library target.
    pub is_library: bool,
    /// Names of binary targets declared by the crate.
    pub binary_target_names: BTreeSet<String>,
    /// Outcome of the most recent release dry-run, if any.
    pub dry_run: Option<G3RsReleaseDryRunOutcome>,
}

/// A single GitHub Actions workflow step.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsReleaseWorkflowStep {
    /// `uses:` action reference, if any.
    pub uses: Option<String>,
    /// `run:` script lines.
    pub run_lines: Vec<String>,
    /// Environment variable names referenced by the step.
    pub env_keys: Vec<String>,
    /// Environment variable bindings declared on the step.
    pub env_bindings: BTreeMap<String, String>,
    /// `with:` bindings declared on the step.
    pub with_bindings: BTreeMap<String, String>,
}

/// A single GitHub Actions workflow job.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsReleaseWorkflowJob {
    /// Job identifier.
    pub id: String,
    /// `runs-on` runner labels.
    pub runs_on: Vec<String>,
    /// Job dependencies declared via `needs:`.
    pub needs: Vec<String>,
    /// Matrix axes declared on the job.
    pub matrix_axes: G3RsReleaseMatrixAxes,
    /// Environment variable names referenced by the job.
    pub env_keys: Vec<String>,
    /// Environment variable bindings declared at the job level.
    pub env_bindings: BTreeMap<String, String>,
    /// Steps declared on the job.
    pub steps: Vec<G3RsReleaseWorkflowStep>,
}

/// Aggregated analysis of a single workflow file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsReleaseWorkflowAnalysis {
    /// Top-level environment variable names referenced by the workflow.
    pub env_keys: Vec<String>,
    /// Top-level environment variable bindings declared by the workflow.
    pub env_bindings: BTreeMap<String, String>,
    /// Jobs declared by the workflow.
    pub jobs: Vec<G3RsReleaseWorkflowJob>,
    /// Steps flattened across all jobs.
    pub steps: Vec<G3RsReleaseWorkflowStep>,
}

/// A GitHub Actions workflow file with its analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseWorkflow {
    /// Repo-relative path of the workflow file.
    pub rel_path: String,
    /// Analysis of the workflow's contents.
    pub analysis: G3RsReleaseWorkflowAnalysis,
}

/// A dependency edge between two crates participating in release-family checks.
#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigEdge {
    /// Source crate of the dependency edge.
    pub source: G3RsReleaseConfigCrate,
    /// Dependency entry name as declared.
    pub dep_name: String,
    /// Resolved package name of the dependency target.
    pub dep_package_name: String,
    /// Manifest section label (e.g. `dependencies`).
    pub section_label: String,
    /// Optional target spec label for `[target.*]` entries.
    pub target_label: Option<String>,
    /// Whether the dependency entry declares a `path = ...` field.
    pub has_path: bool,
    /// Resolution kind for the path target, if any.
    pub path_target_kind: Option<G3RsReleasePathTargetKind>,
    /// Optional version requirement.
    pub version_req: Option<String>,
    /// Resolved target crate, when known.
    pub target: Option<G3RsReleaseConfigCrate>,
}

/// Workflow-presence flags for a release-config repo aggregate.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsReleaseConfigRepoWorkflowFlags {
    /// Whether a release-plz workflow is present.
    pub has_release_plz_workflow: bool,
    /// Whether a publish dry-run workflow is present.
    pub has_publish_dry_run_workflow: bool,
    /// Whether a registry-token-aware workflow is present.
    pub has_registry_token_workflow: bool,
}

/// Repo-level release configuration aggregate.
#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigRepo {
    /// Repo-relative path of the workspace `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Parsed workspace `Cargo.toml`.
    pub cargo: CargoToml,
    /// Repo-relative path of `release-plz.toml`.
    pub release_plz_rel_path: String,
    /// Whether `release-plz.toml` exists.
    pub release_plz_exists: bool,
    /// Parsed `release-plz.toml`, when present.
    pub release_plz: Option<ReleasePlzToml>,
    /// Repo-relative path of `cliff.toml`.
    pub cliff_rel_path: String,
    /// Whether `cliff.toml` exists.
    pub cliff_exists: bool,
    /// Parsed `cliff.toml`, when present.
    pub cliff: Option<CliffToml>,
    /// Discovered GitHub Actions workflow files.
    pub workflows: Vec<G3RsReleaseWorkflow>,
    /// Workflow-presence flags.
    pub workflow_flags: G3RsReleaseConfigRepoWorkflowFlags,
    /// Repo-relative path of the release-plz workflow, if any.
    pub release_plz_workflow_rel_path: Option<String>,
    /// Repo-relative path of the publish dry-run workflow, if any.
    pub publish_dry_run_workflow_rel_path: Option<String>,
    /// Repo-relative path of the registry-token workflow, if any.
    pub registry_token_workflow_rel_path: Option<String>,
    /// Whether `cargo-semver-checks` is installed for this repo.
    pub semver_checks_installed: bool,
}

/// Aggregated input for release-family config checks.
#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigChecksInput {
    /// Repo-level entries.
    pub repos: Vec<G3RsReleaseConfigRepo>,
    /// Per-crate entries.
    pub crates: Vec<G3RsReleaseConfigCrate>,
    /// Per-edge entries.
    pub edges: Vec<G3RsReleaseConfigEdge>,
    /// Input ingestion failures.
    pub input_failures: Vec<G3RsReleaseInputFailure>,
}

/// File-tree-level repo aggregate for release checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseFileTreeRepo {
    /// Repo-relative path of the workspace `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Number of publishable crates in the workspace.
    pub publishable_count: usize,
    /// Repo-relative path of the workspace LICENSE file, if any.
    pub license_rel_path: Option<String>,
    /// Repo-relative path of `release-plz.toml`.
    pub release_plz_rel_path: String,
    /// Whether `release-plz.toml` exists.
    pub release_plz_exists: bool,
    /// Repo-relative path of `cliff.toml`.
    pub cliff_rel_path: String,
    /// Whether `cliff.toml` exists.
    pub cliff_exists: bool,
}

/// File-tree-level README record for a publishable crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseFileTreeReadme {
    /// Crate name.
    pub crate_name: String,
    /// Repo-relative path of the crate's `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Whether the crate is publishable.
    pub publishable: bool,
    /// Whether `readme = false` is declared in the manifest.
    pub readme_declared_false: bool,
    /// Repo-relative path of the README.
    pub readme_rel_path: String,
    /// Whether the README file exists on disk.
    pub readme_exists: bool,
}

/// Aggregated input for release-family file-tree checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseFileTreeChecksInput {
    /// Repo-level file-tree aggregate, when present.
    pub repo: Option<G3RsReleaseFileTreeRepo>,
    /// Per-crate README records.
    pub readmes: Vec<G3RsReleaseFileTreeReadme>,
    /// Input ingestion failures.
    pub input_failures: Vec<G3RsReleaseInputFailure>,
}

/// Source-level README record for a publishable crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseSourceReadme {
    /// Crate name.
    pub crate_name: String,
    /// Repo-relative path of the crate's `Cargo.toml`.
    pub cargo_rel_path: String,
    /// Repo-relative path of the README.
    pub readme_rel_path: String,
    /// README file contents.
    pub content: String,
}

/// Aggregated input for release-family source checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseSourceChecksInput {
    /// Per-crate README source records.
    pub readmes: Vec<G3RsReleaseSourceReadme>,
    /// Input ingestion failures.
    pub input_failures: Vec<G3RsReleaseInputFailure>,
}
