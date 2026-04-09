use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use cargo_toml_parser::CargoToml;
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

use crate::run::IngestionError;

/// Whether a repo-relative source path is fixture content that should not be
/// treated as owned code.
pub(crate) fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/") || rel_path.starts_with("tests/fixtures/")
}

/// Whether a repo-relative source path belongs to test-owned code.
pub(crate) fn is_test_root_path(rel_path: &str) -> bool {
    let segments = rel_path.split('/').collect::<Vec<_>>();
    if segments.first().is_some_and(|segment| *segment == "tests") {
        return true;
    }
    if segments.iter().any(|segment| segment.ends_with("_tests")) {
        return true;
    }
    let Some(file_name) = segments.last() else {
        return false;
    };
    file_name.ends_with("_test.rs") || file_name.ends_with("_tests.rs")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceProfile {
    pub(crate) profile_name: Option<String>,
    pub(crate) is_library_root: bool,
}

#[derive(Debug)]
pub(crate) struct CargoTargetClassifier {
    packages: Vec<PackageTargets>,
}

#[derive(Debug)]
struct PackageTargets {
    package_dir_rel: String,
    library_root_rel: Option<String>,
    binary_root_rels: Vec<String>,
}

impl CargoTargetClassifier {
    pub(crate) fn build(
        crawl: &G3RsWorkspaceCrawl,
        selected_source_rels: &[String],
    ) -> Result<Self, IngestionError> {
        let relevant_manifest_rels = selected_source_rels
            .iter()
            .filter_map(|rel_path| nearest_manifest_rel_path(crawl, rel_path))
            .collect::<BTreeSet<_>>();

        let packages = relevant_manifest_rels
            .into_iter()
            .map(|manifest_rel| load_package_targets(crawl, &manifest_rel))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { packages })
    }

    pub(crate) fn classify(&self, rel_path: &str, is_test: bool) -> SourceProfile {
        if is_test {
            return SourceProfile {
                profile_name: None,
                is_library_root: false,
            };
        }

        let Some(package) = self
            .packages
            .iter()
            .filter(|package| path_is_within(rel_path, &package.package_dir_rel))
            .max_by_key(|package| package.package_dir_rel.len())
        else {
            return SourceProfile {
                profile_name: None,
                is_library_root: false,
            };
        };

        if package.library_root_rel.as_deref() == Some(rel_path) {
            return SourceProfile {
                profile_name: Some("library".to_owned()),
                is_library_root: true,
            };
        }

        if package.binary_root_rels.iter().any(|root| root == rel_path)
            || package.is_binary_owned_file(rel_path)
        {
            return SourceProfile {
                profile_name: Some("binary".to_owned()),
                is_library_root: false,
            };
        }

        if package.is_library_owned_file(rel_path) {
            return SourceProfile {
                profile_name: Some("library".to_owned()),
                is_library_root: false,
            };
        }

        SourceProfile {
            profile_name: None,
            is_library_root: false,
        }
    }
}

impl PackageTargets {
    fn is_binary_owned_file(&self, rel_path: &str) -> bool {
        let package_src_dir = join_rel(&self.package_dir_rel, "src");
        let package_src_bin_dir = join_rel(&self.package_dir_rel, "src/bin");

        if path_is_within(rel_path, &package_src_bin_dir) {
            return true;
        }

        self.binary_root_rels.iter().any(|root| {
            let parent_dir = parent_rel_dir(root);
            parent_dir.is_some_and(|dir| dir != package_src_dir && path_is_within(rel_path, &dir))
        })
    }

    fn is_library_owned_file(&self, rel_path: &str) -> bool {
        let Some(library_root_rel) = self.library_root_rel.as_deref() else {
            return false;
        };
        let Some(library_dir_rel) = parent_rel_dir(library_root_rel) else {
            return false;
        };
        if !path_is_within(rel_path, &library_dir_rel) {
            return false;
        }

        let package_src_bin_dir = join_rel(&self.package_dir_rel, "src/bin");
        if path_is_within(rel_path, &package_src_bin_dir) {
            return false;
        }

        if self.binary_root_rels.iter().any(|root| root == rel_path) {
            return false;
        }

        let tests_dir = join_rel(&self.package_dir_rel, "tests");
        let benches_dir = join_rel(&self.package_dir_rel, "benches");
        let examples_dir = join_rel(&self.package_dir_rel, "examples");
        !path_is_within(rel_path, &tests_dir)
            && !path_is_within(rel_path, &benches_dir)
            && !path_is_within(rel_path, &examples_dir)
    }
}

