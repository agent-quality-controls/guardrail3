use std::collections::BTreeSet;

use super::facts::{
    PublishableCrateFacts, ReleaseEdgeFacts, ReleaseInputFailureFacts, RepoReleaseFacts,
    WorkflowFacts,
};
use super::inputs::{
    PublishableCrateReleaseInput, ReleaseEdgeInput, ReleaseInputFailureInput, RepoReleaseInput,
};

pub fn repo_facts() -> RepoReleaseFacts {
    RepoReleaseFacts {
        cargo_rel_path: "Cargo.toml".to_owned(),
        license_rel_path: None,
        release_plz_rel_path: "release-plz.toml".to_owned(),
        release_plz_exists: false,
        release_plz_parsed: None,
        release_plz_has_workspace: false,
        release_plz_package_names: BTreeSet::new(),
        cliff_rel_path: "cliff.toml".to_owned(),
        cliff_exists: false,
        workflows: Vec::new(),
        publishable_crate_names: BTreeSet::new(),
        publishable_count: 0,
        non_publishable_count: 0,
        semver_checks_installed: false,
        publish_setting: None,
        release_profile_settings: Vec::new(),
    }
}

pub fn workflow(rel_path: &str) -> WorkflowFacts {
    WorkflowFacts {
        rel_path: rel_path.to_owned(),
        has_release_plz_step: false,
        has_publish_dry_run_step: false,
        has_registry_token: false,
        has_binary_release: false,
        has_linux_target: false,
    }
}

pub fn crate_facts(name: &str) -> PublishableCrateFacts {
    PublishableCrateFacts {
        name: name.to_owned(),
        cargo_rel_path: "crates/example/Cargo.toml".to_owned(),
        publishable: true,
        is_binary: false,
        is_library: true,
        description_present: true,
        license_present: true,
        repository_present: true,
        readme_rel_path: "crates/example/README.md".to_owned(),
        readme_exists: true,
        readme_content: Some("# Example\n\n".to_owned() + &"x".repeat(240)),
        keywords_count: Some(3),
        categories_count: Some(1),
        version_string: Some("1.2.3".to_owned()),
        workspace_version: false,
        version_valid: true,
        docs_rs_present: true,
        include_exclude_present: true,
        has_binstall_metadata: false,
        dry_run: None,
    }
}

pub fn edge_facts() -> ReleaseEdgeFacts {
    ReleaseEdgeFacts {
        crate_name: "example".to_owned(),
        cargo_rel_path: "crates/example/Cargo.toml".to_owned(),
        dep_name: "dep".to_owned(),
        section_label: "dependencies".to_owned(),
        target_label: None,
        has_path: true,
        dep_publishable: true,
        version_req: Some("1.0".to_owned()),
        actual_version: Some("1.2.3".to_owned()),
        version_satisfied: Some(true),
    }
}

pub fn failure(rel_path: &str, message: &str) -> ReleaseInputFailureFacts {
    ReleaseInputFailureFacts {
        rel_path: rel_path.to_owned(),
        message: message.to_owned(),
    }
}

pub fn repo_input<'a>(repo: &'a RepoReleaseFacts) -> RepoReleaseInput<'a> {
    RepoReleaseInput::new(repo)
}

pub fn crate_input<'a>(krate: &'a PublishableCrateFacts) -> PublishableCrateReleaseInput<'a> {
    PublishableCrateReleaseInput::new(krate)
}

pub fn edge_input<'a>(edge: &'a ReleaseEdgeFacts) -> ReleaseEdgeInput<'a> {
    ReleaseEdgeInput::new(edge)
}

pub fn failure_input<'a>(failure: &'a ReleaseInputFailureFacts) -> ReleaseInputFailureInput<'a> {
    ReleaseInputFailureInput::new(failure)
}
