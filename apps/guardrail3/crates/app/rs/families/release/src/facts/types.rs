use std::collections::BTreeSet;

use guardrail3_outbound_traits::CommandRunResult;

use crate::release_support::workflows::WorkflowAnalysis;

#[derive(Debug, Clone)]
pub struct RepoReleaseFacts {
    pub cargo_rel_path: String,
    pub license_rel_path: Option<String>,
    pub release_plz_rel_path: String,
    pub release_plz_exists: bool,
    pub release_plz_parsed: Option<toml::Value>,
    pub release_plz_package_names: BTreeSet<String>,
    pub cliff_rel_path: String,
    pub cliff_exists: bool,
    pub cliff_parsed: Option<toml::Value>,
    pub workflows: Vec<WorkflowFacts>,
    pub publishable_crate_names: BTreeSet<String>,
    pub publishable_binary_crate_names: BTreeSet<String>,
    pub publishable_count: usize,
    pub non_publishable_count: usize,
    pub semver_checks_installed: bool,
    pub publish_setting: Option<String>,
    pub release_profile_settings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorkflowFacts {
    pub rel_path: String,
    pub analysis: WorkflowAnalysis,
}

#[derive(Debug, Clone)]
pub struct PublishableCrateFacts {
    pub name: String,
    pub cargo_rel_path: String,
    pub binary_target_names: BTreeSet<String>,
    pub publishable: bool,
    pub is_binary: bool,
    pub is_library: bool,
    pub description_present: bool,
    pub license_present: bool,
    pub repository_present: bool,
    pub readme_declared_false: bool,
    pub readme_rel_path: String,
    pub readme_exists: bool,
    pub readme_content: Option<String>,
    pub keywords_count: Option<usize>,
    pub categories_count: Option<usize>,
    pub version_string: Option<String>,
    pub workspace_version: bool,
    pub version_valid: bool,
    pub docs_rs_present: bool,
    pub include_exclude_present: bool,
    pub has_binstall_metadata: bool,
    pub dry_run: Option<CommandRunResult>,
}

#[derive(Debug, Clone)]
pub struct ReleaseEdgeFacts {
    pub crate_name: String,
    pub cargo_rel_path: String,
    pub dep_name: String,
    #[allow(dead_code)]
    pub dep_package_name: String,
    pub section_label: String,
    pub target_label: Option<String>,
    pub has_path: bool,
    pub dep_publishable: bool,
    pub version_req: Option<String>,
    pub actual_version: Option<String>,
    pub version_satisfied: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ReleaseInputFailureFacts {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct ReleaseFacts {
    pub repo: Vec<RepoReleaseFacts>,
    pub crates: Vec<PublishableCrateFacts>,
    pub edges: Vec<ReleaseEdgeFacts>,
    pub input_failures: Vec<ReleaseInputFailureFacts>,
}

#[derive(Debug, Clone)]
pub(super) struct CargoRootFacts {
    pub(super) rel_dir: String,
    pub(super) cargo_rel_path: String,
    pub(super) parsed: toml::Value,
    pub(super) has_workspace: bool,
    pub(super) has_package: bool,
    pub(super) workspace_members: Vec<String>,
    pub(super) workspace_exclude: Vec<String>,
    pub(super) workspace_dependencies: toml::map::Map<String, toml::Value>,
    pub(super) package_workspace: Option<String>,
}
