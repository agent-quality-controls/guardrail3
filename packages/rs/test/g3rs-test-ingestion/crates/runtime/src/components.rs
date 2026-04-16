use std::collections::BTreeSet;

use cargo_toml_parser::{types::CargoToml, parse};
use g3rs_test_types::{
    G3RsTestComponentFileTreeFacts, G3RsTestComponentSourceFacts, G3RsTestFileKind,
    G3RsTestFileTreeInputFailure, G3RsTestOwnedSidecarFacts, G3RsTestSourceFile,
};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

use crate::roots::{OwnedTestRoot, join_under_root, parent_dir};
use crate::run::IngestionError;

#[derive(Debug, Clone)]
pub(crate) struct OwnedTestComponent {
    pub(crate) rel_dir: String,
    pub(crate) runtime_rel_dir: String,
    pub(crate) runtime_cargo_rel_path: String,
    pub(crate) runtime_package_name: Option<String>,
    pub(crate) runtime_normal_dependencies: BTreeSet<String>,
    pub(crate) runtime_dev_dependencies: BTreeSet<String>,
    pub(crate) assertions_rel_dir: String,
    pub(crate) assertions_cargo_rel_path: String,
    pub(crate) assertions_exists: bool,
    pub(crate) nested_assertions_cargo_rel_path: Option<String>,
    pub(crate) assertions_package_name: Option<String>,
    pub(crate) assertions_dependencies: BTreeSet<String>,
    pub(crate) sidecars: Vec<G3RsTestOwnedSidecarFacts>,
    pub(crate) external_harnesses: Vec<String>,
}

pub(crate) fn collect_components(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
) -> Result<Vec<OwnedTestComponent>, IngestionError> {
    let layout = resolve_assertions_layout(crawl, root, None)?;

    Ok(vec![OwnedTestComponent {
        rel_dir: root.root_rel_dir.clone(),
        runtime_rel_dir: root.runtime_rel_dir.clone(),
        runtime_cargo_rel_path: root.cargo_rel_path.clone(),
        runtime_package_name: root
            .cargo
            .package
            .as_ref()
            .and_then(|package| package.name.as_deref())
            .map(rust_crate_name),
        runtime_normal_dependencies: manifest_normal_dependencies(&root.cargo),
        runtime_dev_dependencies: manifest_dev_dependencies(&root.cargo),
        assertions_rel_dir: layout.assertions_rel_dir.clone(),
        assertions_cargo_rel_path: layout.assertions_cargo_rel_path.clone(),
        assertions_exists: layout.assertions_manifest.is_some(),
        nested_assertions_cargo_rel_path: layout.nested_assertions_cargo_rel_path.clone(),
        assertions_package_name: layout
            .assertions_manifest
            .as_ref()
            .and_then(|manifest| manifest.package.as_ref())
            .and_then(|package| package.name.as_deref())
            .map(rust_crate_name),
        assertions_dependencies: layout
            .assertions_manifest
            .as_ref()
            .map(manifest_normal_dependencies)
            .unwrap_or_default(),
        sidecars: collect_sidecars(crawl, &root.runtime_rel_dir, &layout.assertions_rel_dir),
        external_harnesses: collect_external_harnesses(crawl, &root.runtime_rel_dir),
    }])
}

pub(crate) fn collect_file_tree_components(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
) -> (Vec<OwnedTestComponent>, Vec<G3RsTestFileTreeInputFailure>) {
    let mut input_failures = Vec::new();
    let layout = resolve_assertions_layout(crawl, root, Some(&mut input_failures))
        .expect("lenient assertions layout resolution should not fail");

    (
        vec![OwnedTestComponent {
            rel_dir: root.root_rel_dir.clone(),
            runtime_rel_dir: root.runtime_rel_dir.clone(),
            runtime_cargo_rel_path: root.cargo_rel_path.clone(),
            runtime_package_name: root
                .cargo
                .package
                .as_ref()
                .and_then(|package| package.name.as_deref())
                .map(rust_crate_name),
            runtime_normal_dependencies: manifest_normal_dependencies(&root.cargo),
            runtime_dev_dependencies: manifest_dev_dependencies(&root.cargo),
            assertions_rel_dir: layout.assertions_rel_dir.clone(),
            assertions_cargo_rel_path: layout.assertions_cargo_rel_path.clone(),
            assertions_exists: layout.assertions_manifest.is_some(),
            nested_assertions_cargo_rel_path: layout.nested_assertions_cargo_rel_path.clone(),
            assertions_package_name: layout
                .assertions_manifest
                .as_ref()
                .and_then(|manifest| manifest.package.as_ref())
                .and_then(|package| package.name.as_deref())
                .map(rust_crate_name),
            assertions_dependencies: layout
                .assertions_manifest
                .as_ref()
                .map(manifest_normal_dependencies)
                .unwrap_or_default(),
            sidecars: collect_sidecars(crawl, &root.runtime_rel_dir, &layout.assertions_rel_dir),
            external_harnesses: collect_external_harnesses(crawl, &root.runtime_rel_dir),
        }],
        input_failures,
    )
}

