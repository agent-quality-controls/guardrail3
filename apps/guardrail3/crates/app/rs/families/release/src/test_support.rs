use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_yaml::Value as YamlValue;

use super::facts::{PublishableCrateFacts, ReleaseEdgeFacts, RepoReleaseFacts, WorkflowFacts};
use super::inputs::{PublishableCrateReleaseInput, ReleaseEdgeInput, RepoReleaseInput};
use super::release_support::extract_workflow_analysis;
use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_adapters_outbound_tool_runner::RealToolChecker;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::{CommandRunResult, ToolChecker};

const GOLDEN_REL: &str = "../../../../../tests/fixtures/r_arch_01/golden";

pub fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN_REL)
}

pub fn copy_fixture() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    copy_dir_recursive(&fixture_root(), tmp.path());
    tmp
}

pub fn write_file(root: &Path, rel: &str, content: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, content).expect("write file");
}

pub fn run_family(root: &Path, thorough: bool) -> Vec<CheckResult> {
    let tree = walk_project(&RealFileSystem, root);
    super::check(&tree, &RealToolChecker, thorough)
}

pub fn errors_by_id<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| result.id == id && result.severity == Severity::Error)
        .collect()
}

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|value| (*value).to_owned()).collect(),
        files: files.iter().map(|value| (*value).to_owned()).collect(),
        symlink_dirs: Vec::new(),
        symlink_files: Vec::new(),
    }
}

pub fn project_tree(
    structure: Vec<(&str, DirEntry)>,
    content: Vec<(&str, &str)>,
    root: PathBuf,
) -> ProjectTree {
    for (rel, entry) in &structure {
        let abs_dir = if rel.is_empty() {
            root.clone()
        } else {
            root.join(rel)
        };
        std::fs::create_dir_all(&abs_dir).expect("create project dir");
        for dir in &entry.dirs {
            std::fs::create_dir_all(abs_dir.join(dir)).expect("create child dir");
        }
    }
    for (rel, body) in &content {
        let abs_path = root.join(rel);
        if let Some(parent) = abs_path.parent() {
            std::fs::create_dir_all(parent).expect("create file parent");
        }
        std::fs::write(&abs_path, body).expect("write project file");
    }

    ProjectTree {
        root,
        structure: structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect(),
        content: content
            .into_iter()
            .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
            .collect(),
    }
}

pub fn temp_root(slug: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    );
    let root = std::env::temp_dir().join(unique);
    std::fs::create_dir_all(&root).expect("create temp root");
    root
}

pub struct StubToolChecker {
    semver_checks_installed: bool,
}

impl StubToolChecker {
    pub const fn new(semver_checks_installed: bool) -> Self {
        Self {
            semver_checks_installed,
        }
    }
}

impl ToolChecker for StubToolChecker {
    fn is_installed(&self, tool: &str) -> bool {
        tool == "cargo-semver-checks" && self.semver_checks_installed
    }

    fn run_cargo_publish_dry_run_outcome(&self, _path: &Path) -> Option<CommandRunResult> {
        None
    }
}

pub fn repo_facts() -> RepoReleaseFacts {
    RepoReleaseFacts {
        cargo_rel_path: "Cargo.toml".to_owned(),
        license_rel_path: None,
        release_plz_rel_path: "release-plz.toml".to_owned(),
        release_plz_exists: false,
        release_plz_parsed: None,
        release_plz_package_names: BTreeSet::new(),
        cliff_rel_path: "cliff.toml".to_owned(),
        cliff_exists: false,
        cliff_parsed: None,
        workflows: Vec::new(),
        publishable_crate_names: BTreeSet::new(),
        publishable_binary_crate_names: BTreeSet::new(),
        publishable_count: 0,
        non_publishable_count: 0,
        semver_checks_installed: false,
        publish_setting: None,
        release_profile_settings: Vec::new(),
    }
}

#[allow(clippy::expect_used)] // reason: test helper
pub fn workflow_from_yaml(rel_path: &str, yaml: &str) -> WorkflowFacts {
    let parsed: YamlValue = serde_yaml::from_str(yaml).expect("valid workflow yaml");
    let analysis = extract_workflow_analysis(&parsed);
    WorkflowFacts {
        rel_path: rel_path.to_owned(),
        analysis,
    }
}

pub fn crate_facts(name: &str) -> PublishableCrateFacts {
    let mut binary_target_names = BTreeSet::new();
    let _ = binary_target_names.insert(name.to_owned());
    PublishableCrateFacts {
        name: name.to_owned(),
        cargo_rel_path: "crates/example/Cargo.toml".to_owned(),
        binary_target_names,
        publishable: true,
        is_binary: false,
        is_library: true,
        description_present: true,
        license_present: true,
        repository_present: true,
        readme_declared_false: false,
        readme_rel_path: "crates/example/README.md".to_owned(),
        readme_exists: true,
        readme_content: Some("# Example\n\n".to_owned() + &"x".repeat(240)),
        keywords_count: Some(3),
        categories_count: Some(1),
        version_string: Some("1.2.3".to_owned()),
        workspace_version: false,
        version_valid: true,
        docs_rs_present: true,
        include_exclude_present: true,
        has_binstall_metadata: false,
        dry_run: None,
    }
}

pub fn edge_facts() -> ReleaseEdgeFacts {
    ReleaseEdgeFacts {
        crate_name: "example".to_owned(),
        cargo_rel_path: "crates/example/Cargo.toml".to_owned(),
        dep_name: "dep".to_owned(),
        dep_package_name: "dep".to_owned(),
        section_label: "dependencies".to_owned(),
        target_label: None,
        has_path: true,
        dep_publishable: true,
        version_req: Some("1.0".to_owned()),
        actual_version: Some("1.2.3".to_owned()),
        version_satisfied: Some(true),
    }
}

pub fn repo_input<'a>(repo: &'a RepoReleaseFacts) -> RepoReleaseInput<'a> {
    RepoReleaseInput::new(repo)
}

pub fn crate_input<'a>(krate: &'a PublishableCrateFacts) -> PublishableCrateReleaseInput<'a> {
    PublishableCrateReleaseInput::new(krate)
}

pub fn edge_input<'a>(edge: &'a ReleaseEdgeFacts) -> ReleaseEdgeInput<'a> {
    ReleaseEdgeInput::new(edge)
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("read fixture dir") {
        let entry = entry.expect("read entry");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path).expect("create dst dir");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _ = std::fs::copy(&src_path, &dst_path).expect("copy fixture file");
        }
    }
}
