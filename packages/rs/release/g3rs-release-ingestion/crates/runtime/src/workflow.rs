use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub(crate) struct WorkflowStepFacts {
    pub(crate) uses: Option<String>,
    pub(crate) run_lines: Vec<String>,
    pub(crate) env_keys: Vec<String>,
    pub(crate) env_bindings: BTreeMap<String, String>,
    pub(crate) with_bindings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct WorkflowJobFacts {
    pub(crate) id: String,
    pub(crate) runs_on: Vec<String>,
    pub(crate) needs: Vec<String>,
    pub(crate) matrix_axes: BTreeMap<String, Vec<String>>,
    pub(crate) steps: Vec<WorkflowStepFacts>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct WorkflowAnalysis {
    pub(crate) jobs: Vec<WorkflowJobFacts>,
    pub(crate) steps: Vec<WorkflowStepFacts>,
}

pub(crate) fn extract_workflow_analysis(parsed: &serde_yaml::Value) -> WorkflowAnalysis {
    let mut analysis = WorkflowAnalysis::default();
    if let Some(jobs) = yaml_mapping_value(parsed, "jobs").and_then(serde_yaml::Value::as_mapping) {
        for (job_key, job) in jobs {
            let job_id = job_key.as_str().unwrap_or_default();
            let job_facts = collect_job(job_id, job, &mut analysis);
            analysis.jobs.push(job_facts);
        }
    }
    analysis
}

pub(crate) fn release_plz_step_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(step_invokes_release_plz)
}

pub(crate) fn publish_dry_run_step_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(step_invokes_publish_dry_run)
}

pub(crate) fn registry_token_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(|step| {
        step_has_nonempty_registry_token(step)
            && (step_invokes_release_plz_publish(step) || step_invokes_publish_dry_run(step))
    })
}

pub(crate) fn binary_release_present(
    workflow: &WorkflowAnalysis,
    crate_name: &str,
    cargo_rel_path: &str,
    binary_target_names: &BTreeSet<String>,
    publishable_binary_crate_count: usize,
) -> bool {
    workflow.jobs.iter().enumerate().any(|(index, job)| {
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
    })
}

