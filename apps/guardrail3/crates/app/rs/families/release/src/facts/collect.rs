use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::{cargo_roots, inheritance, types};
use crate::release_support;

pub fn collect(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    route: &guardrail3_app_rs_family_mapper::RsReleaseRoute,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> types::ReleaseFacts {
    let mut input_failures = Vec::new();
    let cargo_roots = cargo_roots::collect_cargo_roots(tree, route, &mut input_failures);
    let has_cargo_candidates = route
        .family_files()
        .iter()
        .any(|file| file.kind() == guardrail3_app_rs_ownership::RustFamilyFileKind::CargoToml);
    if cargo_roots.is_empty() && !has_cargo_candidates && input_failures.is_empty() {
        return types::ReleaseFacts::default();
    }
    let crate_index = collect_crate_index(tree, &cargo_roots);
    let mut crates = collect_crate_facts(
        tree,
        &cargo_roots,
        route.validation_scope(),
        tc,
        thorough,
        &mut input_failures,
    );

    let repo = vec![collect_repo_facts(
        tree,
        tc,
        &cargo_roots,
        crate_index.publishable_names.clone(),
        crate_index.publishable_binary_names.clone(),
        crate_index.publishable_count,
        crate_index.non_publishable_count,
        &mut input_failures,
    )];
    let edges = collect_release_edges(
        &cargo_roots,
        route.validation_scope(),
        &crate_index.version_map,
        &crate_index.publishable_names,
    );

    crates.sort_by(|a, b| a.cargo_rel_path.cmp(&b.cargo_rel_path));
    let mut edges = edges;
    edges.sort_by(|a, b| {
        a.cargo_rel_path
            .cmp(&b.cargo_rel_path)
            .then(a.dep_name.cmp(&b.dep_name))
            .then(a.section_label.cmp(&b.section_label))
    });
    input_failures.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.message.cmp(&b.message)));

    types::ReleaseFacts {
        repo,
        crates,
        edges,
        input_failures,
    }
}

struct CrateIndex {
    publishable_names: BTreeSet<String>,
    publishable_binary_names: BTreeSet<String>,
    publishable_count: usize,
    non_publishable_count: usize,
    version_map: BTreeMap<String, String>,
}

fn collect_crate_index(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    cargo_roots: &BTreeMap<String, types::CargoRootFacts>,
) -> CrateIndex {
    let mut publishable_names = BTreeSet::new();
    let mut publishable_binary_names = BTreeSet::new();
    let mut publishable_count = 0usize;
    let mut non_publishable_count = 0usize;
    let mut version_map = BTreeMap::new();

    for root in cargo_roots.values().filter(|root| root.has_package) {
        let package = release_support::binaries::package_table(&root.parsed);
        let workspace_root = cargo_roots::workspace_root_for_package(root, cargo_roots);
        let workspace_package = workspace_root
            .and_then(|workspace_root| workspace_root.parsed.get("workspace"))
            .and_then(|workspace| workspace.get("package"));
        let publishable = inheritance::inherited_publishable(package, workspace_package);
        if !publishable {
            non_publishable_count += 1;
            continue;
        }

        publishable_count += 1;
        let is_binary =
            release_support::binaries::is_binary_crate(tree, &root.rel_dir, &root.parsed);
        let name = package
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or("unknown")
            .to_owned();
        let _ = publishable_names.insert(name.clone());
        if is_binary {
            let _ = publishable_binary_names.insert(name.clone());
        }
        if let Some(version) = inheritance::inherited_version_string(package, workspace_package) {
            let _ = version_map.insert(name, version);
        }
    }

    CrateIndex {
        publishable_names,
        publishable_binary_names,
        publishable_count,
        non_publishable_count,
        version_map,
    }
}

