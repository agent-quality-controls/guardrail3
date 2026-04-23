use g3rs_release_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

pub(super) fn run(has_workflow: bool) -> Vec<G3CheckResult> {
    let mut input = input();
    input.has_release_plz_workflow = has_workflow;
    input.release_plz_workflow_rel_path =
        has_workflow.then(|| ".github/workflows/release.yml".to_owned());
    let mut results = Vec::new();
    super::super::check(&input, &mut results);
    results
}

fn input() -> G3RsReleaseConfigRepo {
    G3RsReleaseConfigRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: cargo_toml_parser::parse(
            r#"
[workspace]
members = ["crates/demo"]
resolver = "2"
"#,
        )
        .expect("repo cargo fixture should parse"),
        release_plz_rel_path: "release-plz.toml".to_owned(),
        release_plz_exists: true,
        release_plz: None,
        cliff_rel_path: "cliff.toml".to_owned(),
        cliff_exists: false,
        cliff: None,
        workflows: Vec::new(),
        has_release_plz_workflow: false,
        release_plz_workflow_rel_path: None,
        has_publish_dry_run_workflow: false,
        publish_dry_run_workflow_rel_path: None,
        has_registry_token_workflow: false,
        registry_token_workflow_rel_path: None,
        semver_checks_installed: false,
    }
}