pub(crate) fn linux_target_present(
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

fn collect_job(
    job_id: &str,
    job: &serde_yaml::Value,
    analysis: &mut WorkflowAnalysis,
) -> WorkflowJobFacts {
    let mut runs_on = Vec::new();
    if let Some(value) = yaml_mapping_value(job, "runs-on") {
        collect_scalar_strings(value, &mut runs_on);
    }
    let needs = yaml_mapping_value(job, "needs")
        .map(collect_yaml_strings)
        .unwrap_or_default();
    let mut job_facts = WorkflowJobFacts {
        id: job_id.to_owned(),
        runs_on,
        needs,
        matrix_axes: matrix_axes_from_job(job),
        steps: Vec::new(),
    };
    if let Some(steps) = yaml_mapping_value(job, "steps").and_then(serde_yaml::Value::as_sequence) {
        for step in steps {
            let step_facts = collect_step(step);
            analysis.steps.push(step_facts.clone());
            job_facts.steps.push(step_facts);
        }
    }
    job_facts
}

fn collect_step(step: &serde_yaml::Value) -> WorkflowStepFacts {
    let mut facts = WorkflowStepFacts::default();
    if let Some(uses) = yaml_mapping_value(step, "uses").and_then(serde_yaml::Value::as_str) {
        facts.uses = Some(uses.to_owned());
    }
    if let Some(run) = yaml_mapping_value(step, "run").and_then(serde_yaml::Value::as_str) {
        facts.run_lines = normalize_run_lines(run);
    }
    if let Some(env) = yaml_mapping_value(step, "env").and_then(serde_yaml::Value::as_mapping) {
        for (key, value) in env {
            if let Some(key) = key.as_str() {
                facts.env_keys.push(key.to_owned());
                let _ = facts
                    .env_bindings
                    .insert(key.to_owned(), scalar_as_string(value).unwrap_or_default());
            }
        }
    }
    if let Some(with) = yaml_mapping_value(step, "with").and_then(serde_yaml::Value::as_mapping) {
        for (key, value) in with {
            if let Some(key) = key.as_str()
                && let Some(value) = scalar_as_string(value)
            {
                let _ = facts.with_bindings.insert(key.to_owned(), value);
            }
        }
    }
    facts
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
            if let Some(needed_index) = workflow.jobs.iter().position(|candidate| candidate.id == *need) {
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
    let Some((key, _)) = token.split_once('=') else {
        return false;
    };
    !key.is_empty() && key.chars().all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn shell_wrapper_inner_command(command: &str, args: &[&str]) -> Option<String> {
    if !matches!(command_basename(command), "bash" | "sh") {
        return None;
    }
    let command_flag = args.iter().position(|arg| matches!(*arg, "-c" | "-lc"))?;
    let wrapped = args.iter().skip(command_flag + 1).copied().collect::<Vec<_>>().join(" ");
    Some(wrapped.trim_matches(|ch| ch == '"' || ch == '\'').to_owned())
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
        return explicit_package_targets.into_iter().any(|target| target == crate_name);
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
    action_slug(uses).is_some_and(|slug| slug == "release-plz/action")
}

fn is_github_release_action(uses: &str) -> bool {
    action_slug(uses).is_some_and(|slug| {
        matches!(slug, "softprops/action-gh-release" | "ncipollo/release-action")
    })
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
    matches!(command.trim().to_ascii_lowercase().as_str(), "release" | "release-pr")
}

fn step_has_nonempty_registry_token(step: &WorkflowStepFacts) -> bool {
    step.env_bindings
        .get("CARGO_REGISTRY_TOKEN")
        .is_some_and(|value| !value.trim().is_empty())
        || (!step.env_bindings.contains_key("CARGO_REGISTRY_TOKEN")
            && step.env_keys.iter().any(|key| key == "CARGO_REGISTRY_TOKEN"))
}

fn is_linux_string(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("ubuntu")
        || lower.contains("linux")
        || lower.contains("x86_64-unknown-linux")
        || lower.contains("unknown-linux-gnu")
        || lower.contains("unknown-linux-musl")
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

fn action_slug(uses: &str) -> Option<&str> {
    if uses.starts_with("./") || uses.starts_with("../") {
        return None;
    }
    let slug = uses.split('@').next().unwrap_or(uses).trim_matches('/');
    let mut segments = slug.split('/');
    let owner = segments.next()?;
    let repo = segments.next()?;
    segments
        .next()
        .is_none()
        .then_some(slug)
        .filter(|_| !owner.is_empty() && !repo.is_empty())
}

fn yaml_mapping_value<'a>(
    value: &'a serde_yaml::Value,
    key: &str,
) -> Option<&'a serde_yaml::Value> {
    value
        .as_mapping()
        .and_then(|mapping| mapping.get(serde_yaml::Value::String(key.to_owned())))
}

fn collect_scalar_strings(value: &serde_yaml::Value, output: &mut Vec<String>) {
    match value {
        serde_yaml::Value::String(value) => output.push(value.clone()),
        serde_yaml::Value::Sequence(values) => {
            for value in values {
                collect_scalar_strings(value, output);
            }
        }
        _ => {}
    }
}

fn collect_yaml_strings(value: &serde_yaml::Value) -> Vec<String> {
    match value {
        serde_yaml::Value::String(value) => vec![value.clone()],
        serde_yaml::Value::Sequence(values) => values.iter().flat_map(collect_yaml_strings).collect(),
        _ => Vec::new(),
    }
}

fn matrix_axes_from_job(job: &serde_yaml::Value) -> BTreeMap<String, Vec<String>> {
    let mut axes = BTreeMap::new();
    let Some(matrix) = yaml_mapping_value(job, "strategy")
        .and_then(|strategy| yaml_mapping_value(strategy, "matrix"))
        .and_then(serde_yaml::Value::as_mapping)
    else {
        return axes;
    };
    for (key, value) in matrix {
        let Some(axis) = key.as_str() else {
            continue;
        };
        if axis == "include" {
            if let Some(entries) = value.as_sequence() {
                for entry in entries {
                    let Some(entry_map) = entry.as_mapping() else {
                        continue;
                    };
                    for (entry_key, entry_value) in entry_map {
                        let Some(entry_axis) = entry_key.as_str() else {
                            continue;
                        };
                        let entry_values = collect_yaml_strings(entry_value);
                        if !entry_values.is_empty() {
                            axes.entry(entry_axis.to_owned()).or_default().extend(entry_values);
                        }
                    }
                }
            }
            continue;
        }
        let values = collect_yaml_strings(value);
        if !values.is_empty() {
            let _ = axes.insert(axis.to_owned(), values);
        }
    }
    axes
}

fn normalize_run_lines(run: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for raw_line in run.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let continued = trimmed.strip_suffix('\\');
        let piece = continued.unwrap_or(trimmed).trim();
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(piece);
        if continued.is_none() {
            lines.push(current.clone());
            current.clear();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn scalar_as_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(value) => Some(value.clone()),
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        serde_yaml::Value::Number(value) => Some(value.to_string()),
        _ => None,
    }
}
