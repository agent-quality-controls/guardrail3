/// Select config entries from a workspace crawl.
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

/// Find the workspace-root `Cargo.toml` in the crawl result.
pub(crate) fn select_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}

/// Find `clippy.toml` or `.clippy.toml` at the workspace root.
pub(crate) fn select_clippy_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl
        .root_file("clippy.toml")
        .or_else(|| crawl.root_file(".clippy.toml"))
}

/// Find the root `guardrail3.toml`.
pub(crate) fn select_guardrail_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("guardrail3.toml")
}

/// Select all non-test, non-fixture Rust source files under the crawled root.
pub(crate) fn select_ast_source_files(crawl: &G3RsWorkspaceCrawl) -> Vec<&G3RsWorkspaceEntry> {
    let mut files = crawl
        .files_with_extension("rs")
        .into_iter()
        .filter(|entry| is_runtime_source_path(entry.path.rel_path.as_str()))
        .filter(|entry| !is_nested_cargo_root_member(crawl, entry.path.rel_path.as_str()))
        .filter(|entry| !is_fixture_path(entry.path.rel_path.as_str()))
        .filter(|entry| !is_test_path(entry.path.rel_path.as_str()))
        .collect::<Vec<_>>();
    files.sort_by(|left, right| left.path.rel_path.cmp(&right.path.rel_path));
    files
}

fn is_runtime_source_path(rel_path: &str) -> bool {
    rel_path == "src/lib.rs"
        || rel_path == "src/main.rs"
        || rel_path
            .strip_prefix("src/")
            .is_some_and(|rest| rest.ends_with(".rs"))
}

fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/") || rel_path.starts_with("tests/fixtures/")
}

fn is_test_path(rel_path: &str) -> bool {
    rel_path == "tests.rs"
        || rel_path == "src/test.rs"
        || rel_path == "src/tests.rs"
        || rel_path.starts_with("tests/")
        || rel_path.contains("/tests/")
        || rel_path.contains("_tests/")
        || rel_path.contains("/test/")
        || rel_path.contains("__tests__")
        || rel_path.ends_with("/test.rs")
        || rel_path.ends_with("/tests.rs")
        || rel_path.ends_with("_test.rs")
        || rel_path.ends_with("_tests.rs")
}

fn is_nested_cargo_root_member(crawl: &G3RsWorkspaceCrawl, rel_path: &str) -> bool {
    let mut prefix = rel_path.rsplit_once('/').map(|(parent, _)| parent);

    while let Some(dir) = prefix {
        if !dir.is_empty() && crawl.entry(&format!("{dir}/Cargo.toml")).is_some() {
            return true;
        }
        prefix = dir.rsplit_once('/').map(|(parent, _)| parent);
    }

    false
}
