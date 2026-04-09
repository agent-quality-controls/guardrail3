use cargo_toml_parser::{CargoToml, parse};
use g3rs_test_types::{G3RsTestComponentAstFacts, G3RsTestFileKind, G3RsTestSourceFile};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

use crate::roots::{OwnedTestRoot, join_under_root, parent_dir};
use crate::run::IngestionError;

#[derive(Debug, Clone)]
pub(crate) struct OwnedTestComponent {
    pub(crate) rel_dir: String,
    pub(crate) runtime_rel_dir: String,
    pub(crate) runtime_package_name: Option<String>,
    pub(crate) assertions_rel_dir: String,
    pub(crate) assertions_exists: bool,
    pub(crate) assertions_package_name: Option<String>,
}

pub(crate) fn collect_components(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
) -> Result<Vec<OwnedTestComponent>, IngestionError> {
    let assertions_rel_dir = if root.runtime_rel_dir == root.root_rel_dir {
        join_under_root(&root.root_rel_dir, "assertions")
    } else {
        format!("{}/assertions", parent_dir(&root.runtime_rel_dir))
    };
    let assertions_cargo_rel_path = format!("{assertions_rel_dir}/Cargo.toml");
    let assertions_manifest = parse_optional_manifest(crawl, &assertions_cargo_rel_path)?;

    Ok(vec![OwnedTestComponent {
        rel_dir: root.root_rel_dir.clone(),
        runtime_rel_dir: root.runtime_rel_dir.clone(),
        runtime_package_name: root
            .cargo
            .package
            .as_ref()
            .and_then(|package| package.name.as_deref())
            .map(rust_crate_name),
        assertions_rel_dir: assertions_rel_dir.clone(),
        assertions_exists: assertions_manifest.is_some(),
        assertions_package_name: assertions_manifest
            .as_ref()
            .and_then(|manifest| manifest.package.as_ref())
            .and_then(|package| package.name.as_deref())
            .map(rust_crate_name),
    }])
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
        .filter_map(|entry| classify_file(&entry.path.rel_path, root, components).map(|file| (entry, file)))
        .map(|(entry, mut file)| {
            if !entry.readable {
                return Err(IngestionError::Unreadable {
                    path: entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }
            let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;
            file.content = content;
            Ok(file)
        })
        .collect::<Result<Vec<_>, _>>()?;

    files.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    Ok(files)
}

pub(crate) fn public_component_facts(
    components: &[OwnedTestComponent],
) -> Vec<G3RsTestComponentAstFacts> {
    components
        .iter()
        .map(|component| G3RsTestComponentAstFacts {
            rel_dir: component.rel_dir.clone(),
            runtime_rel_dir: component.runtime_rel_dir.clone(),
            runtime_package_name: component.runtime_package_name.clone(),
            assertions_rel_dir: component.assertions_rel_dir.clone(),
            assertions_exists: component.assertions_exists,
            assertions_package_name: component.assertions_package_name.clone(),
        })
        .collect()
}

fn classify_file(
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
            } else if let Some(owner_module_name) = owner_module_name_from_sidecar_path(rel_after_src) {
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

fn parse_optional_manifest(
    crawl: &G3RsWorkspaceCrawl,
    rel_path: &str,
) -> Result<Option<CargoToml>, IngestionError> {
    let Some(entry) = crawl.entry(rel_path) else {
        return Ok(None);
    };
    if !entry.readable {
        return Err(IngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;
    parse(&content).map(Some).map_err(|err| IngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })
}

fn rust_crate_name(package_name: &str) -> String {
    package_name.replace('-', "_")
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
    if !rel_after_src.contains("_tests/") || rel_after_src.ends_with("_tests/mod.rs") {
        return None;
    }
    rel_after_src
        .rsplit_once('/')
        .and_then(|(parent, _)| parent.rsplit('/').next())
        .and_then(|segment| segment.strip_suffix("_tests"))
        .map(str::to_owned)
}

fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/")
        || rel_path.starts_with("tests/fixtures/")
        || rel_path.contains("_tests/fixtures/")
        || rel_path.contains("assertions/src/fixtures/")
        || rel_path.contains("test_support/src/fixtures/")
}