pub(crate) fn collect_ast_files(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
    components: &[OwnedTestComponent],
) -> Result<Vec<G3RsTestSourceFile>, IngestionError> {
    let mut files = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with(".rs"))
        .filter_map(|entry| {
            classify_file_for_source(&entry.path.rel_path, root, components)
                .map(|file| (entry, file))
        })
        .map(|(entry, mut file)| {
            if !entry.readable {
                return Err(IngestionError::Unreadable {
                    path: entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }
            let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| {
                IngestionError::Unreadable {
                    path: entry.path.abs_path.clone(),
                    reason: err.to_string(),
                }
            })?;
            file.content = content;
            Ok(file)
        })
        .collect::<Result<Vec<_>, _>>()?;

    files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    Ok(files)
}

pub(crate) fn collect_file_tree_files(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
    components: &[OwnedTestComponent],
) -> (Vec<G3RsTestSourceFile>, Vec<G3RsTestFileTreeInputFailure>) {
    let mut files = Vec::new();
    let mut input_failures = Vec::new();

    for entry in crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with(".rs"))
    {
        let Some(mut file) = classify_file_for_file_tree(&entry.path.rel_path, root, components)
        else {
            continue;
        };
        if !entry.readable {
            input_failures.push(G3RsTestFileTreeInputFailure {
                rel_path: entry.path.rel_path.clone(),
                message:
                    "Failed to read Rust source file for test-family analysis: file is not readable"
                        .to_owned(),
            });
            continue;
        }
        match crate::fs::read_to_string(&entry.path.abs_path) {
            Ok(content) => {
                file.content = content;
                files.push(file);
            }
            Err(err) => input_failures.push(G3RsTestFileTreeInputFailure {
                rel_path: entry.path.rel_path.clone(),
                message: format!("Failed to read Rust source file for test-family analysis: {err}"),
            }),
        }
    }

    files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    input_failures.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    dedupe_failures(&mut input_failures);
    (files, input_failures)
}

pub(crate) fn collect_local_package_names(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
) -> (BTreeSet<String>, Vec<G3RsTestFileTreeInputFailure>) {
    let mut package_names = BTreeSet::new();
    let mut input_failures = Vec::new();

    for entry in crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with("Cargo.toml"))
        .filter(|entry| {
            root_rel_prefix(&entry.path.rel_path, &root.root_rel_dir)
                .is_some_and(|_| !is_fixture_path(&entry.path.rel_path))
        })
    {
        let Some(manifest) =
            parse_manifest_lenient(crawl, &entry.path.rel_path, &mut input_failures)
        else {
            continue;
        };
        if let Some(package_name) = manifest
            .package
            .as_ref()
            .and_then(|package| package.name.as_deref())
        {
            let _ = package_names.insert(rust_crate_name(package_name));
        }
    }

    input_failures.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    dedupe_failures(&mut input_failures);
    (package_names, input_failures)
}

pub(crate) fn public_component_facts(
    components: &[OwnedTestComponent],
) -> Vec<G3RsTestComponentSourceFacts> {
    components
        .iter()
        .map(|component| G3RsTestComponentSourceFacts {
            rel_dir: component.rel_dir.clone(),
            runtime_rel_dir: component.runtime_rel_dir.clone(),
            runtime_package_name: component.runtime_package_name.clone(),
            assertions_rel_dir: component.assertions_rel_dir.clone(),
            assertions_exists: component.assertions_exists,
            assertions_package_name: component.assertions_package_name.clone(),
        })
        .collect()
}

