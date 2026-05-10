use g3ts_jscpd_types::G3TsJscpdChecksInput;
use tempfile::TempDir;

/// Pair of `(rel_path, content)` describing one fixture file written into a
/// test workspace.
pub(super) type SeedFile<'a> = (&'a str, &'a str);

/// Cargo manifest stub that the workspace crawl requires to recognize the
/// directory as a workspace root.
const CARGO_TOML_STUB: &str =
    "[package]\nname = \"fixture\"\nversion = \"0.0.0\"\nedition = \"2021\"\n";

/// Ensure `crawl_root` and any ancestor up to the tempdir root contains a
/// `Cargo.toml` so that `g3_workspace_crawl::crawl` succeeds in tests.
#[expect(
    clippy::disallowed_methods,
    reason = "Seeds a Cargo.toml stub directly via std::fs in test fixtures; routing through \
              the production fs port would require the test sidecar to call a sibling module, \
              which is forbidden by the runtime-assertions-split rule"
)]
fn seed_cargo_manifest_if_missing(crawl_root: &std::path::Path) {
    let manifest = crawl_root.join("Cargo.toml");
    if !manifest.exists() {
        if let Some(parent) = manifest.parent() {
            std::fs::create_dir_all(parent).expect("create temporary cargo manifest dir");
        }
        std::fs::write(&manifest, CARGO_TOML_STUB).expect("write Cargo.toml stub for crawl");
    }
}

/// Write `content` to `root.join(rel_path)`, creating any missing parent
/// directories. Used by ingestion runtime tests.
///
/// # Panics
///
/// Panics if the parent directory cannot be created or the file cannot be
/// written.
#[expect(
    clippy::disallowed_methods,
    reason = "Test fixture helper writes to a tempdir; routing through the production fs port \
              would require the test sidecar to call a sibling module, which is forbidden by \
              the runtime-assertions-split rule"
)]
pub(super) fn write(root: &std::path::Path, rel_path: &str, content: &str) {
    let path = root.join(rel_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create temporary test directory");
    }
    std::fs::write(path, content).expect("write temporary test file");
}

/// Materialize a temporary workspace seeded with `(rel_path, content)` files,
/// crawl from `crawl_subpath`, and ingest the result.
///
/// `crawl_subpath` is joined onto the tempdir before crawling; passing an
/// empty string crawls from the workspace root.
///
/// Returns the populated tempdir and the ingestion input so callers can keep
/// the directory alive for follow-up assertions.
///
/// # Panics
///
/// Panics if the tempdir cannot be created, any seed file cannot be written,
/// or the crawl fails.
pub(super) fn ingest_with_files(
    files: &[SeedFile<'_>],
    crawl_subpath: &str,
) -> (TempDir, G3TsJscpdChecksInput) {
    let tempdir = tempfile::tempdir().expect("create temporary workspace");
    for (rel_path, content) in files {
        write(tempdir.path(), rel_path, content);
    }

    let crawl_root = if crawl_subpath.is_empty() {
        tempdir.path().to_path_buf()
    } else {
        tempdir.path().join(crawl_subpath)
    };
    seed_cargo_manifest_if_missing(&crawl_root);
    let crawl = g3_workspace_crawl::crawl(&crawl_root).expect("crawl temporary workspace");
    let input = super::super::ingest_for_config_checks(&crawl);
    (tempdir, input)
}