fn load_package_targets(
    crawl: &G3RsWorkspaceCrawl,
    manifest_rel_path: &str,
) -> Result<PackageTargets, IngestionError> {
    let manifest_entry = crawl
        .entry(manifest_rel_path)
        .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
        .ok_or_else(|| IngestionError::Unreadable {
            path: crawl.root_abs_path.join(manifest_rel_path),
            reason: "manifest missing from crawl".to_owned(),
        })?;

    if !manifest_entry.readable {
        return Err(IngestionError::Unreadable {
            path: manifest_entry.path.abs_path.clone(),
            reason: "manifest is not readable".to_owned(),
        });
    }

    let content = crate::fs::read_to_string(&manifest_entry.path.abs_path).map_err(|err| {
        IngestionError::Unreadable {
            path: manifest_entry.path.abs_path.clone(),
            reason: err.to_string(),
        }
    })?;

    let manifest =
        cargo_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
            path: manifest_entry.path.abs_path.clone(),
            reason: err.to_string(),
        })?;

    Ok(PackageTargets {
        package_dir_rel: manifest_dir_rel(manifest_rel_path),
        library_root_rel: resolve_library_root(crawl, manifest_rel_path, &manifest),
        binary_root_rels: resolve_binary_roots(crawl, manifest_rel_path, &manifest),
    })
}

fn resolve_library_root(
    crawl: &G3RsWorkspaceCrawl,
    manifest_rel_path: &str,
    manifest: &CargoToml,
) -> Option<String> {
    let package_dir_rel = manifest_dir_rel(manifest_rel_path);
    let autolib_enabled = manifest
        .package
        .as_ref()
        .and_then(|package| package.autolib)
        .unwrap_or(true);

    if let Some(lib_target) = manifest.lib.as_ref() {
        if let Some(path) = lib_target.path.as_deref() {
            return Some(join_rel(&package_dir_rel, path));
        }
        if autolib_enabled {
            let default_rel = join_rel(&package_dir_rel, "src/lib.rs");
            if crawl.entry(&default_rel).is_some() {
                return Some(default_rel);
            }
        }
        return None;
    }

    if !autolib_enabled {
        return None;
    }

    let default_rel = join_rel(&package_dir_rel, "src/lib.rs");
    crawl.entry(&default_rel).map(|_| default_rel)
}

fn resolve_binary_roots(
    crawl: &G3RsWorkspaceCrawl,
    manifest_rel_path: &str,
    manifest: &CargoToml,
) -> Vec<String> {
    let package_dir_rel = manifest_dir_rel(manifest_rel_path);
    let autobins_enabled = manifest
        .package
        .as_ref()
        .and_then(|package| package.autobins)
        .unwrap_or(true);

    let mut roots = manifest
        .bin
        .iter()
        .filter_map(|target| target.path.as_deref())
        .map(|path| join_rel(&package_dir_rel, path))
        .collect::<Vec<_>>();

    if autobins_enabled {
        let default_main_rel = join_rel(&package_dir_rel, "src/main.rs");
        if crawl.entry(&default_main_rel).is_some() {
            roots.push(default_main_rel);
        }

        let src_bin_dir = path_from_rel(&join_rel(&package_dir_rel, "src/bin"));
        roots.extend(
            crawl
                .entries
                .iter()
                .filter(|entry| entry.kind == G3RsWorkspaceEntryKind::File)
                .filter_map(|entry| {
                    let rel_path = entry.path.rel_path.as_str();
                    if !rel_path.ends_with(".rs") {
                        return None;
                    }
                    let rel_path_obj = Path::new(rel_path);
                    if !rel_path_obj.starts_with(&src_bin_dir) {
                        return None;
                    }
                    let parent_dir = rel_path_obj.parent()?;
                    let file_name = rel_path_obj.file_name()?.to_str()?;
                    let is_root = parent_dir == src_bin_dir || file_name == "main.rs";
                    is_root.then(|| rel_path.to_owned())
                }),
        );
    }

    roots.sort();
    roots.dedup();
    roots
}

fn nearest_manifest_rel_path(crawl: &G3RsWorkspaceCrawl, source_rel_path: &str) -> Option<String> {
    let mut current = Path::new(source_rel_path).parent().map(PathBuf::from)?;

    loop {
        let manifest_rel = if current.as_os_str().is_empty() {
            "Cargo.toml".to_owned()
        } else {
            current.join("Cargo.toml").to_string_lossy().into_owned()
        };

        if crawl.entry(&manifest_rel).is_some() {
            return Some(manifest_rel);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

fn manifest_dir_rel(manifest_rel_path: &str) -> String {
    Path::new(manifest_rel_path)
        .parent()
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn join_rel(base_rel: &str, child_rel: &str) -> String {
    if base_rel.is_empty() {
        child_rel.to_owned()
    } else {
        Path::new(base_rel)
            .join(child_rel)
            .to_string_lossy()
            .into_owned()
    }
}

fn parent_rel_dir(rel_path: &str) -> Option<String> {
    Path::new(rel_path)
        .parent()
        .map(|path| path.to_string_lossy().into_owned())
}

fn path_is_within(rel_path: &str, dir_rel: &str) -> bool {
    if dir_rel.is_empty() {
        return true;
    }

    rel_path == dir_rel || rel_path.starts_with(&format!("{dir_rel}/"))
}

fn path_from_rel(rel_path: &str) -> PathBuf {
    if rel_path.is_empty() {
        PathBuf::new()
    } else {
        PathBuf::from(rel_path)
    }
}
