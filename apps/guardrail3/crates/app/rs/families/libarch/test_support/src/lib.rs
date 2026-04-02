use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_app_rs_family_view::{DirEntry, FamilyView as ProjectTree};
use guardrail3_shared_fs::{
    copy_file, create_dir_all, list_dir, remove_dir_all, write_file as write_fs_file,
};
use tempfile::TempDir;

const ROOT_CARGO: &str = "packages/shared/Cargo.toml";
const ROOT_LIB: &str = "packages/shared/src/lib.rs";
const API_CARGO: &str = "packages/shared/crates/api/Cargo.toml";
const CORE_CARGO: &str = "packages/shared/crates/core/Cargo.toml";
const INFRA_CARGO: &str = "packages/shared/crates/infra/Cargo.toml";
const UTIL_CARGO: &str = "packages/shared/crates/util/Cargo.toml";
const GOLDEN_SHARED_TYPES_CARGO: &str = "packages/shared-types/Cargo.toml";
const GOLDEN_SHARED_TYPES_LIB: &str = "packages/shared-types/src/lib.rs";
const GOLDEN_SHARED_TYPES_API_CARGO: &str = "packages/shared-types/crates/api/Cargo.toml";
const GOLDEN_SHARED_TYPES_CORE_CARGO: &str = "packages/shared-types/crates/core/Cargo.toml";
const GOLDEN_SHARED_TYPES_INFRA_CARGO: &str = "packages/shared-types/crates/infra/Cargo.toml";
const GOLDEN_SHARED_TYPES_API_LIB: &str = "packages/shared-types/crates/api/src/lib.rs";
const GOLDEN_SHARED_TYPES_CORE_LIB: &str = "packages/shared-types/crates/core/src/lib.rs";
const GOLDEN_SHARED_TYPES_INFRA_LIB: &str = "packages/shared-types/crates/infra/src/lib.rs";

pub fn temp_repo() -> TempDir {
    tempfile::tempdir().expect("tempdir")
}

pub fn copy_fixture(fixture_rel: &str) -> TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    copy_dir_recursive(&fixture_root(fixture_rel), tmp.path());
    tmp
}

pub fn temp_root(slug: &str) -> PathBuf {
    let unique = format!(
        "guardrail3-libarch-{slug}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    );
    let root = std::env::temp_dir().join(unique);
    create_dir_all(&root).expect("create temp root");
    root
}

pub fn write_project_files(root: &Path, files: &[(&str, &str)]) {
    for (rel, body) in files {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            create_dir_all(parent).expect("create parent dirs");
        }
        write_fs_file(&path, body).expect("write fixture file");
    }
}

pub fn write_file(root: &Path, rel: &str, body: &str) {
    write_project_files(root, &[(rel, body)]);
}

pub fn walk(root: &Path) -> ProjectTree {
    let walked = walk_project(&RealFileSystem, root);
    ProjectTree::build(
        walked.root().clone(),
        walked.structure(),
        walked.content(),
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    )
}

pub fn remove_dir(root: &Path, rel: &str) {
    remove_dir_all(&root.join(rel)).expect("remove dir");
}

pub fn cleanup(root: &Path) {
    remove_dir_all(root).expect("remove temp root");
}

pub fn promote_golden_shared_types_to_layered_library(root: &Path) {
    write_project_files(
        root,
        &[
            (
                GOLDEN_SHARED_TYPES_CARGO,
                "[package]\nname = \"shared-types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\nmembers = [\"crates/api\", \"crates/core\", \"crates/infra\"]\n\n[workspace.dependencies]\nshared-types-api = { path = \"crates/api\" }\nshared-types-core = { path = \"crates/core\" }\nshared-types-infra = { path = \"crates/infra\" }\n",
            ),
            (
                GOLDEN_SHARED_TYPES_LIB,
                "pub use shared_types_api::{AuditStamp, ServiceMode, TenantSlug};\n",
            ),
            (
                GOLDEN_SHARED_TYPES_API_CARGO,
                "[package]\nname = \"shared-types-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nshared-types-core = { workspace = true }\n",
            ),
            (
                GOLDEN_SHARED_TYPES_API_LIB,
                "pub use shared_types_core::{AuditStamp, ServiceMode, TenantSlug};\n",
            ),
            (
                GOLDEN_SHARED_TYPES_CORE_CARGO,
                "[package]\nname = \"shared-types-core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
            ),
            (
                GOLDEN_SHARED_TYPES_CORE_LIB,
                "pub struct TenantSlug(pub String);\n\npub struct AuditStamp {\n    pub actor: String,\n    pub changed_at: String,\n}\n\npub enum ServiceMode {\n    Healthy,\n    Degraded,\n    Maintenance,\n}\n",
            ),
            (
                GOLDEN_SHARED_TYPES_INFRA_CARGO,
                "[package]\nname = \"shared-types-infra\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nshared-types-core = { workspace = true }\n",
            ),
            (GOLDEN_SHARED_TYPES_INFRA_LIB, "pub struct InfraType;\n"),
        ],
    );
}

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|value| (*value).to_owned()).collect(),
        files.iter().map(|value| (*value).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, &str)>) -> ProjectTree {
    let full_structure: BTreeMap<_, _> = structure
        .into_iter()
        .map(|(rel, entry)| (rel.to_owned(), entry))
        .collect();
    let full_content: BTreeMap<_, _> = content
        .into_iter()
        .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
        .collect();
    ProjectTree::build(
        PathBuf::from("/tmp/libarch"),
        &full_structure,
        &full_content,
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    )
}

pub fn write_flat_library(root: &Path, dependency_count: usize) {
    let dependencies = (0..dependency_count)
        .map(|index| format!("dep{index} = \"1\"\n"))
        .collect::<String>();
    write_file(
        root,
        ROOT_CARGO,
        &format!(
            "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\nresolver = \"2\"\n\n[dependencies]\n{dependencies}"
        ),
    );
    write_file(root, ROOT_LIB, "pub fn facade() -> u8 { 1 }\n");
}

pub fn write_layered_library(root: &Path) {
    write_file(
        root,
        ROOT_CARGO,
        "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\nmembers = [\"crates/api\", \"crates/core\", \"crates/infra\"]\n\n[workspace.dependencies]\napi = { path = \"crates/api\" }\ncore = { path = \"crates/core\" }\ninfra = { path = \"crates/infra\" }\n",
    );
    write_file(root, ROOT_LIB, "pub use api::ApiType;\n");
    write_file(
        root,
        API_CARGO,
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ncore = { workspace = true }\n",
    );
    write_file(
        root,
        CORE_CARGO,
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        INFRA_CARGO,
        "[package]\nname = \"infra\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ncore = { workspace = true }\n",
    );
    write_file(
        root,
        "packages/shared/crates/api/src/lib.rs",
        "pub struct ApiType;\n",
    );
    write_file(
        root,
        "packages/shared/crates/core/src/lib.rs",
        "pub struct CoreType;\n",
    );
    write_file(
        root,
        "packages/shared/crates/infra/src/lib.rs",
        "pub struct InfraType;\n",
    );
}

pub fn write_util_member(root: &Path) {
    write_file(
        root,
        UTIL_CARGO,
        "[package]\nname = \"util\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "packages/shared/crates/util/src/lib.rs",
        "pub struct UtilType;\n",
    );
}

fn fixture_root(fixture_rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(fixture_rel)
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in list_dir(src) {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            create_dir_all(&dst_path).expect("create fixture dir");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _ = copy_file(&src_path, &dst_path).expect("copy fixture file");
        }
    }
}
