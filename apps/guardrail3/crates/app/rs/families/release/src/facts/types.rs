use std::collections::BTreeSet;

use guardrail3_outbound_traits::CommandRunResult;

use crate::release_support::workflows::WorkflowAnalysis;

#[derive(Debug, Clone)]
pub struct RepoReleaseFacts {
    pub(crate) cargo_rel_path: String,
    pub(crate) license_rel_path: Option<String>,
    pub(crate) release_plz_rel_path: String,
    pub(crate) release_plz_exists: bool,
    pub(crate) release_plz_parsed: Option<toml::Value>,
    pub(crate) release_plz_package_names: BTreeSet<String>,
    pub(crate) cliff_rel_path: String,
    pub(crate) cliff_exists: bool,
    pub(crate) cliff_parsed: Option<toml::Value>,
    pub(crate) workflows: Vec<WorkflowFacts>,
    pub(crate) publishable_crate_names: BTreeSet<String>,
    pub(crate) publishable_binary_crate_names: BTreeSet<String>,
    pub(crate) publishable_count: usize,
    pub(crate) non_publishable_count: usize,
    pub(crate) semver_checks_installed: bool,
    pub(crate) publish_setting: Option<String>,
    pub(crate) release_profile_settings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorkflowFacts {
    pub(crate) rel_path: String,
    pub(crate) analysis: WorkflowAnalysis,
}

#[derive(Debug, Clone)]
pub struct PublishableCrateFacts {
    pub(crate) name: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) binary_target_names: BTreeSet<String>,
    pub(crate) publishable: bool,
    pub(crate) is_binary: bool,
    pub(crate) is_library: bool,
    pub(crate) description_present: bool,
    pub(crate) license_present: bool,
    pub(crate) repository_present: bool,
    pub(crate) readme_declared_false: bool,
    pub(crate) readme_rel_path: String,
    pub(crate) readme_exists: bool,
    pub(crate) readme_content: Option<String>,
    pub(crate) keywords_count: Option<usize>,
    pub(crate) categories_count: Option<usize>,
    pub(crate) version_string: Option<String>,
    pub(crate) workspace_version: bool,
    pub(crate) version_valid: bool,
    pub(crate) docs_rs_present: bool,
    pub(crate) include_exclude_present: bool,
    pub(crate) has_binstall_metadata: bool,
    pub(crate) dry_run: Option<CommandRunResult>,
}

#[derive(Debug, Clone)]
pub struct ReleaseEdgeFacts {
    pub(crate) crate_name: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) dep_name: String,
    pub(crate) dep_package_name: String,
    pub(crate) section_label: String,
    pub(crate) target_label: Option<String>,
    pub(crate) has_path: bool,
    pub(crate) dep_publishable: bool,
    pub(crate) version_req: Option<String>,
    pub(crate) actual_version: Option<String>,
    pub(crate) version_satisfied: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ReleaseInputFailureFacts {
    pub(crate) rel_path: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone, Default)]
pub struct ReleaseFacts {
    pub(crate) repo: Vec<RepoReleaseFacts>,
    pub(crate) crates: Vec<PublishableCrateFacts>,
    pub(crate) edges: Vec<ReleaseEdgeFacts>,
    pub(crate) input_failures: Vec<ReleaseInputFailureFacts>,
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
