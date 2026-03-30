use std::collections::BTreeSet;
use std::path::Path;

use super::types::{WorkflowAnalysis, WorkflowJobFacts, WorkflowStepFacts};

pub fn release_plz_step_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(step_invokes_release_plz)
}

pub fn publish_dry_run_step_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(step_invokes_publish_dry_run)
}

pub fn registry_token_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(|step| {
        step_has_nonempty_registry_token(step)
            && (step_invokes_release_plz_publish(step) || step_invokes_publish_dry_run(step))
    })
}

pub fn binary_release_present(
    workflow: &WorkflowAnalysis,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    publishable_binary_crate_count: usize,
) -> bool {
    workflow.jobs.iter().enumerate().any(|(index, job)| {
        job_has_release_path(
            workflow,
            index,
            job,
            crate_name,
            cargo_rel_path,
            binary_target_names,
            publishable_binary_crate_count,
        )
    })
}

pub fn linux_target_present(
    workflow: &WorkflowAnalysis,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    publishable_binary_crate_count: usize,
) -> bool {
    workflow.jobs.iter().enumerate().any(|(index, job)| {
        release_path_build_job_indices(
            workflow,
            index,
            job,
            crate_name,
            cargo_rel_path,
            binary_target_names,
            publishable_binary_crate_count,
        )
        .into_iter()
        .any(|job_index| {
            workflow.jobs.get(job_index).is_some_and(|build_job| {
                job_has_linux_target_for_crate(
                    build_job,
                    crate_name,
                    cargo_rel_path,
                    binary_target_names,
                    publishable_binary_crate_count,
                )
            })
        })
    })
}

fn step_invokes_release_plz(step: &WorkflowStepFacts) -> bool {
    step.uses.as_deref().is_some_and(|uses| {
        is_release_plz_action(uses)
            && step
                .with_bindings
                .get("command")
                .is_some_and(|value| is_release_plz_release_flow_command(value))
    }) || step.run_lines.iter().any(|line| {
        line_has_command(line, |command, args| {
            command_basename(command) == "release-plz"
                && release_plz_subcommand(args).is_some_and(is_release_plz_release_flow_command)
        })
    })
}

fn step_invokes_release_plz_publish(step: &WorkflowStepFacts) -> bool {
    step.uses.as_deref().is_some_and(is_release_plz_action)
        && step
            .with_bindings
            .get("command")
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("release"))
        || step.run_lines.iter().any(|line| {
            line_has_command(line, |command, args| {
                command_basename(command) == "release-plz"
                    && release_plz_subcommand(args) == Some("release")
            })
        })
}

fn step_invokes_publish_dry_run(step: &WorkflowStepFacts) -> bool {
    step.run_lines.iter().any(|line| {
        line_has_command(line, |command, args| {
            command_basename(command) == "cargo"
                && cargo_subcommand(args) == Some("publish")
                && args.iter().any(|arg| *arg == "--dry-run")
        })
    })
}

fn step_builds_release_binary_for(
    step: &WorkflowStepFacts,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    publishable_binary_crate_count: usize,
) -> bool {
    step.run_lines.iter().any(|line| {
        line_has_command(line, |command, args| {
            command_basename(command) == "cargo"
                && cargo_subcommand(args) == Some("build")
                && args.iter().any(|arg| *arg == "--release")
                && cargo_build_targets_crate(
                    args,
                    crate_name,
                    cargo_rel_path,
                    binary_target_names,
                    publishable_binary_crate_count,
                )
        })
    })
}

fn step_uses_release_action(step: &WorkflowStepFacts) -> bool {
    step.uses.as_deref().is_some_and(is_github_release_action)
}

fn step_mentions_linux_target(step: &WorkflowStepFacts) -> bool {
    step.run_lines.iter().any(|line| {
        line_has_command(line, |command, args| {
            command_basename(command) == "cargo"
                && cargo_subcommand(args) == Some("build")
                && cargo_target_is_linux(args)
        })
    })
}

fn job_has_release_path(
    workflow: &WorkflowAnalysis,
    index: usize,
    job: &WorkflowJobFacts,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    publishable_binary_crate_count: usize,
) -> bool {
    !release_path_build_job_indices(
        workflow,
        index,
        job,
        crate_name,
        cargo_rel_path,
        binary_target_names,
        publishable_binary_crate_count,
    )
    .is_empty()
}

fn release_path_build_job_indices(
    workflow: &WorkflowAnalysis,
    index: usize,
    job: &WorkflowJobFacts,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    publishable_binary_crate_count: usize,
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
                        publishable_binary_crate_count,
                    )
                })
            })
        })
        .collect()
}