pub(crate) fn public_file_tree_component_facts(
    components: &[OwnedTestComponent],
) -> Vec<G3RsTestComponentFileTreeFacts> {
    components
        .iter()
        .map(|component| G3RsTestComponentFileTreeFacts {
            rel_dir: component.rel_dir.clone(),
            runtime_rel_dir: component.runtime_rel_dir.clone(),
            runtime_cargo_rel_path: component.runtime_cargo_rel_path.clone(),
            runtime_package_name: component.runtime_package_name.clone(),
            runtime_normal_dependencies: component.runtime_normal_dependencies.clone(),
            runtime_dev_dependencies: component.runtime_dev_dependencies.clone(),
            assertions_rel_dir: component.assertions_rel_dir.clone(),
            assertions_cargo_rel_path: component.assertions_cargo_rel_path.clone(),
            assertions_exists: component.assertions_exists,
            nested_assertions_cargo_rel_path: component.nested_assertions_cargo_rel_path.clone(),
            assertions_package_name: component.assertions_package_name.clone(),
            assertions_dependencies: component.assertions_dependencies.clone(),
            sidecars: component.sidecars.clone(),
            external_harnesses: component.external_harnesses.clone(),
        })
        .collect()
}

fn classify_file_for_source(
    rel_path: &str,
    root: &OwnedTestRoot,
    components: &[OwnedTestComponent],
) -> Option<G3RsTestSourceFile> {
    if is_fixture_path(rel_path) {
        return None;
    }

    for component in components {
        let runtime_src = join_under_root(&component.runtime_rel_dir, "src");
        if path_is_under(rel_path, &runtime_src) {
            let rel_after_src = rel_path
                .strip_prefix(&runtime_src)
                .and_then(|rest| rest.strip_prefix('/'))
                .unwrap_or("");
            let (kind, owner_module_name) = if rel_after_src.ends_with("_tests/mod.rs") {
                (
                    G3RsTestFileKind::InternalSidecarMod,
                    rel_after_src
                        .rsplit_once('/')
                        .and_then(|(parent, _)| parent.rsplit('/').next())
                        .and_then(|segment| segment.strip_suffix("_tests"))
                        .map(str::to_owned),
                )
            } else if let Some(owner_module_name) =
                owner_module_name_from_sidecar_path(rel_after_src)
            {
                (
                    G3RsTestFileKind::InternalSidecarSupport,
                    Some(owner_module_name),
                )
            } else {
                (
                    G3RsTestFileKind::Source,
                    file_stem(rel_path).map(str::to_owned),
                )
            };
            return Some(G3RsTestSourceFile {
                rel_path: rel_path.to_owned(),
                kind,
                owner_module_name,
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
                content: String::new(),
            });
        }

        let runtime_tests = join_under_root(&component.runtime_rel_dir, "tests");
        if path_is_under(rel_path, &runtime_tests) && rel_path.ends_with(".rs") {
            return Some(G3RsTestSourceFile {
                rel_path: rel_path.to_owned(),
                kind: G3RsTestFileKind::ExternalHarness,
                owner_module_name: None,
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
                content: String::new(),
            });
        }

        let assertions_src = join_under_root(&component.assertions_rel_dir, "src");
        if path_is_under(rel_path, &assertions_src) {
            return Some(G3RsTestSourceFile {
                rel_path: rel_path.to_owned(),
                kind: G3RsTestFileKind::AssertionsModule,
                owner_module_name: file_stem(rel_path).map(str::to_owned),
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
                content: String::new(),
            });
        }
    }

    let root_runtime_src = join_under_root(&root.runtime_rel_dir, "src");
    if path_is_under(rel_path, &root_runtime_src) {
        return Some(G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::Other,
            owner_module_name: file_stem(rel_path).map(str::to_owned),
            component_rel_dir: None,
            assertions_package_name: None,
            content: String::new(),
        });
    }

    None
}

