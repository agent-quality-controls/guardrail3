use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use g3_workspace_crawl::G3WorkspaceCrawl;
use g3rs_release_types::{G3RsReleaseDryRunOutcome, G3RsReleaseInputFailure};

use super::collect::{WorkflowFacts, input_failure};

/// `(file exists, rel path, parsed config, declared package names)`.
type ReleasePlzParse = (
    bool,
    String,
    Option<release_plz_toml_parser::types::ReleasePlzToml>,
    BTreeSet<String>,
);

/// `(file exists, rel path, parsed config)`.
type CliffParse = (bool, String, Option<cliff_toml_parser::types::CliffToml>);

/// `parse_release_plz` function.
pub(super) fn parse_release_plz(
    crawl: &G3WorkspaceCrawl,
    failures: &mut Vec<G3RsReleaseInputFailure>,
) -> ReleasePlzParse {
    let rel_path = "release-plz.toml".to_owned();
    let Some(entry) = crate::select::select_release_plz_toml(crawl) else {
        return (false, rel_path, None, BTreeSet::new());
    };
    if !entry.readable {
        failures.push(input_failure(
            &entry.path.rel_path,
            "Failed to read release-plz.toml: file is not readable.",
        ));
        return (true, rel_path, None, BTreeSet::new());
    }
    let content = match crate::parse::read_to_string(&entry.path.abs_path) {
        Ok(content) => content,
        Err(error) => {
            failures.push(input_failure(
                &entry.path.rel_path,
                format!("Failed to read release-plz.toml: {error}"),
            ));
            return (true, rel_path, None, BTreeSet::new());
        }
    };
    match crate::parse::parse_release_plz_toml(&content, &entry.path.abs_path) {
        Ok(parsed) => {
            let package_names = parsed
                .package
                .iter()
                .filter_map(|package| package.name.clone())
                .collect::<BTreeSet<_>>();
            (true, rel_path, Some(parsed), package_names)
        }
        Err(error) => {
            failures.push(input_failure(
                &entry.path.rel_path,
                format!("Failed to parse release-plz.toml: {error}"),
            ));
            (true, rel_path, None, BTreeSet::new())
        }
    }
}

/// `parse_cliff` function.
pub(super) fn parse_cliff(
    crawl: &G3WorkspaceCrawl,
    failures: &mut Vec<G3RsReleaseInputFailure>,
) -> CliffParse {
    let rel_path = "cliff.toml".to_owned();
    let Some(entry) = crate::select::select_cliff_toml(crawl) else {
        return (false, rel_path, None);
    };
    if !entry.readable {
        failures.push(input_failure(
            &entry.path.rel_path,
            "Failed to read cliff.toml: file is not readable.",
        ));
        return (true, rel_path, None);
    }
    let content = match crate::parse::read_to_string(&entry.path.abs_path) {
        Ok(content) => content,
        Err(error) => {
            failures.push(input_failure(
                &entry.path.rel_path,
                format!("Failed to read cliff.toml: {error}"),
            ));
            return (true, rel_path, None);
        }
    };
    match crate::parse::parse_cliff_toml(&content, &entry.path.abs_path) {
        Ok(parsed) => (true, rel_path, Some(parsed)),
        Err(error) => {
            failures.push(input_failure(
                &entry.path.rel_path,
                format!("Failed to parse cliff.toml: {error}"),
            ));
            (true, rel_path, None)
        }
    }
}

/// `collect_workflows` function.
pub(super) fn collect_workflows(
    crawl: &G3WorkspaceCrawl,
    failures: &mut Vec<G3RsReleaseInputFailure>,
) -> Vec<WorkflowFacts> {
    let mut workflows = Vec::new();
    for entry in crate::select::select_workflow_entries(crawl) {
        if !entry.readable {
            failures.push(input_failure(
                &entry.path.rel_path,
                "Failed to read workflow YAML: file is not readable.",
            ));
            continue;
        }
        let content = match crate::parse::read_to_string(&entry.path.abs_path) {
            Ok(content) => content,
            Err(error) => {
                failures.push(input_failure(
                    &entry.path.rel_path,
                    format!("Failed to read workflow YAML: {error}"),
                ));
                continue;
            }
        };
        let parsed = match crate::parse::parse_workflow_yaml(&content, &entry.path.abs_path) {
            Ok(parsed) => parsed,
            Err(error) => {
                failures.push(input_failure(
                    &entry.path.rel_path,
                    format!("Failed to parse workflow YAML: {error}"),
                ));
                continue;
            }
        };
        workflows.push(WorkflowFacts {
            rel_path: entry.path.rel_path.clone(),
            analysis: crate::workflow::extract_workflow_analysis(&parsed),
        });
    }
    workflows
}

/// `tool_is_available` function.
pub(super) fn tool_is_available(tool: &str, path_env: Option<&OsStr>) -> bool {
    let Some(path_env) = path_env else {
        return false;
    };

    std::env::split_paths(path_env).any(|dir| {
        let candidate = dir.join(tool);
        let Ok(metadata) = crate::fs::metadata(&candidate) else {
            return false;
        };
        if !metadata.is_file() {
            return false;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            metadata.permissions().mode() & 0o111 != 0
        }
        #[cfg(not(unix))]
        {
            true
        }
    })
}

/// `run_publish_dry_run` function.
#[expect(
    clippy::disallowed_methods,
    reason = "repo.rs is the centralized process boundary for invoking cargo from release ingestion"
)]
pub(super) fn run_publish_dry_run(manifest_path: &Path) -> G3RsReleaseDryRunOutcome {
    let manifest_dir = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let manifest_name = manifest_path
        .file_name()
        .unwrap_or_else(|| OsStr::new("Cargo.toml"));

    match Command::new("cargo")
        .args(["publish", "--dry-run", "--manifest-path"])
        .arg(manifest_name)
        .current_dir(manifest_dir)
        .output()
    {
        Ok(output) if output.status.success() => G3RsReleaseDryRunOutcome::Passed,
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let excerpt = if stderr.trim().is_empty() {
                stdout.lines().take(3).collect::<Vec<_>>().join("; ")
            } else {
                stderr.lines().take(3).collect::<Vec<_>>().join("; ")
            };
            G3RsReleaseDryRunOutcome::Failed(excerpt)
        }
        Err(error) => G3RsReleaseDryRunOutcome::Failed(error.to_string()),
    }
}