fn collect_crate_facts(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    cargo_roots: &BTreeMap<String, types::CargoRootFacts>,
    validation_scope: Option<&str>,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
    input_failures: &mut Vec<types::ReleaseInputFailureFacts>,
) -> Vec<types::PublishableCrateFacts> {
    let mut crates = Vec::new();

    for root in cargo_roots
        .values()
        .filter(|root| root.has_package)
        .filter(|root| {
            validation_scope
                .is_none_or(|scope| rel_intersects_validation_scope(&root.rel_dir, scope))
        })
    {
        let package = release_support::binaries::package_table(&root.parsed);
        let workspace_root = cargo_roots::workspace_root_for_package(root, cargo_roots);
        let workspace_package = workspace_root
            .and_then(|workspace_root| workspace_root.parsed.get("workspace"))
            .and_then(|workspace| workspace.get("package"));
        let publishable = inheritance::inherited_publishable(package, workspace_package);
        let is_binary =
            release_support::binaries::is_binary_crate(tree, &root.rel_dir, &root.parsed);
        let binary_target_names =
            release_support::binaries::binary_target_names(tree, &root.rel_dir, &root.parsed);
        let is_library =
            release_support::binaries::is_library_crate(tree, &root.rel_dir, &root.parsed);
        let name = package
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or("unknown")
            .to_owned();
        let readme_declared_false =
            inheritance::inherited_readme_declared_false(package, workspace_package);
        let (readme_path_field, readme_from_workspace) =
            inheritance::inherited_readme_path(package, workspace_package);
        let readme_base_rel_dir = if readme_from_workspace {
            workspace_root.map_or(root.rel_dir.as_str(), |workspace_root| {
                workspace_root.rel_dir.as_str()
            })
        } else {
            root.rel_dir.as_str()
        };
        let (readme_rel_path, readme_abs_path) = release_support::binaries::readme_target_path(
            tree,
            readme_base_rel_dir,
            readme_path_field,
        );
        let readme_exists =
            !readme_declared_false && release_support::binaries::path_file_exists(&readme_abs_path);
        let readme_content = if publishable && readme_exists {
            match guardrail3_shared_fs::read_file_err(&readme_abs_path) {
                Ok(content) => Some(content),
                Err(read_error) => {
                    input_failures.push(types::ReleaseInputFailureFacts {
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
        let version_string = inheritance::inherited_version_string(package, workspace_package);
        let version_valid = inheritance::version_is_valid(version_string.as_deref());

        crates.push(types::PublishableCrateFacts {
            name,
            cargo_rel_path: root.cargo_rel_path.clone(),
            binary_target_names,
            publishable,
            is_binary,
            is_library,
            description_present: inheritance::inherited_string_field_present(
                package,
                workspace_package,
                "description",
            ),
            license_present: inheritance::inherited_license_present(package, workspace_package),
            repository_present: inheritance::inherited_string_field_present(
                package,
                workspace_package,
                "repository",
            ),
            readme_declared_false,
            readme_rel_path,
            readme_exists,
            readme_content,
            keywords_count: inheritance::inherited_array_count(
                package,
                workspace_package,
                "keywords",
            ),
            categories_count: inheritance::inherited_array_count(
                package,
                workspace_package,
                "categories",
            ),
            version_string,
            workspace_version,
            version_valid,
            docs_rs_present: package
                .and_then(|package| package.get("metadata"))
                .and_then(inheritance::docs_rs_table)
                .is_some_and(inheritance::has_supported_docs_rs_settings),
            include_exclude_present: package
                .is_some_and(inheritance::has_include_or_exclude_patterns),
            has_binstall_metadata: package
                .and_then(|package| package.get("metadata"))
                .and_then(|metadata| metadata.get("binstall"))
                .and_then(toml::Value::as_table)
                .is_some(),
            dry_run: if publishable && thorough {
                tree.abs_path(&root.rel_dir)
                    .and_then(|abs| tc.run_cargo_publish_dry_run_outcome(&abs))
            } else {
                None
            },
        });
    }

    crates
}

fn collect_repo_facts(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    cargo_roots: &BTreeMap<String, types::CargoRootFacts>,
    publishable_names: BTreeSet<String>,
    publishable_binary_names: BTreeSet<String>,
    publishable_count: usize,
    non_publishable_count: usize,
    input_failures: &mut Vec<types::ReleaseInputFailureFacts>,
) -> types::RepoReleaseFacts {
    let root_cargo_rel_path = "Cargo.toml".to_owned();
    let release_plz_rel_path = "release-plz.toml".to_owned();
    let (release_plz_exists, release_plz_parsed, release_plz_package_names) =
        parse_release_plz(tree, input_failures, &release_plz_rel_path);
    let cliff_rel_path = "cliff.toml".to_owned();
    let (cliff_exists, cliff_parsed) = parse_optional_toml(
        tree,
        input_failures,
        &cliff_rel_path,
        "cliff.toml",
        "Failed to parse cliff.toml",
    );
    let workflows = collect_workflows(tree, input_failures);
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
                .or_else(|| root.parsed.get("package"))
        })
        .and_then(|table| release_support::binaries::publish_setting_string(Some(table)));
    let license_rel_path = ["LICENSE", "LICENSE-MIT", "LICENSE-APACHE", "LICENSE.md"]
        .iter()
        .find(|name| tree.file_exists(name))
        .map(|name| (*name).to_owned());

    types::RepoReleaseFacts {
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
        publishable_crate_names: publishable_names,
        publishable_binary_crate_names: publishable_binary_names,
        publishable_count,
        non_publishable_count,
        semver_checks_installed: tc.is_installed("cargo-semver-checks"),
        publish_setting,
        release_profile_settings,
    }
}

fn parse_release_plz(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    input_failures: &mut Vec<types::ReleaseInputFailureFacts>,
    rel_path: &str,
) -> (bool, Option<toml::Value>, BTreeSet<String>) {
    if !tree.file_exists(rel_path) {
        return (false, None, BTreeSet::new());
    }
    let Some(content) = tree.file_content(rel_path) else {
        input_failures.push(types::ReleaseInputFailureFacts {
            rel_path: rel_path.to_owned(),
            message: "Failed to read release-plz.toml.".to_owned(),
        });
        return (true, None, BTreeSet::new());
    };
    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => {
            let package_names = parsed
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
            (true, Some(parsed), package_names)
        }
        Err(parse_error) => {
            input_failures.push(types::ReleaseInputFailureFacts {
                rel_path: rel_path.to_owned(),
                message: format!("Failed to parse release-plz.toml: {parse_error}"),
            });
            (true, None, BTreeSet::new())
        }
    }
}

fn parse_optional_toml(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    input_failures: &mut Vec<types::ReleaseInputFailureFacts>,
    rel_path: &str,
    display_name: &str,
    parse_error_prefix: &str,
) -> (bool, Option<toml::Value>) {
    if !tree.file_exists(rel_path) {
        return (false, None);
    }
    let Some(content) = tree.file_content(rel_path) else {
        input_failures.push(types::ReleaseInputFailureFacts {
            rel_path: rel_path.to_owned(),
            message: format!("Failed to read {display_name}."),
        });
        return (true, None);
    };
    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => (true, Some(parsed)),
        Err(parse_error) => {
            input_failures.push(types::ReleaseInputFailureFacts {
                rel_path: rel_path.to_owned(),
                message: format!("{parse_error_prefix}: {parse_error}"),
            });
            (true, None)
        }
    }
}

fn collect_release_edges(
    cargo_roots: &BTreeMap<String, types::CargoRootFacts>,
    validation_scope: Option<&str>,
    version_map: &BTreeMap<String, String>,
    publishable_names: &BTreeSet<String>,
) -> Vec<types::ReleaseEdgeFacts> {
    let mut edges = Vec::new();
    for root in cargo_roots
        .values()
        .filter(|root| root.has_package)
        .filter(|root| {
            validation_scope
                .is_none_or(|scope| rel_intersects_validation_scope(&root.rel_dir, scope))
        })
    {
        let package = release_support::binaries::package_table(&root.parsed);
        let workspace_root = cargo_roots::workspace_root_for_package(root, cargo_roots);
        let workspace_package = workspace_root
            .and_then(|workspace_root| workspace_root.parsed.get("workspace"))
            .and_then(|workspace| workspace.get("package"));
        if !inheritance::inherited_publishable(package, workspace_package) {
            continue;
        }
        let crate_name = package
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap_or("unknown")
            .to_owned();
        let workspace_dependencies = workspace_root
            .map(|workspace| &workspace.workspace_dependencies)
            .cloned()
            .unwrap_or_default();
        for edge in
            release_support::dependencies::dependency_edges(&root.parsed, &workspace_dependencies)
        {
            let actual_version = version_map.get(&edge.dep_package_name).cloned();
            let dep_publishable = publishable_names.contains(&edge.dep_package_name);
            let version_satisfied = edge
                .version_req
                .as_deref()
                .zip(actual_version.as_deref())
                .map(|(req, actual)| {
                    release_support::binaries::version_requirement_satisfied(actual, req)
                });
            edges.push(types::ReleaseEdgeFacts {
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
    edges
}

fn rel_intersects_validation_scope(rel_dir: &str, validation_scope: &str) -> bool {
    if validation_scope.is_empty() || rel_dir.is_empty() {
        return true;
    }

    rel_dir == validation_scope
        || rel_dir
            .strip_prefix(validation_scope)
            .is_some_and(|rest| rest.starts_with('/'))
        || validation_scope
            .strip_prefix(rel_dir)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn collect_workflows(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    input_failures: &mut Vec<types::ReleaseInputFailureFacts>,
) -> Vec<types::WorkflowFacts> {
    let mut workflow_paths = tree
        .structure()
        .iter()
        .flat_map(|(dir_rel, entry)| {
            entry.files().iter().filter_map(move |file_name| {
                let rel_path =
                    guardrail3_app_rs_family_view::FamilyView::join_rel(dir_rel, file_name);
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
                input_failures.push(types::ReleaseInputFailureFacts {
                    rel_path: rel_path.clone(),
                    message: "Failed to read workflow YAML.".to_owned(),
                });
                return None;
            };
            let parsed = match serde_yaml::from_str::<serde_yaml::Value>(content) {
                Ok(parsed) => parsed,
                Err(parse_error) => {
                    input_failures.push(types::ReleaseInputFailureFacts {
                        rel_path: rel_path.clone(),
                        message: format!("Failed to parse workflow YAML: {parse_error}"),
                    });
                    return None;
                }
            };
            let analysis = release_support::workflows::extract_workflow_analysis(&parsed);
            Some(types::WorkflowFacts {
                rel_path: rel_path.clone(),
                analysis,
            })
        })
        .collect::<Vec<_>>();
    workflows.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    workflows
}
