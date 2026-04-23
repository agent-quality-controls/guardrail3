use std::collections::{BTreeMap, BTreeSet};

use cargo_toml_parser::{types::CargoToml, types::WorkspacePackageSection};
use cliff_toml_parser::types::CliffToml;
use release_plz_toml_parser::types::ReleasePlzToml;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsReleaseDryRunOutcome {
    Passed,
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsReleasePathTargetKind {
    InWorkspace,
    OutsideWorkspace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseInputFailure {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigCrate {
    pub name: String,
    pub cargo_rel_path: String,
    pub cargo: CargoToml,
    pub workspace_package: Option<WorkspacePackageSection>,
    pub is_binary: bool,
    pub is_library: bool,
    pub binary_target_names: BTreeSet<String>,
    pub dry_run: Option<G3RsReleaseDryRunOutcome>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsReleaseWorkflowStep {
    pub uses: Option<String>,
    pub run_lines: Vec<String>,
    pub env_keys: Vec<String>,
    pub env_bindings: BTreeMap<String, String>,
    pub with_bindings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsReleaseWorkflowJob {
    pub id: String,
    pub runs_on: Vec<String>,
    pub needs: Vec<String>,
    pub matrix_axes: BTreeMap<String, Vec<String>>,
    pub env_keys: Vec<String>,
    pub env_bindings: BTreeMap<String, String>,
    pub steps: Vec<G3RsReleaseWorkflowStep>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsReleaseWorkflowAnalysis {
    pub jobs: Vec<G3RsReleaseWorkflowJob>,
    pub steps: Vec<G3RsReleaseWorkflowStep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseWorkflow {
    pub rel_path: String,
    pub analysis: G3RsReleaseWorkflowAnalysis,
}

#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigEdge {
    pub source: G3RsReleaseConfigCrate,
    pub dep_name: String,
    pub dep_package_name: String,
    pub section_label: String,
    pub target_label: Option<String>,
    pub has_path: bool,
    pub path_target_kind: Option<G3RsReleasePathTargetKind>,
    pub version_req: Option<String>,
    pub target: Option<G3RsReleaseConfigCrate>,
}

#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigRepo {
    pub cargo_rel_path: String,
    pub cargo: CargoToml,
    pub release_plz_rel_path: String,
    pub release_plz_exists: bool,
    pub release_plz: Option<ReleasePlzToml>,
    pub cliff_rel_path: String,
    pub cliff_exists: bool,
    pub cliff: Option<CliffToml>,
    pub workflows: Vec<G3RsReleaseWorkflow>,
    pub has_release_plz_workflow: bool,
    pub release_plz_workflow_rel_path: Option<String>,
    pub has_publish_dry_run_workflow: bool,
    pub publish_dry_run_workflow_rel_path: Option<String>,
    pub has_registry_token_workflow: bool,
    pub registry_token_workflow_rel_path: Option<String>,
    pub semver_checks_installed: bool,
}

#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigChecksInput {
    pub repo_checks: Vec<G3RsReleaseConfigRepo>,
    pub crate_checks: Vec<G3RsReleaseConfigCrate>,
    pub edge_checks: Vec<G3RsReleaseConfigEdge>,
    pub input_failure_checks: Vec<G3RsReleaseInputFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseFileTreeRepo {
    pub cargo_rel_path: String,
    pub publishable_count: usize,
    pub license_rel_path: Option<String>,
    pub release_plz_rel_path: String,
    pub release_plz_exists: bool,
    pub cliff_rel_path: String,
    pub cliff_exists: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseFileTreeReadme {
    pub crate_name: String,
    pub cargo_rel_path: String,
    pub publishable: bool,
    pub readme_declared_false: bool,
    pub readme_rel_path: String,
    pub readme_exists: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseFileTreeChecksInput {
    pub repo: Option<G3RsReleaseFileTreeRepo>,
    pub readmes: Vec<G3RsReleaseFileTreeReadme>,
    pub input_failures: Vec<G3RsReleaseInputFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseSourceReadme {
    pub crate_name: String,
    pub cargo_rel_path: String,
    pub readme_rel_path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseSourceChecksInput {
    pub readmes: Vec<G3RsReleaseSourceReadme>,
    pub input_failures: Vec<G3RsReleaseInputFailure>,
}
