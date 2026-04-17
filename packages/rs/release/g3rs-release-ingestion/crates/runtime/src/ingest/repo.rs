use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use g3rs_release_types::{G3RsReleaseDryRunOutcome, G3RsReleaseInputFailure};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use super::collect::{WorkflowFacts, input_failure};

pub(super) fn parse_release_plz(
    crawl: &G3RsWorkspaceCrawl,
    failures: &mut Vec<G3RsReleaseInputFailure>,
) -> (
    bool,
    String,
    Option<release_plz_toml_parser::types::ReleasePlzToml>,
    BTreeSet<String>,
) {
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

pub(super) fn parse_cliff(
    crawl: &G3RsWorkspaceCrawl,
    failures: &mut Vec<G3RsReleaseInputFailure>,
) -> (bool, String, Option<cliff_toml_parser::types::CliffToml>) {
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

pub(super) fn collect_workflows(
    crawl: &G3RsWorkspaceCrawl,
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

pub(super) fn release_profile_settings(raw_root: &toml::Value) -> Vec<String> {
    raw_root
        .get("profile")
        .and_then(|value| value.get("release"))
        .and_then(|value| value.as_table())
        .map(|table| {
            table
                .iter()
                .map(|(key, value)| format!("{key} = {value}"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub(super) fn publish_setting_string(raw_root: &toml::Value) -> Option<String> {
    let publish = raw_root
        .get("workspace")
        .and_then(|value| value.get("package"))
        .and_then(|value| value.get("publish"))
        .or_else(|| raw_root.get("package").and_then(|value| value.get("publish")))?;

    Some(match publish {
        toml::Value::Boolean(value) => value.to_string(),
        toml::Value::Array(values) => format!(
            "[{}]",
            values
                .iter()
                .filter_map(|value| value.as_str())
                .map(|value| format!("\"{value}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => publish.to_string(),
    })
}

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