fn classify_file_for_file_tree(
    rel_path: &str,
    root: &OwnedTestRoot,
    components: &[OwnedTestComponent],
) -> Option<G3RsTestSourceFile> {
    if is_fixture_path(rel_path) {
        return None;
    }

    for component in components {
        let runtime_src = join_under_root(&component.runtime_rel_dir, "src");
        if path_is_under(rel_path, &runtime_src) {
            let rel_after_src = rel_path
                .strip_prefix(&runtime_src)
                .and_then(|rest| rest.strip_prefix('/'))
                .unwrap_or("");
            if rel_after_src.ends_with("_tests/mod.rs") {
                return Some(G3RsTestSourceFile {
                    rel_path: rel_path.to_owned(),
                    kind: G3RsTestFileKind::InternalSidecarMod,
                    owner_module_name: rel_after_src
                        .rsplit_once('/')
                        .and_then(|(parent, _)| parent.rsplit('/').next())
                        .and_then(|segment| segment.strip_suffix("_tests"))
                        .map(str::to_owned),
                    component_rel_dir: Some(component.rel_dir.clone()),
                    assertions_package_name: component.assertions_package_name.clone(),
                    content: String::new(),
                });
            }
            if let Some(owner_module_name) = owner_module_name_from_sidecar_path(rel_after_src) {
                return Some(G3RsTestSourceFile {
                    rel_path: rel_path.to_owned(),
                    kind: G3RsTestFileKind::InternalSidecarSupport,
                    owner_module_name: Some(owner_module_name),
                    component_rel_dir: Some(component.rel_dir.clone()),
                    assertions_package_name: component.assertions_package_name.clone(),
                    content: String::new(),
                });
            }
            return Some(G3RsTestSourceFile {
                rel_path: rel_path.to_owned(),
                kind: G3RsTestFileKind::Source,
                owner_module_name: file_stem(rel_path).map(str::to_owned),
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
                content: String::new(),
            });
        }

        let runtime_tests = join_under_root(&component.runtime_rel_dir, "tests");
        if path_is_under(rel_path, &runtime_tests) && rel_path.ends_with(".rs") {
            return Some(G3RsTestSourceFile {
                rel_path: rel_path.to_owned(),
                kind: G3RsTestFileKind::ExternalHarness,
                owner_module_name: None,
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
                content: String::new(),
            });
        }

        let assertions_src = join_under_root(&component.assertions_rel_dir, "src");
        if path_is_under(rel_path, &assertions_src) {
            return Some(G3RsTestSourceFile {
                rel_path: rel_path.to_owned(),
                kind: G3RsTestFileKind::AssertionsModule,
                owner_module_name: file_stem(rel_path).map(str::to_owned),
                component_rel_dir: Some(component.rel_dir.clone()),
                assertions_package_name: component.assertions_package_name.clone(),
                content: String::new(),
            });
        }
    }

    if is_test_support_path(root, rel_path) {
        return Some(G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::TestSupport,
            owner_module_name: file_stem(rel_path).map(str::to_owned),
            component_rel_dir: None,
            assertions_package_name: None,
            content: String::new(),
        });
    }

    let root_relative = root_rel_prefix(rel_path, &root.root_rel_dir)?;
    if let Some(rel_after_src) = rel_after_named_dir(root_relative, "src") {
        if rel_after_src.ends_with("_tests/mod.rs") {
            return Some(G3RsTestSourceFile {
                rel_path: rel_path.to_owned(),
                kind: G3RsTestFileKind::InternalSidecarMod,
                owner_module_name: rel_after_src
                    .rsplit_once('/')
                    .and_then(|(parent, _)| parent.rsplit('/').next())
                    .and_then(|segment| segment.strip_suffix("_tests"))
                    .map(str::to_owned),
                component_rel_dir: None,
                assertions_package_name: None,
                content: String::new(),
            });
        }
        if let Some(owner_module_name) = owner_module_name_from_sidecar_path(rel_after_src) {
            return Some(G3RsTestSourceFile {
                rel_path: rel_path.to_owned(),
                kind: G3RsTestFileKind::InternalSidecarSupport,
                owner_module_name: Some(owner_module_name),
                component_rel_dir: None,
                assertions_package_name: None,
                content: String::new(),
            });
        }
        return Some(G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::Source,
            owner_module_name: file_stem(rel_path).map(str::to_owned),
            component_rel_dir: None,
            assertions_package_name: None,
            content: String::new(),
        });
    }

    let kind = if rel_after_named_dir(root_relative, "tests").is_some() {
        G3RsTestFileKind::ExternalHarness
    } else {
        G3RsTestFileKind::Other
    };
    Some(G3RsTestSourceFile {
        rel_path: rel_path.to_owned(),
        kind,
        owner_module_name: file_stem(rel_path).map(str::to_owned),
        component_rel_dir: None,
        assertions_package_name: None,
        content: String::new(),
    })
}

