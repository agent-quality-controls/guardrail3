use std::collections::{BTreeMap, BTreeSet};

use serde_yaml::Value as YamlValue;

use crate::domain::project_tree::ProjectTree;
use crate::ports::outbound::{CommandRunResult, ToolChecker};

use super::release_support::{
    binary_release_present, dependency_edges, extract_workflow_analysis, is_binary_crate,
    is_library_crate, is_publishable, join_under_root, linux_target_present, package_table,
    path_file_exists, publish_dry_run_step_present, publish_setting_string, readme_target_path,
    registry_token_present, release_plz_step_present, string_field_present, valid_semver,
    version_requirement_satisfied,
};

#[derive(Debug, Clone)]
pub struct RepoReleaseFacts {
    pub cargo_rel_path: String,
    pub license_rel_path: Option<String>,
    pub release_plz_rel_path: String,
    pub release_plz_exists: bool,
    pub release_plz_parsed: Option<toml::Value>,
    pub release_plz_has_workspace: bool,
    pub release_plz_package_names: BTreeSet<String>,
    pub cliff_rel_path: String,
    pub cliff_exists: bool,
    pub workflows: Vec<WorkflowFacts>,
    pub publishable_crate_names: BTreeSet<String>,
    pub publishable_count: usize,
    pub non_publishable_count: usize,
    pub semver_checks_installed: bool,
    pub publish_setting: Option<String>,
    pub release_profile_settings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorkflowFacts {
    pub rel_path: String,
    pub has_release_plz_step: bool,
    pub has_publish_dry_run_step: bool,
    pub has_registry_token: bool,
    pub has_binary_release: bool,
    pub has_linux_target: bool,
}

#[derive(Debug, Clone)]
pub struct PublishableCrateFacts {
    pub name: String,
    pub cargo_rel_path: String,
    pub publishable: bool,
    pub is_binary: bool,
    pub is_library: bool,
    pub description_present: bool,
    pub license_present: bool,
    pub repository_present: bool,
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
struct CargoRootFacts {
    rel_dir: String,
    cargo_rel_path: String,
    parsed: toml::Value,
    has_workspace: bool,
    has_package: bool,
    workspace_dependencies: toml::map::Map<String, toml::Value>,
}

pub fn collect(tree: &ProjectTree, tc: &dyn ToolChecker, thorough: bool) -> ReleaseFacts {
    let mut input_failures = Vec::new();
    let cargo_roots = collect_cargo_roots(tree, &mut input_failures);
    let workspace_roots: Vec<_> = cargo_roots
        .values()
        .filter(|root| root.has_workspace)
        .map(|root| root.rel_dir.clone())
        .collect();
    let mut crates = Vec::new();
    let mut version_map = BTreeMap::new();
    let mut publishable_names = BTreeSet::new();

    for root in cargo_roots.values().filter(|root| root.has_package) {
        let package = package_table(&root.parsed);
        let publishable = is_publishable(package);
        let is_binary = is_binary_crate(tree, &root.rel_dir, &root.parsed);
        let is_library = is_library_crate(tree, &root.rel_dir, &root.parsed);
        let name = package
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or("unknown")
            .to_owned();
        let readme_field = package
            .and_then(|package| package.get("readme"))
            .and_then(toml::Value::as_str);
        let (readme_rel_path, readme_abs_path) = readme_target_path(tree, &root.rel_dir, readme_field);
        let readme_exists = path_file_exists(&readme_abs_path);
        let readme_content = if readme_exists {
            match crate::fs::read_file_err(&readme_abs_path) {
                Ok(content) => Some(content),
                Err(read_error) => {
                    input_failures.push(ReleaseInputFailureFacts {
                        rel_path: readme_rel_path.clone(),
                        message: format!("Failed to read README for release checks: {read_error}"),
                    });
                    None
                }
            }
        } else {
            None
        };
        let version_value = package.and_then(|package| package.get("version"));
        let workspace_version = version_value
            .and_then(toml::Value::as_table)
            .and_then(|table| table.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        let version_string = version_value
            .and_then(toml::Value::as_str)
            .map(str::to_owned);
        let version_valid = workspace_version
            || version_string
                .as_deref()
                .is_some_and(valid_semver);
        let facts = PublishableCrateFacts {
            name: name.clone(),
            cargo_rel_path: root.cargo_rel_path.clone(),
            publishable,
            is_binary,
            is_library,
            description_present: string_field_present(package, "description"),
            license_present: string_field_present(package, "license")
                || string_field_present(package, "license-file"),
            repository_present: string_field_present(package, "repository"),
            readme_rel_path,
            readme_exists,
            readme_content,
            keywords_count: package
                .and_then(|package| package.get("keywords"))
                .and_then(toml::Value::as_array)
                .map(Vec::len),
            categories_count: package
                .and_then(|package| package.get("categories"))
                .and_then(toml::Value::as_array)
                .map(Vec::len),
            version_string: version_string.clone(),
            workspace_version,
            version_valid,
            docs_rs_present: package
                .and_then(|package| package.get("metadata"))
                .and_then(|metadata| metadata.get("docs.rs"))
                .is_some(),
            include_exclude_present: package
                .is_some_and(|package| package.get("include").is_some() || package.get("exclude").is_some()),
            has_binstall_metadata: package
                .and_then(|package| package.get("metadata"))
                .and_then(|metadata| metadata.get("binstall"))
                .is_some(),
            dry_run: if publishable && thorough {
                tc.run_cargo_publish_dry_run_outcome(&tree.abs_path(&root.rel_dir))
            } else {
                None
            },
        };
        if publishable {
            let _ = publishable_names.insert(name.clone());
            if let Some(version) = version_string {
                let _ = version_map.insert(name.clone(), version);
            }
        }
        crates.push(facts);
    }

    let mut release_plz_parsed = None;
    let mut release_plz_exists = false;
    let mut release_plz_has_workspace = false;
    let mut release_plz_package_names = BTreeSet::new();
    let release_plz_rel_path = "release-plz.toml".to_owned();
    if let Some(content) = tree.file_content(&release_plz_rel_path) {
        release_plz_exists = true;
        match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => {
                release_plz_has_workspace = parsed.get("workspace").is_some();
                release_plz_package_names = parsed
                    .get("package")
                    .and_then(toml::Value::as_array)
                    .map(|entries| {
                        entries
                            .iter()
                            .filter_map(|entry| entry.get("name"))
                            .filter_map(toml::Value::as_str)
                            .map(str::to_owned)
                            .collect::<BTreeSet<_>>()
                    })
                    .unwrap_or_default();
                release_plz_parsed = Some(parsed);
            }
            Err(parse_error) => input_failures.push(ReleaseInputFailureFacts {
                rel_path: release_plz_rel_path.clone(),
                message: format!("Failed to parse release-plz.toml: {parse_error}"),
            }),
        }
    }

    let cliff_rel_path = "cliff.toml".to_owned();
    let cliff_exists = tree.file_content(&cliff_rel_path).is_some();
    if let Some(content) = tree.file_content(&cliff_rel_path) {
        if let Err(parse_error) = toml::from_str::<toml::Value>(content) {
            input_failures.push(ReleaseInputFailureFacts {
                rel_path: cliff_rel_path.clone(),
                message: format!("Failed to parse cliff.toml: {parse_error}"),
            });
        }
    }

    let workflows = collect_workflows(tree, &mut input_failures);

    let root_cargo_rel_path = "Cargo.toml".to_owned();
    let release_profile_settings = cargo_roots
        .get("")
        .and_then(|root| root.parsed.get("profile"))
        .and_then(|profile| profile.get("release"))
        .and_then(toml::Value::as_table)
        .map(|table| {
            table
                .iter()
                .map(|(key, value)| format!("{key} = {value}"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let publish_setting = cargo_roots
        .get("")
        .and_then(|root| {
            root.parsed
                .get("workspace")
                .and_then(|workspace| workspace.get("package"))
                .and_then(Some)
                .or_else(|| root.parsed.get("package"))
        })
        .and_then(|table| publish_setting_string(Some(table)));

    let license_rel_path = ["LICENSE", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE.md"]
        .iter()
        .find(|name| tree.file_exists(name))
        .map(|name| (*name).to_owned());

    let repo = vec![RepoReleaseFacts {
        cargo_rel_path: root_cargo_rel_path,
        license_rel_path,
        release_plz_rel_path,
        release_plz_exists,
        release_plz_parsed,
        release_plz_has_workspace,
        release_plz_package_names,
        cliff_rel_path,
        cliff_exists,
        workflows,
        publishable_crate_names: publishable_names.clone(),
        publishable_count: crates.iter().filter(|krate| krate.publishable).count(),
        non_publishable_count: crates.iter().filter(|krate| !krate.publishable).count(),
        semver_checks_installed: tc.is_installed("cargo-semver-checks"),
        publish_setting,
        release_profile_settings,
    }];

    let mut edges = Vec::new();
    for root in cargo_roots.values().filter(|root| root.has_package) {
        let package = package_table(&root.parsed);
        let crate_name = package
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or("unknown")
            .to_owned();
        if !is_publishable(package) {
            continue;
        }
        let workspace_root = workspace_roots
            .iter()
            .filter(|workspace| root.rel_dir == **workspace || root.rel_dir.starts_with(&format!("{workspace}/")))
            .max_by_key(|workspace| workspace.len())
            .and_then(|workspace| cargo_roots.get(workspace));
        let workspace_dependencies = workspace_root
            .map(|workspace| &workspace.workspace_dependencies)
            .cloned()
            .unwrap_or_default();
        for edge in dependency_edges(&root.parsed, &workspace_dependencies) {
            let actual_version = version_map.get(&edge.dep_name).cloned();
            let dep_publishable = publishable_names.contains(&edge.dep_name);
            let version_satisfied = edge
                .version_req
                .as_deref()
                .zip(actual_version.as_deref())
                .map(|(req, actual)| version_requirement_satisfied(actual, req));
            edges.push(ReleaseEdgeFacts {
                crate_name: crate_name.clone(),
                cargo_rel_path: root.cargo_rel_path.clone(),
                dep_name: edge.dep_name,
                section_label: edge.section_label,
                target_label: edge.target_label,
                has_path: edge.has_path,
                dep_publishable,
                version_req: edge.version_req,
                actual_version,
                version_satisfied,
            });
        }
    }

    crates.sort_by(|a, b| a.cargo_rel_path.cmp(&b.cargo_rel_path));
    edges.sort_by(|a, b| {
        a.cargo_rel_path
            .cmp(&b.cargo_rel_path)
            .then(a.dep_name.cmp(&b.dep_name))
            .then(a.section_label.cmp(&b.section_label))
    });
    input_failures.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.message.cmp(&b.message)));

    ReleaseFacts {
        repo,
        crates,
        edges,
        input_failures,
    }
}

fn collect_cargo_roots(
    tree: &ProjectTree,
    input_failures: &mut Vec<ReleaseInputFailureFacts>,
) -> BTreeMap<String, CargoRootFacts> {
    let mut dirs = BTreeSet::new();
    if tree.file_exists("Cargo.toml") {
        let _ = dirs.insert(String::new());
    }
    dirs.extend(tree.dirs_with_file("Cargo.toml"));

    dirs.into_iter()
        .filter_map(|rel_dir| {
            let cargo_rel_path = if rel_dir.is_empty() {
                "Cargo.toml".to_owned()
            } else {
                join_under_root(&rel_dir, "Cargo.toml")
            };
            let content = tree.file_content(&cargo_rel_path)?;
            let parsed = match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => parsed,
                Err(parse_error) => {
                    input_failures.push(ReleaseInputFailureFacts {
                        rel_path: cargo_rel_path.clone(),
                        message: format!(
                            "Failed to parse Cargo.toml for release-family discovery: {parse_error}"
                        ),
                    });
                    return None;
                }
            };
            let workspace_dependencies = parsed
                .get("workspace")
                .and_then(|workspace| workspace.get("dependencies"))
                .and_then(toml::Value::as_table)
                .cloned()
                .unwrap_or_default();
            Some((
                rel_dir.clone(),
                CargoRootFacts {
                    rel_dir,
                    cargo_rel_path,
                    has_workspace: parsed.get("workspace").is_some(),
                    has_package: parsed.get("package").is_some(),
                    workspace_dependencies,
                    parsed,
                },
            ))
        })
        .collect()
}

fn collect_workflows(
    tree: &ProjectTree,
    input_failures: &mut Vec<ReleaseInputFailureFacts>,
) -> Vec<WorkflowFacts> {
    let mut workflows = tree
        .content
        .iter()
        .filter(|(rel_path, _)| rel_path.contains(".github/workflows/"))
        .filter_map(|(rel_path, content)| {
            let parsed = match serde_yaml::from_str::<YamlValue>(content) {
                Ok(parsed) => parsed,
                Err(parse_error) => {
                    input_failures.push(ReleaseInputFailureFacts {
                        rel_path: rel_path.clone(),
                        message: format!("Failed to parse workflow YAML: {parse_error}"),
                    });
                    return None;
                }
            };
            let analysis = extract_workflow_analysis(&parsed);
            Some(WorkflowFacts {
                rel_path: rel_path.clone(),
                has_release_plz_step: release_plz_step_present(&analysis),
                has_publish_dry_run_step: publish_dry_run_step_present(&analysis),
                has_registry_token: registry_token_present(&analysis),
                has_binary_release: binary_release_present(&analysis),
                has_linux_target: linux_target_present(&analysis),
            })
        })
        .collect::<Vec<_>>();
    workflows.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    workflows
}
