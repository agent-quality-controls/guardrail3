use std::collections::BTreeSet;
use std::path::Path;

use g3rs_release_types::{
    G3RsReleaseConfigCrate, G3RsReleaseConfigRepo, G3RsReleaseWorkflowAnalysis,
    G3RsReleaseWorkflowJob, G3RsReleaseWorkflowStep,
};

/// `crate_binary_release_workflow_present` function.
pub(crate) fn crate_binary_release_workflow_present(
    repo: &G3RsReleaseConfigRepo,
    krate: &G3RsReleaseConfigCrate,
    binary_crate_count: usize,
) -> bool {
    repo.workflows.iter().any(|workflow| {
        binary_release_present(
            &workflow.analysis,
            &krate.name,
            &krate.cargo_rel_path,
            &krate.binary_target_names,
            binary_crate_count,
        )
    })
}

/// `crate_linux_release_target_present` function.
pub(crate) fn crate_linux_release_target_present(
    repo: &G3RsReleaseConfigRepo,
    krate: &G3RsReleaseConfigCrate,
    binary_crate_count: usize,
) -> bool {
    repo.workflows.iter().any(|workflow| {
        linux_target_present(
            &workflow.analysis,
            &krate.name,
            &krate.cargo_rel_path,
            &krate.binary_target_names,
            binary_crate_count,
        )
    })
}

/// `binary_release_present` function.
fn binary_release_present(
    workflow: &G3RsReleaseWorkflowAnalysis,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    binary_crate_count: usize,
) -> bool {
    workflow.jobs.iter().enumerate().any(|(index, job)| {
        !release_path_build_job_indices(
            workflow,
            index,
            job,
            crate_name,
            cargo_rel_path,
            binary_target_names,
            binary_crate_count,
        )
        .is_empty()
    })
}

/// `linux_target_present` function.
fn linux_target_present(
    workflow: &G3RsReleaseWorkflowAnalysis,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    binary_crate_count: usize,
) -> bool {
    workflow.jobs.iter().enumerate().any(|(index, job)| {
        release_path_build_job_indices(
            workflow,
            index,
            job,
            crate_name,
            cargo_rel_path,
            binary_target_names,
            binary_crate_count,
        )
        .into_iter()
        .any(|job_index| {
            workflow.jobs.get(job_index).is_some_and(|build_job| {
                job_has_linux_target_for_crate(
                    build_job,
                    crate_name,
                    cargo_rel_path,
                    binary_target_names,
                    binary_crate_count,
                )
            })
        })
    })
}

/// `release_path_build_job_indices` function.
fn release_path_build_job_indices(
    workflow: &G3RsReleaseWorkflowAnalysis,
    index: usize,
    job: &G3RsReleaseWorkflowJob,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    binary_crate_count: usize,
) -> Vec<usize> {
    if !job.steps.iter().any(step_uses_release_action) {
        return Vec::new();
    }

    release_path_job_indices(workflow, index)
        .into_iter()
        .filter(|job_index| {
            workflow.jobs.get(*job_index).is_some_and(|candidate| {
                candidate.steps.iter().any(|step| {
                    step_builds_release_binary_for(
                        step,
                        crate_name,
                        cargo_rel_path,
                        binary_target_names,
                        binary_crate_count,
                    )
                })
            })
        })
        .collect()
}

/// `release_path_job_indices` function.
fn release_path_job_indices(
    workflow: &G3RsReleaseWorkflowAnalysis,
    start_index: usize,
) -> Vec<usize> {
    let mut visited = Vec::new();
    let mut stack = vec![start_index];
    while let Some(index) = stack.pop() {
        if visited.contains(&index) {
            continue;
        }
        visited.push(index);
        let Some(job) = workflow.jobs.get(index) else {
            continue;
        };
        for need in &job.needs {
            if let Some(needed_index) = workflow
                .jobs
                .iter()
                .position(|candidate| candidate.id == *need)
            {
                stack.push(needed_index);
            }
        }
    }
    visited
}

/// `job_has_linux_target_for_crate` function.
fn job_has_linux_target_for_crate(
    job: &G3RsReleaseWorkflowJob,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    binary_crate_count: usize,
) -> bool {
    job.steps.iter().any(|step| {
        step_targets_linux_binary_for(
            step,
            crate_name,
            cargo_rel_path,
            binary_target_names,
            binary_crate_count,
        )
    })
}

/// `step_uses_release_action` function.
fn step_uses_release_action(step: &G3RsReleaseWorkflowStep) -> bool {
    step.uses.as_deref().is_some_and(|uses| {
        uses == "taiki-e/upload-rust-binary-action@v1"
            || uses.starts_with("softprops/action-gh-release@")
    })
}

/// `step_builds_release_binary_for` function.
fn step_builds_release_binary_for(
    step: &G3RsReleaseWorkflowStep,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    binary_crate_count: usize,
) -> bool {
    step.run_lines.iter().any(|line| {
        line_has_command(line, |command, args| {
            command_basename(command) == "cargo"
                && cargo_subcommand(args) == Some("build")
                && args.contains(&"--release")
                && command_mentions_crate(
                    args,
                    crate_name,
                    cargo_rel_path,
                    binary_target_names,
                    binary_crate_count,
                )
        })
    })
}

/// `step_targets_linux_binary_for` function.
fn step_targets_linux_binary_for(
    step: &G3RsReleaseWorkflowStep,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    binary_crate_count: usize,
) -> bool {
    let has_linux_target = step.run_lines.iter().any(|line| {
        line_has_command(line, |command, args| {
            command_basename(command) == "cargo"
                && cargo_subcommand(args) == Some("build")
                && args.contains(&"--target")
                && args.iter().any(|arg| arg.contains("linux"))
                && command_mentions_crate(
                    args,
                    crate_name,
                    cargo_rel_path,
                    binary_target_names,
                    binary_crate_count,
                )
        })
    });
    has_linux_target
        || step
            .with_bindings
            .get("target")
            .is_some_and(|target| target.contains("linux"))
}

/// `command_mentions_crate` function.
fn command_mentions_crate(
    args: &[&str],
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    binary_crate_count: usize,
) -> bool {
    if binary_crate_count == 1 {
        return true;
    }

    args.windows(2).any(|window| match window {
        ["-p" | "--package", value] => *value == crate_name,
        ["--manifest-path", value] => manifest_path_matches(value, cargo_rel_path),
        _ => false,
    }) || args
        .windows(2)
        .any(|window| matches!(window, ["--bin", value] if binary_target_names.contains(*value)))
}

/// `manifest_path_matches` function.
fn manifest_path_matches(value: &str, cargo_rel_path: &str) -> bool {
    Path::new(value).ends_with(Path::new(cargo_rel_path))
}

/// `cargo_subcommand` function.
fn cargo_subcommand<'a>(args: &'a [&'a str]) -> Option<&'a str> {
    args.iter().copied().find(|arg| !arg.starts_with('-'))
}

/// `command_basename` function.
fn command_basename(command: &str) -> &str {
    Path::new(command)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(command)
}

/// `line_has_command` function.
fn line_has_command<F>(line: &str, predicate: F) -> bool
where
    F: Fn(&str, &[&str]) -> bool,
{
    let tokens = line.split_whitespace().collect::<Vec<_>>();
    let Some((command, args)) = tokens.split_first() else {
        return false;
    };
    predicate(command, args)
}