fn parse_optional_manifest(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Result<Option<CargoToml>, IngestionError> {
    let Some(entry) = g3rs_workspace_crawl::entry(crawl, rel_path) else {
        return Ok(None);
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: err.to_string(),
        }
    })?;
    parse(&content)
        .map(Some)
        .map_err(|err| IngestionError::ParseFailed {
            path: entry.path.abs_path.clone(),
            reason: err.to_string(),
        })
}

fn parse_optional_manifest_lenient(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
    input_failures: &mut Vec<G3RsTestFileTreeInputFailure>,
) -> Option<CargoToml> {
    if g3rs_workspace_crawl::entry(crawl, rel_path).is_none() {
        return None;
    }
    parse_manifest_lenient(crawl, rel_path, input_failures)
}

fn parse_manifest_lenient(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
    input_failures: &mut Vec<G3RsTestFileTreeInputFailure>,
) -> Option<CargoToml> {
    let Some(entry) = g3rs_workspace_crawl::entry(crawl, rel_path) else {
        input_failures.push(G3RsTestFileTreeInputFailure {
            rel_path: rel_path.to_owned(),
            message: "Failed to parse Cargo.toml for test-family boundaries: required Cargo.toml entry is missing from crawl".to_owned(),
        });
        return None;
    };
    if !entry.readable {
        input_failures.push(G3RsTestFileTreeInputFailure {
            rel_path: rel_path.to_owned(),
            message: "Failed to read Cargo.toml for test-family boundaries: file is not readable"
                .to_owned(),
        });
        return None;
    }
    let content = match crate::fs::read_to_string(&entry.path.abs_path) {
        Ok(content) => content,
        Err(err) => {
            input_failures.push(G3RsTestFileTreeInputFailure {
                rel_path: rel_path.to_owned(),
                message: format!("Failed to read Cargo.toml for test-family boundaries: {err}"),
            });
            return None;
        }
    };
    match parse(&content) {
        Ok(manifest) => Some(manifest),
        Err(err) => {
            input_failures.push(G3RsTestFileTreeInputFailure {
                rel_path: rel_path.to_owned(),
                message: format!("Failed to parse Cargo.toml for test-family boundaries: {err}"),
            });
            None
        }
    }
}

fn manifest_normal_dependencies(manifest: &CargoToml) -> BTreeSet<String> {
    manifest
        .dependencies
        .keys()
        .chain(manifest.build_dependencies.keys())
        .map(|name| rust_crate_name(name))
        .collect()
}

fn manifest_dev_dependencies(manifest: &CargoToml) -> BTreeSet<String> {
    manifest
        .dev_dependencies
        .keys()
        .map(|name| rust_crate_name(name))
        .collect()
}

struct AssertionsLayout {
    assertions_rel_dir: String,
    assertions_cargo_rel_path: String,
    assertions_manifest: Option<CargoToml>,
    nested_assertions_cargo_rel_path: Option<String>,
}

fn resolve_assertions_layout(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
    mut input_failures: Option<&mut Vec<G3RsTestFileTreeInputFailure>>,
) -> Result<AssertionsLayout, IngestionError> {
    let nested_assertions_cargo_rel_path = g3rs_workspace_crawl::entry(
        crawl,
        &join_under_root(&root.root_rel_dir, "assertions/Cargo.toml"),
    )
    .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
    .map(|_| join_under_root(&root.root_rel_dir, "assertions/Cargo.toml"));
    let assertions_rel_dir = if root.runtime_rel_dir == root.root_rel_dir
        && nested_assertions_cargo_rel_path.is_some()
    {
        join_under_root(&root.root_rel_dir, "crates/assertions")
    } else if root.runtime_rel_dir == root.root_rel_dir {
        join_under_root(&root.root_rel_dir, "assertions")
    } else {
        format!("{}/assertions", parent_dir(&root.runtime_rel_dir))
    };
    let assertions_cargo_rel_path = format!("{assertions_rel_dir}/Cargo.toml");
    let assertions_manifest = if let Some(failures) = input_failures.as_deref_mut() {
        parse_optional_manifest_lenient(crawl, &assertions_cargo_rel_path, failures)
    } else {
        parse_optional_manifest(crawl, &assertions_cargo_rel_path)?
    };

    Ok(AssertionsLayout {
        assertions_rel_dir,
        assertions_cargo_rel_path: assertions_cargo_rel_path.clone(),
        assertions_manifest,
        nested_assertions_cargo_rel_path: nested_assertions_cargo_rel_path
            .filter(|nested| nested != &assertions_cargo_rel_path),
    })
}

