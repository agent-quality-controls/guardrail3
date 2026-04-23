use std::collections::{BTreeMap, BTreeSet};

use cargo_toml_parser::types::{CargoToml, InheritableValue, WorkspacePackageSection};
use g3rs_release_config_checks_assertions::run as run_assertions;
use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseConfigCrate, G3RsReleaseConfigEdge,
    G3RsReleaseConfigRepo, G3RsReleaseInputFailure, G3RsReleasePathTargetKind, G3RsReleaseWorkflow,
    G3RsReleaseWorkflowAnalysis, G3RsReleaseWorkflowJob, G3RsReleaseWorkflowStep,
};
use guardrail3_check_types::G3Severity;

#[test]
fn dispatches_grouped_config_surfaces() {
    let mut repo_input = config_input_for_repo(None, None);
    repo_input.repo_checks[0].semver_checks_installed = true;
    let crate_input = config_input_for_crate(
        r#"
[package]
name = "demo"
version = "0.1.0"
"#,
        None,
    );
    let publishable_input = config_input_for_publishable_crate(
        r#"
[package]
name = "demo"
version = "0.1.0"
publish = true
"#,
        None,
    );
    let edge_source = publishable_input
        .crate_checks
        .first()
        .expect("source crate fixture should exist")
        .clone();
    let mut crate_checks = crate_input.crate_checks;
    crate_checks.extend(publishable_input.crate_checks);

    let results = crate::run::check(&G3RsReleaseConfigChecksInput {
        repo_checks: repo_input.repo_checks,
        crate_checks,
        edge_checks: vec![G3RsReleaseConfigEdge {
            source: edge_source,
            dep_name: "private".to_owned(),
            dep_package_name: "private".to_owned(),
            section_label: "dependencies".to_owned(),
            target_label: None,
            has_path: true,
            path_target_kind: Some(G3RsReleasePathTargetKind::InWorkspace),
            version_req: None,
            target: None,
        }],
        input_failure_checks: vec![G3RsReleaseInputFailure {
            rel_path: "Cargo.toml".to_owned(),
            message: "config failure".to_owned(),
        }],
    });

    run_assertions::assert_result_id_count(&results, "RS-RELEASE-CONFIG-00", 1);
    run_assertions::assert_result_id_count(&results, "RS-RELEASE-CONFIG-15", 1);
    run_assertions::assert_result_id_count(&results, "RS-RELEASE-CONFIG-19", 1);
    run_assertions::assert_result_id_count(&results, "RS-RELEASE-CONFIG-25", 1);
}