fn release_path_job_indices(workflow: &WorkflowAnalysis, start_index: usize) -> Vec<usize> {
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

fn job_has_linux_target_for_crate(
    job: &WorkflowJobFacts,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    publishable_binary_crate_count: usize,
) -> bool {
    job_runs_on_linux(job)
        || job.steps.iter().any(|step| {
            step_builds_release_binary_for(
                step,
                crate_name,
                cargo_rel_path,
                binary_target_names,
                publishable_binary_crate_count,
            ) && step_mentions_linux_target(step)
        })
}

fn job_runs_on_linux(job: &WorkflowJobFacts) -> bool {
    job.runs_on.iter().any(|value| is_linux_string(value))
        || job
            .runs_on
            .iter()
            .filter_map(|value| matrix_axis_reference(value))
            .any(|axis| {
                job.matrix_axes
                    .get(axis)
                    .is_some_and(|values| values.iter().any(|value| is_linux_string(value)))
            })
}

fn matrix_axis_reference(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    let start = trimmed.find("matrix.")?;
    let axis = &trimmed[(start + "matrix.".len())..];
    let axis = axis
        .split(|ch: char| !(ch == '_' || ch.is_ascii_alphanumeric()))
        .next()
        .unwrap_or_default();
    (!axis.is_empty()).then_some(axis)
}

fn line_has_command<F>(line: &str, matches_command: F) -> bool
where
    F: Fn(&str, &[&str]) -> bool,
{
    line_has_command_impl(line, &matches_command)
}

fn line_has_command_impl(line: &str, matches_command: &dyn Fn(&str, &[&str]) -> bool) -> bool {
    split_shell_segments(line).into_iter().any(|segment| {
        let trimmed = segment.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return false;
        }
        let words = trimmed.split_whitespace().collect::<Vec<_>>();
        if words.is_empty() {
            return false;
        }
        let mut command_index = 0;
        while words
            .get(command_index)
            .is_some_and(|token| is_env_assignment_token(token))
        {
            command_index += 1;
        }
        let Some(command) = words.get(command_index).copied() else {
            return false;
        };
        let args = &words[(command_index + 1)..];
        matches_command(command, args)
            || shell_wrapper_inner_command(command, args)
                .is_some_and(|wrapped| line_has_command_impl(&wrapped, matches_command))
            || env_wrapper_inner_command(command, args)
                .is_some_and(|wrapped| line_has_command_impl(&wrapped, matches_command))
            || shell_control_inner_command(command, args)
                .is_some_and(|wrapped| line_has_command_impl(&wrapped, matches_command))
    })
}

fn split_shell_segments(line: &str) -> Vec<&str> {
    let mut segments = vec![line];
    for separator in ["&&", "||", ";"] {
        segments = segments
            .into_iter()
            .flat_map(|segment| segment.split(separator))
            .collect();
    }
    segments
}

