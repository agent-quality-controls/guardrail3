use std::collections::BTreeSet;

use cargo_toml_parser::{CargoToml, WorkspacePackageSection};
use cliff_toml_parser::CliffToml;
use release_plz_toml_parser::ReleasePlzToml;

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
    pub publish_declared: bool,
    pub publishable: bool,
    pub is_binary: bool,
    pub is_library: bool,
    pub binary_target_names: BTreeSet<String>,
    pub description_present: bool,
    pub license_present: bool,
    pub repository_present: bool,
    pub keywords_count: Option<usize>,
    pub categories_count: Option<usize>,
    pub version_string: Option<String>,
    pub workspace_version: bool,
    pub version_valid: bool,
    pub docs_rs_present: bool,
    pub include_exclude_present: bool,
    pub has_binstall_metadata: bool,
    pub binary_release_workflow_present: bool,
    pub linux_release_target_present: bool,
    pub dry_run: Option<G3RsReleaseDryRunOutcome>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsReleaseConfigEdge {
    pub crate_name: String,
    pub cargo_rel_path: String,
    pub source_publishable: bool,
    pub dep_name: String,
    pub dep_package_name: String,
    pub section_label: String,
    pub target_label: Option<String>,
    pub has_path: bool,
    pub path_target_kind: Option<G3RsReleasePathTargetKind>,
    pub dep_publishable: bool,
    pub version_req: Option<String>,
    pub actual_version: Option<String>,
    pub version_satisfied: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigRepo {
    pub cargo_rel_path: String,
    pub release_plz_rel_path: String,
    pub release_plz_exists: bool,
    pub release_plz: Option<ReleasePlzToml>,
    pub release_plz_package_names: BTreeSet<String>,
    pub cliff_rel_path: String,
    pub cliff_exists: bool,
    pub cliff: Option<CliffToml>,
    pub has_release_plz_workflow: bool,
    pub release_plz_workflow_rel_path: Option<String>,
    pub has_publish_dry_run_workflow: bool,
    pub publish_dry_run_workflow_rel_path: Option<String>,
    pub has_registry_token_workflow: bool,
    pub registry_token_workflow_rel_path: Option<String>,
    pub publishable_crate_names: BTreeSet<String>,
    pub publishable_binary_crate_names: BTreeSet<String>,
    pub publishable_count: usize,
    pub non_publishable_count: usize,
    pub semver_checks_installed: bool,
    pub publish_setting: Option<String>,
    pub release_profile_settings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct G3RsReleaseConfigChecksInput {
    pub repo: Option<G3RsReleaseConfigRepo>,
    pub crates: Vec<G3RsReleaseConfigCrate>,
    pub edges: Vec<G3RsReleaseConfigEdge>,
    pub input_failures: Vec<G3RsReleaseInputFailure>,
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
