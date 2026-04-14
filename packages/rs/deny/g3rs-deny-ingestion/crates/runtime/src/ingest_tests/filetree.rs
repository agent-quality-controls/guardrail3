use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_deny_filetree_checks_assertions::{
    rs_deny_filetree_01_coverage as coverage_assertions,
    rs_deny_filetree_03_shadowing as shadowing_assertions,
};
use tempfile::tempdir;

fn git_init(path: &Path) {
    let _ = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture file");
}

#[test]
fn pipeline_reports_missing_deny_config() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[coverage_assertions::error_no_file(
            "workspace root uncovered by deny config",
            "workspace root `.` is not covered by any allowed deny config.",
            false,
        )],
    );
}

#[test]
fn pipeline_inventories_selected_root_deny_config() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[coverage_assertions::info(
            "workspace root covered by deny config",
            "workspace root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        )],
    );
    shadowing_assertions::assert_no_findings(&results);
}

#[test]
fn pipeline_reports_same_root_conflicts() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join(".deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join(".cargo/deny.toml"), "[advisories]\nyanked = \"warn\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[coverage_assertions::info(
            "workspace root covered by deny config",
            "workspace root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        )],
    );
    shadowing_assertions::assert_findings(
        &results,
        &[shadowing_assertions::error(
            "multiple deny configs at one policy root",
            "`.` has multiple accepted deny configs: .cargo/deny.toml, .deny.toml, deny.toml.",
            ".cargo/deny.toml",
            false,
        )],
    );
}

#[test]
fn pipeline_reports_selected_deny_parse_failures() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[
            coverage_assertions::error(
                "deny input failure",
                "Failed to parse root deny config `deny.toml` for deny checks: invalid deny.toml: TOML parse error at line 1, column 12\n  |\n1 | [advisories\n  |            ^\ninvalid table header\nexpected `.`, `]`\n",
                "deny.toml",
                false,
            ),
            coverage_assertions::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

#[test]
fn pipeline_reports_rust_policy_parse_failures_without_hiding_selected_coverage() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join("guardrail3-rs.toml"), "profile = \"invalid\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[
            coverage_assertions::error(
                "deny rust policy is not parseable",
                "Failed to parse root Rust policy `guardrail3-rs.toml` for deny profile resolution: invalid guardrail3-rs.toml: TOML parse error at line 1, column 11\n  |\n1 | profile = \"invalid\"\n  |           ^^^^^^^^^\nunknown variant `invalid`, expected `service` or `library`\n",
                "guardrail3-rs.toml",
                false,
            ),
            coverage_assertions::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

#[test]
fn pipeline_reports_unreadable_selected_deny_file() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    let deny_path = root.join("deny.toml");
    write(&deny_path, "[advisories]\nyanked = \"warn\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let mut permissions = fs::metadata(&deny_path)
        .expect("fixture file should exist before chmod")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&deny_path, permissions).expect("chmod fixture unreadable");

    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[
            coverage_assertions::error(
                "deny input failure",
                "Failed to read root deny config `deny.toml` for deny checks: file is not readable",
                "deny.toml",
                false,
            ),
            coverage_assertions::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

#[test]
fn pipeline_reports_unreadable_rust_policy() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    let guardrail_path = root.join("guardrail3-rs.toml");
    write(&guardrail_path, "profile = \"service\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let mut permissions = fs::metadata(&guardrail_path)
        .expect("fixture file should exist before chmod")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&guardrail_path, permissions).expect("chmod fixture unreadable");

    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[
            coverage_assertions::error(
                "deny rust policy is not parseable",
                "Failed to parse root Rust policy `guardrail3-rs.toml` for deny profile resolution: file is not readable",
                "guardrail3-rs.toml",
                false,
            ),
            coverage_assertions::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

#[test]
fn pipeline_reports_shadowed_root_parse_failures() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join(".deny.toml"), "[advisories");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[
            coverage_assertions::error(
                "deny input failure",
                "Failed to parse root deny config `.deny.toml` for deny checks: invalid deny.toml: TOML parse error at line 1, column 12\n  |\n1 | [advisories\n  |            ^\ninvalid table header\nexpected `.`, `]`\n",
                ".deny.toml",
                false,
            ),
            coverage_assertions::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
    shadowing_assertions::assert_findings(
        &results,
        &[shadowing_assertions::error(
            "multiple deny configs at one policy root",
            "`.` has multiple accepted deny configs: .deny.toml, deny.toml.",
            ".deny.toml",
            false,
        )],
    );
}

#[test]
fn pipeline_reports_shadowed_root_unreadable_failures() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    let dot_deny_path = root.join(".deny.toml");
    write(&dot_deny_path, "[advisories]\nyanked = \"warn\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let mut permissions = fs::metadata(&dot_deny_path)
        .expect("fixture file should exist before chmod")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&dot_deny_path, permissions).expect("chmod fixture unreadable");

    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);

    coverage_assertions::assert_findings(
        &results,
        &[
            coverage_assertions::error(
                "deny input failure",
                "Failed to read root deny config `.deny.toml` for deny checks: file is not readable",
                ".deny.toml",
                false,
            ),
            coverage_assertions::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
    shadowing_assertions::assert_findings(
        &results,
        &[shadowing_assertions::error(
            "multiple deny configs at one policy root",
            "`.` has multiple accepted deny configs: .deny.toml, deny.toml.",
            ".deny.toml",
            false,
        )],
    );
}