fn is_env_assignment_token(token: &str) -> bool {
    let Some((key, _value)) = token.split_once('=') else {
        return false;
    };
    !key.is_empty()
        && key
            .chars()
            .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn shell_wrapper_inner_command(command: &str, args: &[&str]) -> Option<String> {
    if !matches!(command_basename(command), "bash" | "sh") {
        return None;
    }
    let command_flag = args.iter().position(|arg| matches!(*arg, "-c" | "-lc"))?;
    let wrapped = args
        .iter()
        .skip(command_flag + 1)
        .copied()
        .collect::<Vec<_>>()
        .join(" ");
    Some(
        wrapped
            .trim_matches(|ch| ch == '"' || ch == '\'')
            .to_owned(),
    )
}

fn env_wrapper_inner_command(command: &str, args: &[&str]) -> Option<String> {
    if command_basename(command) != "env" {
        return None;
    }
    let mut index = 0;
    while args
        .get(index)
        .is_some_and(|arg| is_env_assignment_token(arg) || arg.starts_with('-'))
    {
        index += 1;
    }
    let wrapped = args.get(index..)?.join(" ");
    (!wrapped.is_empty()).then_some(wrapped)
}

fn shell_control_inner_command(command: &str, args: &[&str]) -> Option<String> {
    matches!(command, "then" | "do" | "elif")
        .then(|| args.join(" "))
        .filter(|wrapped| !wrapped.is_empty())
}

fn command_basename(command: &str) -> &str {
    command.rsplit('/').next().unwrap_or(command)
}

fn cargo_subcommand<'a>(args: &'a [&'a str]) -> Option<&'a str> {
    let mut index = 0;
    while let Some(arg) = args.get(index).copied() {
        if arg.starts_with('+') {
            index += 1;
            continue;
        }
        if !arg.starts_with('-') {
            return Some(arg);
        }
        index += 1;
        if matches!(
            arg,
            "--manifest-path" | "--config" | "-Z" | "--target" | "--target-dir" | "--color"
        ) {
            index += 1;
        }
    }
    None
}

fn cargo_build_targets_crate(
    args: &[&str],
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    publishable_binary_crate_count: usize,
) -> bool {
    if crate_name.is_empty() {
        return true;
    }

    let mut explicit_package_targets = Vec::new();
    let mut explicit_binary_targets = Vec::new();
    let mut manifest_path = None;
    let mut index = 0;

    while let Some(arg) = args.get(index).copied() {
        match arg {
            "-p" | "--package" => {
                if let Some(value) = args.get(index + 1).copied() {
                    explicit_package_targets.push(value);
                }
                index += 2;
                continue;
            }
            value if value.starts_with("--package=") => {
                explicit_package_targets.push(&value["--package=".len()..]);
            }
            "--manifest-path" => {
                manifest_path = args.get(index + 1).copied();
                index += 2;
                continue;
            }
            value if value.starts_with("--manifest-path=") => {
                manifest_path = Some(&value["--manifest-path=".len()..]);
            }
            "--bin" => {
                if let Some(value) = args.get(index + 1).copied() {
                    explicit_binary_targets.push(value);
                }
                index += 2;
                continue;
            }
            value if value.starts_with("--bin=") => {
                explicit_binary_targets.push(&value["--bin=".len()..]);
            }
            _ => {}
        }
        index += 1;
    }

    if !explicit_package_targets.is_empty() {
        return explicit_package_targets
            .into_iter()
            .any(|target| target == crate_name);
    }

    if !explicit_binary_targets.is_empty() {
        return explicit_binary_targets
            .into_iter()
            .any(|target| binary_target_names.contains(target));
    }

    if let Some(manifest_path) = manifest_path {
        return normalize_rel_path(manifest_path) == normalize_rel_path(cargo_rel_path);
    }

    publishable_binary_crate_count <= 1
}

fn cargo_target_is_linux(args: &[&str]) -> bool {
    let mut index = 0;
    while let Some(arg) = args.get(index).copied() {
        match arg {
            "--target" => {
                if let Some(value) = args.get(index + 1).copied() {
                    return is_linux_string(value);
                }
                return false;
            }
            value if value.starts_with("--target=") => {
                return is_linux_string(&value["--target=".len()..]);
            }
            _ => {}
        }
        index += 1;
    }
    false
}

fn is_release_plz_action(uses: &str) -> bool {
    !uses.starts_with("./") && !uses.starts_with("../") && uses.contains("release-plz/")
}

fn is_github_release_action(uses: &str) -> bool {
    if uses.starts_with("./") || uses.starts_with("../") {
        return false;
    }
    let action = uses.split('@').next().unwrap_or(uses).trim_matches('/');
    let segments = action.split('/').collect::<Vec<_>>();
    segments.len() >= 2
        && segments
            .last()
            .is_some_and(|segment| matches!(*segment, "action-gh-release" | "release-action"))
}

fn release_plz_subcommand<'a>(args: &'a [&'a str]) -> Option<&'a str> {
    let mut index = 0;
    while let Some(arg) = args.get(index).copied() {
        if !arg.starts_with('-') {
            return Some(arg);
        }
        index += 1;
        if matches!(arg, "--config" | "-c") {
            index += 1;
        }
    }
    None
}

fn is_release_plz_release_flow_command(command: &str) -> bool {
    matches!(
        command.trim().to_ascii_lowercase().as_str(),
        "release" | "release-pr"
    )
}

fn step_has_nonempty_registry_token(step: &WorkflowStepFacts) -> bool {
    step.env_bindings
        .get("CARGO_REGISTRY_TOKEN")
        .is_some_and(|value| !value.trim().is_empty())
        || (!step.env_bindings.contains_key("CARGO_REGISTRY_TOKEN")
            && step
                .env_keys
                .iter()
                .any(|key| key == "CARGO_REGISTRY_TOKEN"))
}

fn is_linux_string(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("ubuntu")
        || lower.contains("linux")
        || lower.contains("x86_64-unknown-linux")
        || lower.contains("unknown-linux-gnu")
        || lower.contains("unknown-linux-musl")
}

fn normalize_rel_path(path: &str) -> String {
    Path::new(path)
        .components()
        .filter_map(|component| match component {
            std::path::Component::Normal(value) => Some(value.to_string_lossy().into_owned()),
            std::path::Component::ParentDir => Some("..".to_owned()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}