fn rust_crate_name(package_name: &str) -> String {
    package_name.replace('-', "_")
}

fn collect_sidecars(
    crawl: &G3RsWorkspaceCrawl,
    runtime_rel_dir: &str,
    assertions_rel_dir: &str,
) -> Vec<G3RsTestOwnedSidecarFacts> {
    let src_rel_dir = join_under_root(runtime_rel_dir, "src");
    let mut sidecars = Vec::new();
    for entry in crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.starts_with(src_rel_dir.as_str()))
        .filter(|entry| entry.path.rel_path.ends_with("_tests/mod.rs"))
    {
        let dir_rel = parent_dir(&entry.path.rel_path).to_owned();
        let Some(dir_name) = dir_rel.rsplit('/').next() else {
            continue;
        };
        let Some(owner_module_name) = dir_name.strip_suffix("_tests") else {
            continue;
        };
        let sidecar_root_rel = dir_rel
            .strip_prefix(&src_rel_dir)
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or(dir_name);
        let relative_parent = parent_dir(sidecar_root_rel);
        let assertions_src_rel = join_under_root(assertions_rel_dir, "src");
        let assertions_module_rel_path = if relative_parent.is_empty() {
            join_under_root(&assertions_src_rel, &format!("{owner_module_name}.rs"))
        } else {
            join_under_root(
                &assertions_src_rel,
                &format!("{relative_parent}/{owner_module_name}.rs"),
            )
        };
        sidecars.push(G3RsTestOwnedSidecarFacts {
            mod_rel_path: entry.path.rel_path.clone(),
            assertions_module_rel_path,
        });
    }

    sidecars.sort_by(|left, right| left.mod_rel_path.cmp(&right.mod_rel_path));
    sidecars
}

fn collect_external_harnesses(crawl: &G3RsWorkspaceCrawl, runtime_rel_dir: &str) -> Vec<String> {
    let tests_rel_dir = join_under_root(runtime_rel_dir, "tests");
    let mut files = crawl
        .entries
        .iter()
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .filter(|entry| entry.path.rel_path.ends_with(".rs"))
        .filter_map(|entry| {
            let rel_path = entry.path.rel_path.as_str();
            let parent = parent_dir(rel_path);
            (parent == tests_rel_dir).then(|| rel_path.to_owned())
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn dedupe_failures(input_failures: &mut Vec<G3RsTestFileTreeInputFailure>) {
    input_failures
        .dedup_by(|left, right| left.rel_path == right.rel_path && left.message == right.message);
}

fn path_is_under(rel_path: &str, prefix: &str) -> bool {
    rel_path == prefix
        || rel_path
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn file_stem(rel_path: &str) -> Option<&str> {
    rel_path
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".rs"))
}

fn owner_module_name_from_sidecar_path(rel_after_src: &str) -> Option<String> {
    rel_after_src.split('/').find_map(|segment| {
        segment
            .strip_suffix("_tests")
            .map(str::to_owned)
            .filter(|value| !value.is_empty())
    })
}

fn root_rel_prefix<'a>(rel_path: &'a str, root_rel_dir: &str) -> Option<&'a str> {
    if root_rel_dir.is_empty() {
        Some(rel_path)
    } else {
        rel_path
            .strip_prefix(root_rel_dir)
            .and_then(|rest| rest.strip_prefix('/'))
    }
}

fn rel_after_named_dir<'a>(root_relative: &'a str, dir_name: &str) -> Option<&'a str> {
    let prefix = format!("{dir_name}/");
    if let Some(rest) = root_relative.strip_prefix(&prefix) {
        return Some(rest);
    }
    let marker = format!("/{dir_name}/");
    root_relative.rsplit_once(&marker).map(|(_, rest)| rest)
}

fn is_test_support_path(root: &OwnedTestRoot, rel_path: &str) -> bool {
    [
        join_under_root(&root.root_rel_dir, "test_support/src"),
        join_under_root(&root.root_rel_dir, "crates/test_support/src"),
    ]
    .into_iter()
    .any(|test_support_src| path_is_under(rel_path, &test_support_src))
}

fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/")
        || rel_path.starts_with("tests/fixtures/")
        || rel_path.contains("_tests/fixtures/")
        || rel_path.contains("assertions/src/fixtures/")
        || rel_path.contains("test_support/src/fixtures/")
}