#[test]
fn manifest_path_matching_stays_on_the_target_crate() {
    let repo = G3RsReleaseConfigRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: cargo_toml_parser::parse(
            r#"
[workspace]
members = ["crates/cli", "crates/tool"]
resolver = "2"
"#,
        )
        .expect("repo cargo fixture should parse"),
        release_plz_rel_path: "release-plz.toml".to_owned(),
        release_plz_exists: false,
        release_plz: None,
        cliff_rel_path: "cliff.toml".to_owned(),
        cliff_exists: false,
        cliff: None,
        workflows: vec![G3RsReleaseWorkflow {
            rel_path: ".github/workflows/release-binaries.yml".to_owned(),
            analysis: G3RsReleaseWorkflowAnalysis {
                env_keys: vec![],
                env_bindings: BTreeMap::new(),
                jobs: vec![
                    G3RsReleaseWorkflowJob {
                        id: "build".to_owned(),
                        runs_on: vec!["ubuntu-latest".to_owned()],
                        needs: vec![],
                        matrix_axes: BTreeMap::new(),
                        env_keys: vec![],
                        env_bindings: BTreeMap::new(),
                        steps: vec![G3RsReleaseWorkflowStep {
                            uses: None,
                            run_lines: vec![
                                "cargo build --release --manifest-path crates/tool/Cargo.toml --target x86_64-unknown-linux-gnu".to_owned(),
                            ],
                            env_keys: vec![],
                            env_bindings: BTreeMap::new(),
                            with_bindings: BTreeMap::new(),
                        }],
                    },
                    G3RsReleaseWorkflowJob {
                        id: "release".to_owned(),
                        runs_on: vec!["ubuntu-latest".to_owned()],
                        needs: vec!["build".to_owned()],
                        matrix_axes: BTreeMap::new(),
                        env_keys: vec![],
                        env_bindings: BTreeMap::new(),
                        steps: vec![G3RsReleaseWorkflowStep {
                            uses: Some("softprops/action-gh-release@v2".to_owned()),
                            run_lines: vec![],
                            env_keys: vec![],
                            env_bindings: BTreeMap::new(),
                            with_bindings: BTreeMap::new(),
                        }],
                    },
                ],
                steps: vec![],
            },
        }],
        has_release_plz_workflow: false,
        release_plz_workflow_rel_path: None,
        has_publish_dry_run_workflow: false,
        publish_dry_run_workflow_rel_path: None,
        has_registry_token_workflow: false,
        registry_token_workflow_rel_path: None,
        semver_checks_installed: false,
    };

    let cli = build_crate(
        "crates/cli/Cargo.toml",
        cargo_toml_parser::parse(
            r#"
[package]
name = "cli"
version = "0.1.0"
publish = true

[[bin]]
name = "cli"
path = "src/main.rs"
"#,
        )
        .expect("cli cargo fixture should parse"),
        None,
    );
    let tool = build_crate(
        "crates/tool/Cargo.toml",
        cargo_toml_parser::parse(
            r#"
[package]
name = "tool"
version = "0.1.0"
publish = true

[[bin]]
name = "tool"
path = "src/main.rs"
"#,
        )
        .expect("tool cargo fixture should parse"),
        None,
    );

    let results = crate::run::check(&G3RsReleaseConfigChecksInput {
        repo_checks: vec![repo],
        crate_checks: vec![cli, tool],
        edge_checks: Vec::new(),
        input_failure_checks: Vec::new(),
    });

    run_assertions::assert_contains_result(
        &results,
        "RS-RELEASE-CONFIG-23",
        G3Severity::Info,
        "tool: binary release workflow present",
    );
    run_assertions::assert_contains_result(
        &results,
        "RS-RELEASE-CONFIG-24",
        G3Severity::Info,
        "tool: linux release target present",
    );
    run_assertions::assert_contains_result(
        &results,
        "RS-RELEASE-CONFIG-23",
        G3Severity::Info,
        "cli: no binary release workflow",
    );
    run_assertions::assert_contains_result(
        &results,
        "RS-RELEASE-CONFIG-24",
        G3Severity::Info,
        "cli: no linux release target",
    );
}

