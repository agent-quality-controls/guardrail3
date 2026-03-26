use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use glob::Pattern;
use serde_yaml::Value as YamlValue;

use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::{CommandRunResult, ToolChecker};

use super::release_support::{
    WorkflowAnalysis, binary_target_names, dependency_edges, extract_workflow_analysis,
    is_binary_crate, is_library_crate, is_publishable, join_under_root, package_table,
    path_file_exists, publish_setting_string, readme_target_path, string_field_present,
    valid_semver, version_requirement_satisfied,
};

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
struct CargoRootFacts {
    rel_dir: String,
    cargo_rel_path: String,
    parsed: toml::Value,
    has_workspace: bool,
    has_package: bool,
    workspace_members: Vec<String>,
    workspace_exclude: Vec<String>,
    workspace_dependencies: toml::map::Map<String, toml::Value>,
    package_workspace: Option<String>,
}

pub fn collect(tree: &ProjectTree, tc: &dyn ToolChecker, thorough: bool) -> ReleaseFacts {
    let mut input_failures = Vec::new();
    let cargo_roots = collect_cargo_roots(tree, &mut input_failures);
    let mut crates = Vec::new();
    let mut version_map = BTreeMap::new();
    let mut publishable_names = BTreeSet::new();
    let mut publishable_binary_names = BTreeSet::new();

    for root in cargo_roots.values().filter(|root| root.has_package) {
        let package = package_table(&root.parsed);
        let workspace_root = workspace_root_for_package(root, &cargo_roots);
        let workspace_package = workspace_root
            .and_then(|workspace_root| workspace_root.parsed.get("workspace"))
            .and_then(|workspace| workspace.get("package"));
        let publishable = inherited_publishable(package, workspace_package);
        let is_binary = is_binary_crate(tree, &root.rel_dir, &root.parsed);
        let binary_target_names = binary_target_names(tree, &root.rel_dir, &root.parsed);
        let is_library = is_library_crate(tree, &root.rel_dir, &root.parsed);
        let name = package
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or("unknown")
            .to_owned();
        let readme_declared_false = inherited_readme_declared_false(package, workspace_package);
        let (readme_path_field, readme_from_workspace) =
            inherited_readme_path(package, workspace_package);
        let readme_base_rel_dir = if readme_from_workspace {
            workspace_root.map_or(root.rel_dir.as_str(), |workspace_root| {
                workspace_root.rel_dir.as_str()
            })
        } else {
            root.rel_dir.as_str()
        };
        let (readme_rel_path, readme_abs_path) =
            readme_target_path(tree, readme_base_rel_dir, readme_path_field);
        let readme_exists = !readme_declared_false && path_file_exists(&readme_abs_path);
        let readme_content = if publishable && readme_exists {
            match guardrail3_shared_fs::read_file_err(&readme_abs_path) {
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
        let version_string = inherited_version_string(package, workspace_package);
        let version_valid = version_string.as_deref().is_some_and(valid_semver);
        let facts = PublishableCrateFacts {
            name: name.clone(),
            cargo_rel_path: root.cargo_rel_path.clone(),
            binary_target_names,
            publishable,
            is_binary,
            is_library,
            description_present: inherited_string_field_present(
                package,
                workspace_package,
                "description",
            ),
            license_present: inherited_license_present(package, workspace_package),
            repository_present: inherited_string_field_present(
                package,
                workspace_package,
                "repository",
            ),
            readme_declared_false,
            readme_rel_path,
            readme_exists,
            readme_content,
            keywords_count: inherited_array_count(package, workspace_package, "keywords"),
            categories_count: inherited_array_count(package, workspace_package, "categories"),
            version_string: version_string.clone(),
            workspace_version,
            version_valid,
            docs_rs_present: package
                .and_then(|package| package.get("metadata"))
                .and_then(docs_rs_table)
                .is_some_and(has_supported_docs_rs_settings),
            include_exclude_present: package.is_some_and(has_include_or_exclude_patterns),
            has_binstall_metadata: package
                .and_then(|package| package.get("metadata"))
                .and_then(|metadata| metadata.get("binstall"))
                .and_then(toml::Value::as_table)
                .is_some(),
            dry_run: if publishable && thorough {
                tc.run_cargo_publish_dry_run_outcome(&tree.abs_path(&root.rel_dir))
            } else {
                None
            },
        };
        if publishable {
            let _ = publishable_names.insert(name.clone());
            if is_binary {
                let _ = publishable_binary_names.insert(name.clone());
            }
            if let Some(version) = version_string.clone() {
                let _ = version_map.insert(name.clone(), version);
            }
        }
        crates.push(facts);
    }

    let mut release_plz_parsed = None;
    let mut release_plz_exists = false;
    let mut release_plz_package_names = BTreeSet::new();
    let release_plz_rel_path = "release-plz.toml".to_owned();
    if tree.file_exists(&release_plz_rel_path) {
        release_plz_exists = true;
        if let Some(content) = tree.file_content(&release_plz_rel_path) {
            match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => {
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
        } else {
            input_failures.push(ReleaseInputFailureFacts {
                rel_path: release_plz_rel_path.clone(),
                message: "Failed to read release-plz.toml.".to_owned(),
            });
        }
    }

    let cliff_rel_path = "cliff.toml".to_owned();
    let cliff_exists = tree.file_exists(&cliff_rel_path);
    let mut cliff_parsed = None;
    if cliff_exists {
        if let Some(content) = tree.file_content(&cliff_rel_path) {
            match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => cliff_parsed = Some(parsed),
                Err(parse_error) => {
                    input_failures.push(ReleaseInputFailureFacts {
                        rel_path: cliff_rel_path.clone(),
                        message: format!("Failed to parse cliff.toml: {parse_error}"),
                    });
                }
            }
        } else {
            input_failures.push(ReleaseInputFailureFacts {
                rel_path: cliff_rel_path.clone(),
                message: "Failed to read cliff.toml.".to_owned(),
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
        release_plz_package_names,
        cliff_rel_path,
        cliff_exists,
        cliff_parsed,
        workflows,
        publishable_crate_names: publishable_names.clone(),
        publishable_binary_crate_names: publishable_binary_names,
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
        let workspace_root = workspace_root_for_package(root, &cargo_roots);
        let workspace_package = workspace_root
            .and_then(|workspace_root| workspace_root.parsed.get("workspace"))
            .and_then(|workspace| workspace.get("package"));
        if !inherited_publishable(package, workspace_package) {
            continue;
        }
        let workspace_dependencies = workspace_root
            .map(|workspace| &workspace.workspace_dependencies)
            .cloned()
            .unwrap_or_default();
        for edge in dependency_edges(&root.parsed, &workspace_dependencies) {
            let actual_version = version_map.get(&edge.dep_package_name).cloned();
            let dep_publishable = publishable_names.contains(&edge.dep_package_name);
            let version_satisfied = edge
                .version_req
                .as_deref()
                .zip(actual_version.as_deref())
                .map(|(req, actual)| version_requirement_satisfied(actual, req));
            edges.push(ReleaseEdgeFacts {
                crate_name: crate_name.clone(),
                cargo_rel_path: root.cargo_rel_path.clone(),
                dep_name: edge.dep_name,
                dep_package_name: edge.dep_package_name,
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
            let Some(content) = tree.file_content(&cargo_rel_path) else {
                input_failures.push(ReleaseInputFailureFacts {
                    rel_path: cargo_rel_path.clone(),
                    message: "Failed to read Cargo.toml for release-family discovery.".to_owned(),
                });
                return None;
            };
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
            let workspace_members = parsed
                .get("workspace")
                .and_then(|workspace| workspace.get("members"))
                .and_then(toml::Value::as_array)
                .map(|members| {
                    members
                        .iter()
                        .filter_map(toml::Value::as_str)
                        .map(str::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let workspace_exclude = parsed
                .get("workspace")
                .and_then(|workspace| workspace.get("exclude"))
                .and_then(toml::Value::as_array)
                .map(|exclude| {
                    exclude
                        .iter()
                        .filter_map(toml::Value::as_str)
                        .map(str::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let package_workspace = parsed
                .get("package")
                .and_then(|package| package.get("workspace"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned);
            Some((
                rel_dir.clone(),
                CargoRootFacts {
                    rel_dir,
                    cargo_rel_path,
                    has_workspace: parsed.get("workspace").is_some(),
                    has_package: parsed.get("package").is_some(),
                    workspace_members,
                    workspace_exclude,
                    workspace_dependencies,
                    package_workspace,
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
    let mut workflow_paths = tree
        .structure
        .iter()
        .flat_map(|(dir_rel, entry)| {
            entry.files.iter().filter_map(move |file_name| {
                let rel_path = ProjectTree::join_rel(dir_rel, file_name);
                let is_workflow = rel_path.starts_with(".github/workflows/")
                    && Path::new(file_name)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .is_some_and(|ext| {
                            ext.eq_ignore_ascii_case("yml") || ext.eq_ignore_ascii_case("yaml")
                        });
                is_workflow.then_some(rel_path)
            })
        })
        .collect::<Vec<_>>();
    workflow_paths.sort();

    let mut workflows = workflow_paths
        .into_iter()
        .filter_map(|rel_path| {
            let Some(content) = tree.file_content(&rel_path) else {
                input_failures.push(ReleaseInputFailureFacts {
                    rel_path: rel_path.clone(),
                    message: "Failed to read workflow YAML.".to_owned(),
                });
                return None;
            };
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
                analysis,
            })
        })
        .collect::<Vec<_>>();
    workflows.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    workflows
}

fn workspace_root_for_package<'a>(
    root: &CargoRootFacts,
    cargo_roots: &'a BTreeMap<String, CargoRootFacts>,
) -> Option<&'a CargoRootFacts> {
    if let Some(workspace_ref) = root.package_workspace.as_deref() {
        let workspace_rel_dir = normalize_rel_dir(join_rel_dir(&root.rel_dir, workspace_ref));
        return cargo_roots.get(&workspace_rel_dir).filter(|candidate| {
            candidate.has_workspace && workspace_contains_package(candidate, root)
        });
    }

    cargo_roots
        .values()
        .filter(|candidate| candidate.has_workspace && workspace_contains_package(candidate, root))
        .max_by_key(|candidate| candidate.rel_dir.len())
}

fn workspace_contains_package(
    workspace_root: &CargoRootFacts,
    package_root: &CargoRootFacts,
) -> bool {
    if package_root.rel_dir == workspace_root.rel_dir {
        return true;
    }
    if workspace_root.workspace_exclude.iter().any(|pattern| {
        workspace_member_pattern_matches(workspace_root, pattern, &package_root.rel_dir)
    }) {
        return false;
    }
    workspace_root.workspace_members.iter().any(|pattern| {
        workspace_member_pattern_matches(workspace_root, pattern, &package_root.rel_dir)
    })
}

fn workspace_member_pattern_matches(
    workspace_root: &CargoRootFacts,
    pattern: &str,
    package_rel_dir: &str,
) -> bool {
    let repo_pattern = normalize_rel_dir(join_rel_dir(&workspace_root.rel_dir, pattern));
    Pattern::new(&repo_pattern)
        .map(|pattern| pattern.matches(package_rel_dir))
        .unwrap_or(false)
}

fn join_rel_dir(base_rel_dir: &str, rel: &str) -> PathBuf {
    if base_rel_dir.is_empty() {
        PathBuf::from(rel)
    } else {
        Path::new(base_rel_dir).join(rel)
    }
}

fn normalize_rel_dir(path: PathBuf) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = parts.pop();
            }
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::RootDir | Component::Prefix(_) => {}
        }
    }
    parts.join("/")
}

fn inherited_string_field_present(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
    field: &str,
) -> bool {
    string_field_present(package, field)
        || package
            .and_then(|package| package.get(field))
            .and_then(toml::Value::as_table)
            .and_then(|table| table.get("workspace"))
            .and_then(toml::Value::as_bool)
            .is_some_and(|workspace| workspace)
            && string_field_present(workspace_package, field)
}

fn inherited_license_present(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
) -> bool {
    inherited_string_field_present(package, workspace_package, "license")
        || inherited_string_field_present(package, workspace_package, "license-file")
}

fn docs_rs_table<'a>(metadata: &'a toml::Value) -> Option<&'a toml::map::Map<String, toml::Value>> {
    metadata
        .get("docs.rs")
        .and_then(toml::Value::as_table)
        .or_else(|| {
            metadata
                .get("docs")
                .and_then(toml::Value::as_table)
                .and_then(|docs| docs.get("rs"))
                .and_then(toml::Value::as_table)
        })
}

fn has_supported_docs_rs_settings(table: &toml::map::Map<String, toml::Value>) -> bool {
    const SUPPORTED_KEYS: &[&str] = &[
        "all-features",
        "features",
        "no-default-features",
        "default-target",
        "targets",
        "rustdoc-args",
        "cargo-args",
    ];

    SUPPORTED_KEYS.iter().any(|key| table.contains_key(*key))
}

fn inherited_array_count(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
    field: &str,
) -> Option<usize> {
    package
        .and_then(|package| package.get(field))
        .and_then(toml::Value::as_array)
        .map(Vec::len)
        .or_else(|| {
            package
                .and_then(|package| package.get(field))
                .and_then(toml::Value::as_table)
                .and_then(|table| table.get("workspace"))
                .and_then(toml::Value::as_bool)
                .is_some_and(|workspace| workspace)
                .then(|| {
                    workspace_package
                        .and_then(|workspace_package| workspace_package.get(field))
                        .and_then(toml::Value::as_array)
                        .map(Vec::len)
                })
                .flatten()
        })
}

fn inherited_publishable(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
) -> bool {
    if !is_publishable(package) {
        return false;
    }
    let inherits = package
        .and_then(|package| package.get("publish"))
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    if !inherits {
        return true;
    }
    is_publishable(workspace_package)
}

fn inherited_readme_declared_false(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
) -> bool {
    package
        .and_then(|package| package.get("readme"))
        .and_then(toml::Value::as_bool)
        .is_some_and(|value| !value)
        || package
            .and_then(|package| package.get("readme"))
            .and_then(toml::Value::as_table)
            .and_then(|table| table.get("workspace"))
            .and_then(toml::Value::as_bool)
            .is_some_and(|workspace| workspace)
            && workspace_package
                .and_then(|workspace_package| workspace_package.get("readme"))
                .and_then(toml::Value::as_bool)
                .is_some_and(|value| !value)
}

fn inherited_readme_path<'a>(
    package: Option<&'a toml::Value>,
    workspace_package: Option<&'a toml::Value>,
) -> (Option<&'a str>, bool) {
    if let Some(local) = package
        .and_then(|package| package.get("readme"))
        .and_then(toml::Value::as_str)
    {
        return (Some(local), false);
    }
    let inherited = package
        .and_then(|package| package.get("readme"))
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    if !inherited {
        return (None, false);
    }
    (
        workspace_package
            .and_then(|workspace_package| workspace_package.get("readme"))
            .and_then(toml::Value::as_str),
        true,
    )
}

fn inherited_version_string(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
) -> Option<String> {
    let version_value = package.and_then(|package| package.get("version"));
    if let Some(version) = version_value.and_then(toml::Value::as_str) {
        return Some(version.to_owned());
    }
    let inherits = version_value
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    if !inherits {
        return None;
    }
    workspace_package
        .and_then(|workspace_package| workspace_package.get("version"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

fn has_include_or_exclude_patterns(package: &toml::Value) -> bool {
    has_pattern_entries(package.get("include")) || has_pattern_entries(package.get("exclude"))
}

fn has_pattern_entries(value: Option<&toml::Value>) -> bool {
    value
        .and_then(toml::Value::as_array)
        .is_some_and(|entries| !entries.is_empty())
}
