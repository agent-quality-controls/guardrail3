use std::path::{Path, PathBuf};

use semver::{Version, VersionReq};

use crate::domain::project_tree::ProjectTree;

#[derive(Debug, Clone, Default)]
pub struct WorkflowStepFacts {
    pub uses: Option<String>,
    pub run_lines: Vec<String>,
    pub env_keys: Vec<String>,
    pub env_values: Vec<String>,
    pub with_values: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkflowJobFacts {
    pub runs_on: Vec<String>,
    pub steps: Vec<WorkflowStepFacts>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkflowAnalysis {
    pub jobs: Vec<WorkflowJobFacts>,
    pub steps: Vec<WorkflowStepFacts>,
    pub scalar_strings: Vec<String>,
    pub env_keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DependencyEdgeFacts {
    pub dep_name: String,
    pub section_label: String,
    pub target_label: Option<String>,
    pub has_path: bool,
    pub version_req: Option<String>,
}

pub fn package_table(parsed: &toml::Value) -> Option<&toml::Value> {
    parsed.get("package")
}

pub fn string_field_present(table: Option<&toml::Value>, field: &str) -> bool {
    table
        .and_then(|table| table.get(field))
        .and_then(toml::Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
}

pub fn bool_field_false(table: Option<&toml::Value>, field: &str) -> bool {
    table
        .and_then(|table| table.get(field))
        .and_then(toml::Value::as_bool)
        .is_some_and(|value| !value)
}

pub fn publish_setting_string(table: Option<&toml::Value>) -> Option<String> {
    let publish = table.and_then(|table| table.get("publish"))?;
    Some(match publish {
        toml::Value::Boolean(value) => value.to_string(),
        toml::Value::Array(values) => format!(
            "[{}]",
            values
                .iter()
                .filter_map(toml::Value::as_str)
                .map(|value| format!("\"{value}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => publish.to_string(),
    })
}

pub fn is_publishable(table: Option<&toml::Value>) -> bool {
    !bool_field_false(table, "publish")
        && !table
            .and_then(|table| table.get("publish"))
            .and_then(toml::Value::as_array)
            .is_some_and(|publish| publish.is_empty())
}

pub fn is_library_crate(tree: &ProjectTree, rel_dir: &str, parsed: &toml::Value) -> bool {
    parsed.get("lib").is_some() || tree.file_exists(&join_under_root(rel_dir, "src/lib.rs"))
}

pub fn is_binary_crate(tree: &ProjectTree, rel_dir: &str, parsed: &toml::Value) -> bool {
    parsed.get("bin").and_then(toml::Value::as_array).is_some()
        || tree.file_exists(&join_under_root(rel_dir, "src/main.rs"))
}

pub fn join_under_root(root_rel_dir: &str, child: &str) -> String {
    if root_rel_dir.is_empty() {
        child.to_owned()
    } else {
        ProjectTree::join_rel(root_rel_dir, child)
    }
}

pub fn resolve_manifest_relative_path(
    tree: &ProjectTree,
    manifest_rel_dir: &str,
    relative: &str,
) -> (String, PathBuf) {
    let abs = if manifest_rel_dir.is_empty() {
        tree.root.join(relative)
    } else {
        tree.root.join(manifest_rel_dir).join(relative)
    };
    let rel = abs
        .strip_prefix(&tree.root)
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|_| relative.to_owned());
    (rel, abs)
}

pub fn readme_target_path(
    tree: &ProjectTree,
    manifest_rel_dir: &str,
    readme_field: Option<&str>,
) -> (String, PathBuf) {
    let readme_rel = readme_field.unwrap_or("README.md");
    resolve_manifest_relative_path(tree, manifest_rel_dir, readme_rel)
}

pub fn valid_semver(version: &str) -> bool {
    Version::parse(version).is_ok()
}

pub fn version_requirement_satisfied(actual: &str, req: &str) -> bool {
    let Ok(actual) = Version::parse(actual) else {
        return false;
    };
    let normalized = if req.trim_start().starts_with(['^', '~', '>', '<', '=']) {
        req.trim().to_owned()
    } else {
        format!("^{req}")
    };
    let Ok(req) = VersionReq::parse(&normalized) else {
        return false;
    };
    req.matches(&actual)
}

pub fn extract_workflow_analysis(parsed: &serde_yaml::Value) -> WorkflowAnalysis {
    let mut analysis = WorkflowAnalysis::default();
    collect_scalar_strings(parsed, &mut analysis.scalar_strings);
    let workflow_env_keys = env_keys_from_value(parsed);
    analysis.env_keys.extend(workflow_env_keys.iter().cloned());
    if let Some(jobs) = yaml_mapping_value(parsed, "jobs").and_then(serde_yaml::Value::as_mapping) {
        for job in jobs.values() {
            let job_facts = collect_job(job, &workflow_env_keys, &mut analysis);
            analysis.jobs.push(job_facts);
        }
    }
    analysis.env_keys.sort();
    analysis.env_keys.dedup();
    analysis.scalar_strings.sort();
    analysis.scalar_strings.dedup();
    analysis
}

pub fn release_plz_step_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(step_invokes_release_plz)
}

pub fn publish_dry_run_step_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(step_invokes_publish_dry_run)
}

pub fn registry_token_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(|step| {
        step.env_keys
            .iter()
            .any(|key| key == "CARGO_REGISTRY_TOKEN")
            && (step_invokes_release_plz_publish(step) || step_invokes_publish_dry_run(step))
    })
}

pub fn binary_release_present(workflow: &WorkflowAnalysis) -> bool {
    let has_release_build = workflow
        .jobs
        .iter()
        .any(|job| job.steps.iter().any(step_builds_release_binary));
    let has_release_action = workflow
        .jobs
        .iter()
        .any(|job| job.steps.iter().any(step_uses_release_action));
    has_release_build && has_release_action
}

pub fn linux_target_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.jobs.iter().any(|job| {
        let release_related = job
            .steps
            .iter()
            .any(|step| step_builds_release_binary(step) || step_uses_release_action(step));
        let linux_target = job.runs_on.iter().any(|value| is_linux_string(value))
            || job.steps.iter().any(step_mentions_linux_target);
        release_related && linux_target
    })
}

pub fn dependency_edges(
    parsed: &toml::Value,
    workspace_versions: &toml::map::Map<String, toml::Value>,
) -> Vec<DependencyEdgeFacts> {
    let mut edges = Vec::new();
    collect_dependency_edges_from_table(
        parsed,
        "dependencies",
        None,
        workspace_versions,
        &mut edges,
    );
    collect_dependency_edges_from_table(
        parsed,
        "build-dependencies",
        None,
        workspace_versions,
        &mut edges,
    );
    if let Some(target_table) = parsed.get("target").and_then(toml::Value::as_table) {
        for (target_name, target_config) in target_table {
            collect_dependency_edges_from_table(
                target_config,
                "dependencies",
                Some(target_name.as_str()),
                workspace_versions,
                &mut edges,
            );
            collect_dependency_edges_from_table(
                target_config,
                "build-dependencies",
                Some(target_name.as_str()),
                workspace_versions,
                &mut edges,
            );
        }
    }
    edges
}

fn collect_dependency_edges_from_table(
    table: &toml::Value,
    section_label: &str,
    target_label: Option<&str>,
    workspace_versions: &toml::map::Map<String, toml::Value>,
    edges: &mut Vec<DependencyEdgeFacts>,
) {
    let Some(section) = table.get(section_label).and_then(toml::Value::as_table) else {
        return;
    };
    for (dep_name, dep_value) in section {
        let mut has_path = dep_value
            .as_table()
            .and_then(|table| table.get("path"))
            .and_then(toml::Value::as_str)
            .is_some();
        let workspace_inherited = dep_value
            .as_table()
            .and_then(|table| table.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
        if workspace_inherited {
            has_path = has_path
                || workspace_versions
                    .get(dep_name)
                    .and_then(extract_workspace_dependency_path)
                    .is_some();
        }
        let version_req = dep_value
            .as_str()
            .map(str::to_owned)
            .or_else(|| {
                dep_value
                    .as_table()
                    .and_then(|table| table.get("version"))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
            })
            .or_else(|| {
                if workspace_inherited {
                    workspace_versions
                        .get(dep_name)
                        .and_then(extract_workspace_dependency_version)
                } else {
                    None
                }
            });
        edges.push(DependencyEdgeFacts {
            dep_name: dep_name.clone(),
            section_label: section_label.to_owned(),
            target_label: target_label.map(str::to_owned),
            has_path,
            version_req,
        });
    }
}

fn extract_workspace_dependency_version(value: &toml::Value) -> Option<String> {
    value.as_str().map(str::to_owned).or_else(|| {
        value
            .as_table()
            .and_then(|table| table.get("version"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
    })
}

fn extract_workspace_dependency_path(value: &toml::Value) -> Option<String> {
    value
        .as_table()
        .and_then(|table| table.get("path"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

fn collect_job(
    job: &serde_yaml::Value,
    inherited_env_keys: &[String],
    analysis: &mut WorkflowAnalysis,
) -> WorkflowJobFacts {
    let mut effective_env_keys = inherited_env_keys.to_vec();
    effective_env_keys.extend(env_keys_from_value(job));
    analysis.env_keys.extend(effective_env_keys.iter().cloned());
    let mut runs_on = Vec::new();
    if let Some(value) = yaml_mapping_value(job, "runs-on") {
        collect_scalar_strings(value, &mut runs_on);
    }
    let mut job_facts = WorkflowJobFacts {
        runs_on,
        steps: Vec::new(),
    };
    if let Some(steps) = yaml_mapping_value(job, "steps").and_then(serde_yaml::Value::as_sequence) {
        for step in steps {
            let step_facts = collect_step(step, &effective_env_keys);
            analysis.steps.push(step_facts.clone());
            analysis
                .env_keys
                .extend(step_facts.env_keys.iter().cloned());
            job_facts.steps.push(step_facts);
        }
    }
    job_facts
}

fn collect_step(step: &serde_yaml::Value, inherited_env_keys: &[String]) -> WorkflowStepFacts {
    let mut facts = WorkflowStepFacts::default();
    facts.env_keys.extend(inherited_env_keys.iter().cloned());
    if let Some(uses) = yaml_mapping_value(step, "uses").and_then(serde_yaml::Value::as_str) {
        facts.uses = Some(uses.to_owned());
    }
    if let Some(run) = yaml_mapping_value(step, "run").and_then(serde_yaml::Value::as_str) {
        facts.run_lines = run
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect();
    }
    if let Some(env) = yaml_mapping_value(step, "env").and_then(serde_yaml::Value::as_mapping) {
        for (key, value) in env {
            if let Some(key) = key.as_str() {
                facts.env_keys.push(key.to_owned());
            }
            if let Some(value) = scalar_as_string(value) {
                facts.env_values.push(value);
            }
        }
    }
    if let Some(with) = yaml_mapping_value(step, "with").and_then(serde_yaml::Value::as_mapping) {
        for value in with.values() {
            if let Some(value) = scalar_as_string(value) {
                facts.with_values.push(value);
            }
        }
    }
    facts
}

fn env_keys_from_value(value: &serde_yaml::Value) -> Vec<String> {
    let mut env_keys = Vec::new();
    if let Some(env) = yaml_mapping_value(value, "env").and_then(serde_yaml::Value::as_mapping) {
        for key in env.keys() {
            if let Some(key) = key.as_str() {
                env_keys.push(key.to_owned());
            }
        }
    }
    env_keys
}

fn collect_scalar_strings(value: &serde_yaml::Value, output: &mut Vec<String>) {
    match value {
        serde_yaml::Value::String(value) => output.push(value.clone()),
        serde_yaml::Value::Sequence(values) => {
            for value in values {
                collect_scalar_strings(value, output);
            }
        }
        serde_yaml::Value::Mapping(values) => {
            for (key, value) in values {
                collect_scalar_strings(key, output);
                collect_scalar_strings(value, output);
            }
        }
        _ => {}
    }
}

fn yaml_mapping_value<'a>(
    value: &'a serde_yaml::Value,
    key: &str,
) -> Option<&'a serde_yaml::Value> {
    value
        .as_mapping()
        .and_then(|mapping| mapping.get(serde_yaml::Value::String(key.to_owned())))
}

fn step_invokes_release_plz(step: &WorkflowStepFacts) -> bool {
    step.uses
        .as_deref()
        .is_some_and(|uses| uses.contains("release-plz/"))
        || step
            .run_lines
            .iter()
            .any(|line| line_has_command(line, |command, _args| command == "release-plz"))
}

fn step_invokes_release_plz_publish(step: &WorkflowStepFacts) -> bool {
    step.uses
        .as_deref()
        .is_some_and(|uses| uses.contains("release-plz/"))
        && step
            .with_values
            .iter()
            .any(|value| value.trim().eq_ignore_ascii_case("release"))
        || step.run_lines.iter().any(|line| {
            line_has_command(line, |command, args| {
                command == "release-plz" && args.first() == Some(&"release")
            })
        })
}

fn step_invokes_publish_dry_run(step: &WorkflowStepFacts) -> bool {
    step.run_lines.iter().any(|line| {
        line_has_command(line, |command, args| {
            command == "cargo"
                && args.iter().any(|arg| *arg == "publish")
                && args.iter().any(|arg| *arg == "--dry-run")
        })
    })
}

fn step_builds_release_binary(step: &WorkflowStepFacts) -> bool {
    step.run_lines.iter().any(|line| {
        line_has_command(line, |command, args| {
            command == "cargo"
                && args.iter().any(|arg| *arg == "build")
                && args.iter().any(|arg| *arg == "--release")
        })
    })
}

fn step_uses_release_action(step: &WorkflowStepFacts) -> bool {
    step.uses
        .as_deref()
        .is_some_and(|uses| uses.contains("action-gh-release"))
}

fn step_mentions_linux_target(step: &WorkflowStepFacts) -> bool {
    step.run_lines.iter().any(|line| {
        let lower = line.to_ascii_lowercase();
        lower.contains("x86_64-unknown-linux")
            || lower.contains("unknown-linux-gnu")
            || lower.contains("unknown-linux-musl")
            || (lower.contains("--target") && lower.contains("linux"))
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
            .all(|ch| ch == '_' || ch.is_ascii_uppercase() || ch.is_ascii_digit())
}

fn shell_wrapper_inner_command(command: &str, args: &[&str]) -> Option<String> {
    if !matches!(command, "bash" | "sh") {
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

fn is_linux_string(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("ubuntu")
        || lower.contains("linux")
        || lower.contains("x86_64-unknown-linux")
        || lower.contains("unknown-linux-gnu")
        || lower.contains("unknown-linux-musl")
}

fn scalar_as_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(value) => Some(value.clone()),
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        serde_yaml::Value::Number(value) => Some(value.to_string()),
        _ => None,
    }
}

pub fn path_file_exists(path: &Path) -> bool {
    crate::fs::metadata(path).is_some()
}
