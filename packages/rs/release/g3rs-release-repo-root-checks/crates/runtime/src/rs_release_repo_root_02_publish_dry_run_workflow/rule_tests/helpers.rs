use std::collections::BTreeSet;

use g3rs_release_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

pub(super) fn run(has_workflow: bool) -> Vec<G3CheckResult> {
    let mut input = input();
    input.has_publish_dry_run_workflow = has_workflow;
    input.publish_dry_run_workflow_rel_path =
        has_workflow.then(|| ".github/workflows/release.yml".to_owned());
    let mut results = Vec::new();
    super::super::check(&input, &mut results);
    results
}

fn input() -> G3RsReleaseConfigRepo {
    G3RsReleaseConfigRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        release_plz_rel_path: "release-plz.toml".to_owned(),
        release_plz_exists: true,
        release_plz: None,
        release_plz_package_names: BTreeSet::new(),
        cliff_rel_path: "cliff.toml".to_owned(),
        cliff_exists: false,
        cliff: None,
        has_release_plz_workflow: false,
        release_plz_workflow_rel_path: None,
        has_publish_dry_run_workflow: false,
        publish_dry_run_workflow_rel_path: None,
        has_registry_token_workflow: false,
        registry_token_workflow_rel_path: None,
        publishable_crate_names: BTreeSet::new(),
        publishable_binary_crate_names: BTreeSet::new(),
        publishable_count: 0,
        non_publishable_count: 0,
        semver_checks_installed: false,
        publish_setting: None,
        release_profile_settings: Vec::new(),
    }
}