#[test]
fn manifest_path_matching_does_not_credit_other_binary_crates() {
    let repo = G3RsReleaseConfigRepo {
        cargo_rel_path: "Cargo.toml".to_owned(),
        cargo: cargo_toml_parser::parse(
            r#"
[workspace]
members = ["crates/cli", "crates/tool"]
resolver = "2"
"#,
        )
        .expect("repo cargo fixture should parse"),
        release_plz_rel_path: "release-plz.toml".to_owned(),
        release_plz_exists: false,
        release_plz: None,
        cliff_rel_path: "cliff.toml".to_owned(),
        cliff_exists: false,
        cliff: None,
        workflows: vec![G3RsReleaseWorkflow {
            rel_path: ".github/workflows/release-binaries.yml".to_owned(),
            analysis: G3RsReleaseWorkflowAnalysis {
                env_keys: vec![],
                env_bindings: BTreeMap::new(),
                jobs: vec![
                    G3RsReleaseWorkflowJob {
                        id: "build".to_owned(),
                        runs_on: vec!["ubuntu-latest".to_owned()],
                        needs: vec![],
                        matrix_axes: BTreeMap::new(),
                        env_keys: vec![],
                        env_bindings: BTreeMap::new(),
                        steps: vec![
                            G3RsReleaseWorkflowStep {
                                uses: Some("taiki-e/upload-rust-binary-action@v1".to_owned()),
                                run_lines: vec![],
                                env_keys: vec![],
                                env_bindings: BTreeMap::new(),
                                with_bindings: BTreeMap::new(),
                            },
                            G3RsReleaseWorkflowStep {
                                uses: None,
                                run_lines: vec![
                                    "cargo build --release --manifest-path crates/tool/Cargo.toml --target x86_64-unknown-linux-gnu".to_owned(),
                                ],
                                env_keys: vec![],
                                env_bindings: BTreeMap::new(),
                                with_bindings: BTreeMap::new(),
                            },
                        ],
                    },
                ],
                steps: vec![],
            },
        }],
        has_release_plz_workflow: false,
        release_plz_workflow_rel_path: None,
        has_publish_dry_run_workflow: false,
        publish_dry_run_workflow_rel_path: None,
        has_registry_token_workflow: false,
        registry_token_workflow_rel_path: None,
        semver_checks_installed: false,
    };
    let cli = build_crate(
        "crates/cli/Cargo.toml",
        cargo_toml_parser::parse(
            r#"
[package]
name = "cli"
version = "0.1.0"
publish = true

[[bin]]
name = "cli"
path = "src/main.rs"
"#,
        )
        .expect("cli cargo fixture should parse"),
        None,
    );
    let tool = build_crate(
        "crates/tool/Cargo.toml",
        cargo_toml_parser::parse(
            r#"
[package]
name = "tool"
version = "0.1.0"
publish = false

[[bin]]
name = "tool"
path = "src/main.rs"
"#,
        )
        .expect("tool cargo fixture should parse"),
        None,
    );

    let results = crate::run::check(&G3RsReleaseConfigChecksInput {
        repo_checks: vec![repo],
        crate_checks: vec![cli, tool],
        edge_checks: Vec::new(),
        input_failure_checks: Vec::new(),
    });

    run_assertions::assert_contains_result(
        &results,
        "RS-RELEASE-CONFIG-23",
        G3Severity::Info,
        "cli: no binary release workflow",
    );
    run_assertions::assert_contains_result(
        &results,
        "RS-RELEASE-CONFIG-24",
        G3Severity::Info,
        "cli: no linux release target",
    );
}

#[test]
fn description_rule_reads_parsed_cargo_surface() {
    let input = config_input_for_publishable_crate(
        r#"
[package]
name = "demo"
version = "0.1.0"
publish = true
"#,
        None,
    );

    let results = crate::run::check(&input);

    run_assertions::assert_contains_result(
        &results,
        "RS-RELEASE-CONFIG-01",
        G3Severity::Error,
        "demo: missing description",
    );
}

#[test]
fn docs_rs_rule_reads_parsed_cargo_surface() {
    let input = config_input_for_publishable_crate(
        r#"
[package]
name = "demo"
version = "0.1.0"
publish = true

[lib]
path = "src/lib.rs"
"#,
        None,
    );

    let results = crate::run::check(&input);

    run_assertions::assert_contains_result(
        &results,
        "RS-RELEASE-CONFIG-07",
        G3Severity::Warn,
        "demo: docs.rs metadata missing",
    );
}

fn config_input_for_crate(
    cargo_toml: &str,
    workspace_cargo_toml: Option<&str>,
) -> G3RsReleaseConfigChecksInput {
    let cargo = cargo_toml_parser::parse(cargo_toml).expect("cargo fixture should parse");
    let workspace_package = workspace_package(workspace_cargo_toml);

    G3RsReleaseConfigChecksInput {
        repo_checks: Vec::new(),
        crate_checks: vec![build_crate("Cargo.toml", cargo, workspace_package)],
        edge_checks: Vec::new(),
        input_failure_checks: Vec::new(),
    }
}

