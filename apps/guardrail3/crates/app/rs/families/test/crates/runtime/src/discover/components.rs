use std::collections::BTreeSet;

use guardrail3_domain_project_tree::ProjectTree;

use crate::facts::{
    DiscoveredTestFile, InputFailureFacts, SidecarFacts, TestComponentFacts, TestFileKind,
    TestRootFacts,
};

pub(super) fn collect_components(
    tree: &ProjectTree,
    root_rel_dir: &str,
    root_has_package: bool,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<TestComponentFacts> {
    let crates_rel_dir = super::join_under_root(root_rel_dir, "crates");
    let direct_runtime_rel_dir = ProjectTree::join_rel(&crates_rel_dir, "runtime");
    let direct_runtime_cargo_rel_path =
        ProjectTree::join_rel(&direct_runtime_rel_dir, "Cargo.toml");
    if tree.file_exists(&direct_runtime_cargo_rel_path) {
        return vec![build_component_facts(
            tree,
            root_rel_dir,
            root_rel_dir,
            &direct_runtime_rel_dir,
            input_failures,
        )];
    }

    if !root_has_package {
        return Vec::new();
    }

    vec![build_component_facts(
        tree,
        root_rel_dir,
        root_rel_dir,
        root_rel_dir,
        input_failures,
    )]
}

fn build_component_facts(
    tree: &ProjectTree,
    root_rel_dir: &str,
    component_rel_dir: &str,
    runtime_rel_dir: &str,
    input_failures: &mut Vec<InputFailureFacts>,
) -> TestComponentFacts {
    let runtime_cargo_rel_path = ProjectTree::join_rel(runtime_rel_dir, "Cargo.toml");
    let runtime_parsed =
        parse_manifest(tree, root_rel_dir, &runtime_cargo_rel_path, input_failures);
    let component_parent = if runtime_rel_dir == component_rel_dir {
        component_rel_dir.to_owned()
    } else {
        super::parent_dir(runtime_rel_dir).to_owned()
    };
    let assertions_rel_dir = ProjectTree::join_rel(&component_parent, "assertions");
    let assertions_cargo_rel_path = ProjectTree::join_rel(&assertions_rel_dir, "Cargo.toml");
    let assertions_exists = tree.file_exists(&assertions_cargo_rel_path);
    let assertions_parsed = if assertions_exists {
        parse_manifest(
            tree,
            root_rel_dir,
            &assertions_cargo_rel_path,
            input_failures,
        )
    } else {
        None
    };

    TestComponentFacts {
        rel_dir: component_rel_dir.to_owned(),
        runtime_rel_dir: runtime_rel_dir.to_owned(),
        runtime_cargo_rel_path,
        runtime_package_name: runtime_parsed.as_ref().and_then(manifest_package_name),
        runtime_normal_dependencies: runtime_parsed
            .as_ref()
            .map(manifest_normal_dependencies)
            .unwrap_or_default(),
        runtime_dev_dependencies: runtime_parsed
            .as_ref()
            .map(manifest_dev_dependencies)
            .unwrap_or_default(),
        assertions_rel_dir: assertions_rel_dir.clone(),
        assertions_cargo_rel_path,
        assertions_exists,
        assertions_package_name: assertions_parsed.as_ref().and_then(manifest_package_name),
        assertions_dependencies: assertions_parsed
            .as_ref()
            .map(manifest_normal_dependencies)
            .unwrap_or_default(),
        sidecars: collect_sidecars(tree, runtime_rel_dir, &assertions_rel_dir),
        external_harnesses: collect_external_harnesses(tree, runtime_rel_dir),
    }
}

fn parse_manifest(
    tree: &ProjectTree,
    root_rel_dir: &str,
    rel_path: &str,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Option<toml::Value> {
    let content = match super::read_cached_or_fs(tree, rel_path) {
        Ok(Some(content)) => content,
        Ok(None) => return None,
        Err(read_error) => {
            input_failures.push(InputFailureFacts {
                root_rel_dir: root_rel_dir.to_owned(),
                rel_path: rel_path.to_owned(),
                message: format!(
                    "Failed to read Cargo.toml for test-family boundaries: {read_error}"
                ),
            });
            return None;
        }
    };
    match toml::from_str::<toml::Value>(&content) {
        Ok(parsed) => Some(parsed),
        Err(parse_error) => {
            input_failures.push(InputFailureFacts {
                root_rel_dir: root_rel_dir.to_owned(),
                rel_path: rel_path.to_owned(),
                message: format!(
                    "Failed to parse Cargo.toml for test-family boundaries: {parse_error}"
                ),
            });
            None
        }
    }
}

pub(super) fn manifest_package_name(parsed: &toml::Value) -> Option<String> {
    parsed
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
        .map(rust_crate_name)
}

fn manifest_normal_dependencies(parsed: &toml::Value) -> BTreeSet<String> {
    dependency_names(parsed, ["dependencies", "build-dependencies"])
}

fn manifest_dev_dependencies(parsed: &toml::Value) -> BTreeSet<String> {
    dependency_names(parsed, ["dev-dependencies"])
}

fn dependency_names<const N: usize>(parsed: &toml::Value, sections: [&str; N]) -> BTreeSet<String> {
    sections
        .into_iter()
        .filter_map(|section| parsed.get(section).and_then(toml::Value::as_table))
        .flat_map(|table| table.keys().map(|name| rust_crate_name(name)))
        .collect()
}

fn rust_crate_name(package_name: &str) -> String {
    package_name.replace('-', "_")
}

fn collect_sidecars(
    tree: &ProjectTree,
    runtime_rel_dir: &str,
    assertions_rel_dir: &str,
) -> Vec<SidecarFacts> {
    let src_rel_dir = ProjectTree::join_rel(runtime_rel_dir, "src");
    let mut sidecars = Vec::new();

    for dir_rel in tree.all_dir_rels() {
        if !super::path_is_under(&dir_rel, &src_rel_dir) {
            continue;
        }
        let Some(dir_name) = dir_rel.rsplit('/').next() else {
            continue;
        };
        let Some(owner_module_name) = dir_name.strip_suffix("_tests") else {
            continue;
        };
        let mod_rel_path = ProjectTree::join_rel(&dir_rel, "mod.rs");
        if !tree.file_exists(&mod_rel_path) {
            continue;
        }
        let sidecar_root_rel = dir_rel
            .strip_prefix(&src_rel_dir)
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or(dir_name);
        let relative_parent = super::parent_dir(sidecar_root_rel);
        let assertions_src_rel = ProjectTree::join_rel(assertions_rel_dir, "src");
        let assertions_module_rel_path = if relative_parent.is_empty() {
            ProjectTree::join_rel(&assertions_src_rel, &format!("{owner_module_name}.rs"))
        } else {
            ProjectTree::join_rel(
                &assertions_src_rel,
                &format!("{relative_parent}/{owner_module_name}.rs"),
            )
        };
        sidecars.push(SidecarFacts {
            mod_rel_path,
            assertions_module_rel_path,
        });
    }

    sidecars.sort_by(|left, right| left.mod_rel_path.cmp(&right.mod_rel_path));
    sidecars
}

fn collect_external_harnesses(tree: &ProjectTree, runtime_rel_dir: &str) -> Vec<String> {
    let tests_rel_dir = ProjectTree::join_rel(runtime_rel_dir, "tests");
    let Some(tests_dir) = tree.dir_contents(&tests_rel_dir) else {
        return Vec::new();
    };

    let mut files = tests_dir
        .files
        .iter()
        .filter(|file_name| file_name.ends_with(".rs"))
        .map(|file_name| ProjectTree::join_rel(&tests_rel_dir, file_name))
        .collect::<Vec<_>>();
    files.sort();
    files
}

pub(super) fn classify_file(root: &TestRootFacts, rel_path: &str) -> DiscoveredTestFile {
    for component in &root.components {
        let runtime_src = ProjectTree::join_rel(&component.runtime_rel_dir, "src");
        if super::path_is_under(rel_path, &runtime_src) {
            let rel_after_src = rel_path
                .strip_prefix(&runtime_src)
                .and_then(|rest| rest.strip_prefix('/'))
                .unwrap_or("");
            if rel_after_src.ends_with("_tests/mod.rs") {
                return DiscoveredTestFile {
                    rel_path: rel_path.to_owned(),
                    root_rel_dir: root.rel_dir.clone(),
                    kind: TestFileKind::InternalSidecarMod,
                    owner_module_name: rel_after_src
                        .rsplit_once('/')
                        .and_then(|(parent, _)| parent.rsplit('/').next())
                        .and_then(|segment| segment.strip_suffix("_tests"))
                        .map(str::to_owned),
                    component_rel_dir: Some(component.rel_dir.clone()),
                    assertions_package_name: component.assertions_package_name.clone(),
                };
            }
            if let Some(owner_module_name) = owner_module_name_from_sidecar_path(rel_after_src) {
                return DiscoveredTestFile {
                    rel_path: rel_path.to_owned(),
                    root_rel_dir: root.rel_dir.clone(),
                    kind: TestFileKind::InternalSidecarSupport,
                    owner_module_name: Some(owner_module_name),
                    component_rel_dir: Some(component.rel_dir.clone()),
                    assertions_package_name: component.assertions_package_name.clone(),
                };
            }
            return DiscoveredTestFile {
                rel_path: rel_path.to_owned(),
                root_rel_dir: root.rel_dir.clone(),
                kind: TestFileKind::Source,
                owner_module_name: super::file_stem(rel_path).map(str::to_owned),
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
            };
        }

        for external_harness in &component.external_harnesses {
            if rel_path == external_harness {
                return DiscoveredTestFile {
                    rel_path: rel_path.to_owned(),
                    root_rel_dir: root.rel_dir.clone(),
                    kind: TestFileKind::ExternalHarness,
                    owner_module_name: None,
                    component_rel_dir: Some(component.rel_dir.clone()),
                    assertions_package_name: component.assertions_package_name.clone(),
                };
            }
        }

        let assertions_src = ProjectTree::join_rel(&component.assertions_rel_dir, "src");
        if super::path_is_under(rel_path, &assertions_src) {
            return DiscoveredTestFile {
                rel_path: rel_path.to_owned(),
                root_rel_dir: root.rel_dir.clone(),
                kind: TestFileKind::AssertionsModule,
                owner_module_name: super::file_stem(rel_path).map(str::to_owned),
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
            };
        }
    }

    let root_relative = super::root_relative(rel_path, &root.rel_dir);
    if let Some(rel_after_src) = rel_after_named_dir(root_relative, "src") {
        if rel_after_src.ends_with("_tests/mod.rs") {
            return DiscoveredTestFile {
                rel_path: rel_path.to_owned(),
                root_rel_dir: root.rel_dir.clone(),
                kind: TestFileKind::InternalSidecarMod,
                owner_module_name: rel_after_src
                    .rsplit_once('/')
                    .and_then(|(parent, _)| parent.rsplit('/').next())
                    .and_then(|segment| segment.strip_suffix("_tests"))
                    .map(str::to_owned),
                component_rel_dir: None,
                assertions_package_name: None,
            };
        }
        if let Some(owner_module_name) = owner_module_name_from_sidecar_path(rel_after_src) {
            return DiscoveredTestFile {
                rel_path: rel_path.to_owned(),
                root_rel_dir: root.rel_dir.clone(),
                kind: TestFileKind::InternalSidecarSupport,
                owner_module_name: Some(owner_module_name),
                component_rel_dir: None,
                assertions_package_name: None,
            };
        }
        return DiscoveredTestFile {
            rel_path: rel_path.to_owned(),
            root_rel_dir: root.rel_dir.clone(),
            kind: TestFileKind::Source,
            owner_module_name: super::file_stem(rel_path).map(str::to_owned),
            component_rel_dir: None,
            assertions_package_name: None,
        };
    }
    let kind = if rel_after_named_dir(root_relative, "tests").is_some() {
        TestFileKind::ExternalHarness
    } else {
        TestFileKind::Other
    };
    DiscoveredTestFile {
        rel_path: rel_path.to_owned(),
        root_rel_dir: root.rel_dir.clone(),
        kind,
        owner_module_name: super::file_stem(rel_path).map(str::to_owned),
        component_rel_dir: None,
        assertions_package_name: None,
    }
}

fn owner_module_name_from_sidecar_path(rel_after_src: &str) -> Option<String> {
    rel_after_src.split('/').find_map(|segment| {
        segment
            .strip_suffix("_tests")
            .map(str::to_owned)
            .filter(|value| !value.is_empty())
    })
}

fn rel_after_named_dir<'a>(root_relative: &'a str, dir_name: &str) -> Option<&'a str> {
    let prefix = format!("{dir_name}/");
    if let Some(rest) = root_relative.strip_prefix(&prefix) {
        return Some(rest);
    }
    let marker = format!("/{dir_name}/");
    root_relative.rsplit_once(&marker).map(|(_, rest)| rest)
}