fn config_input_for_publishable_crate(
    cargo_toml: &str,
    workspace_cargo_toml: Option<&str>,
) -> G3RsReleaseConfigChecksInput {
    let mut cargo = cargo_toml_parser::parse(cargo_toml).expect("cargo fixture should parse");
    if let Some(package) = cargo.package.as_mut()
        && package.publish.is_none()
    {
        package.publish = Some(InheritableValue::Value(
            cargo_toml_parser::types::VecStringOrBool::Bool(true),
        ));
    }

    G3RsReleaseConfigChecksInput {
        repo_checks: Vec::new(),
        crate_checks: vec![build_crate(
            "Cargo.toml",
            cargo,
            workspace_package(workspace_cargo_toml),
        )],
        edge_checks: Vec::new(),
        input_failure_checks: Vec::new(),
    }
}

fn config_input_for_repo(
    release_plz_toml: Option<&str>,
    cliff_toml: Option<&str>,
) -> G3RsReleaseConfigChecksInput {
    let release_plz = release_plz_toml.map(|value| {
        release_plz_toml_parser::parse(value).expect("release-plz fixture should parse")
    });
    let cliff = cliff_toml
        .map(|value| cliff_toml_parser::parse(value).expect("cliff fixture should parse"));
    let publishable_name = release_plz
        .as_ref()
        .and_then(|release_plz| release_plz.package.first())
        .and_then(|package| package.name.as_deref())
        .unwrap_or("demo")
        .to_owned();

    G3RsReleaseConfigChecksInput {
        repo_checks: vec![G3RsReleaseConfigRepo {
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
            release_plz_exists: release_plz.is_some(),
            release_plz,
            cliff_rel_path: "cliff.toml".to_owned(),
            cliff_exists: cliff.is_some(),
            cliff,
            workflows: Vec::new(),
            has_release_plz_workflow: false,
            release_plz_workflow_rel_path: None,
            has_publish_dry_run_workflow: false,
            publish_dry_run_workflow_rel_path: None,
            has_registry_token_workflow: false,
            registry_token_workflow_rel_path: None,
            semver_checks_installed: false,
        }],
        crate_checks: vec![publishable_crate(&publishable_name)],
        edge_checks: Vec::new(),
        input_failure_checks: Vec::new(),
    }
}

fn publishable_crate(name: &str) -> G3RsReleaseConfigCrate {
    build_crate(
        &format!("crates/{name}/Cargo.toml"),
        cargo_toml_parser::parse(&format!(
            r#"
[package]
name = "{name}"
version = "0.1.0"
publish = true
"#
        ))
        .expect("publishable cargo fixture should parse"),
        None,
    )
}

fn workspace_package(workspace_cargo_toml: Option<&str>) -> Option<WorkspacePackageSection> {
    workspace_cargo_toml
        .map(|workspace| {
            cargo_toml_parser::parse(workspace).expect("workspace cargo fixture should parse")
        })
        .and_then(|workspace| workspace.workspace.and_then(|section| section.package))
}

fn build_crate(
    cargo_rel_path: &str,
    cargo: CargoToml,
    workspace_package: Option<WorkspacePackageSection>,
) -> G3RsReleaseConfigCrate {
    let name = cargo
        .package
        .as_ref()
        .and_then(|package| package.name.clone())
        .unwrap_or_else(|| cargo_rel_path.to_owned());

    G3RsReleaseConfigCrate {
        name,
        cargo_rel_path: cargo_rel_path.to_owned(),
        cargo: cargo.clone(),
        workspace_package,
        is_binary: !cargo.bin.is_empty(),
        is_library: cargo.lib.is_some(),
        binary_target_names: cargo
            .bin
            .iter()
            .filter_map(|target| target.name.clone())
            .collect::<BTreeSet<_>>(),
        dry_run: None,
    }
}
