use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use semver::{Version, VersionReq};

use guardrail3_domain_project_tree::ProjectTree;

#[derive(Debug, Clone, Default)]
pub struct WorkflowStepFacts {
    pub uses: Option<String>,
    pub run_lines: Vec<String>,
    pub env_keys: Vec<String>,
    pub env_values: Vec<String>,
    pub with_values: Vec<String>,
    pub env_bindings: BTreeMap<String, String>,
    pub with_bindings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkflowJobFacts {
    pub id: String,
    pub runs_on: Vec<String>,
    pub needs: Vec<String>,
    pub matrix_axes: BTreeMap<String, Vec<String>>,
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
    pub dep_package_name: String,
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
    if parsed.get("bin").and_then(toml::Value::as_array).is_some() {
        return true;
    }

    let autobins_disabled = package_table(parsed)
        .and_then(|package| package.get("autobins"))
        .and_then(toml::Value::as_bool)
        .is_some_and(|autobins| !autobins);

    !autobins_disabled
        && (tree.file_exists(&join_under_root(rel_dir, "src/main.rs"))
            || autodiscovered_bin_exists(tree, rel_dir))
}

pub fn binary_target_names(
    tree: &ProjectTree,
    rel_dir: &str,
    parsed: &toml::Value,
) -> BTreeSet<String> {
    let mut names = BTreeSet::new();

    if let Some(bins) = parsed.get("bin").and_then(toml::Value::as_array) {
        for bin in bins {
            if let Some(name) = bin.get("name").and_then(toml::Value::as_str) {
                let _ = names.insert(name.to_owned());
                continue;
            }
            if let Some(path) = bin.get("path").and_then(toml::Value::as_str)
                && let Some(name) = binary_name_from_path(path)
            {
                let _ = names.insert(name);
            }
        }
    }

    let autobins_disabled = package_table(parsed)
        .and_then(|package| package.get("autobins"))
        .and_then(toml::Value::as_bool)
        .is_some_and(|autobins| !autobins);
    if autobins_disabled {
        return names;
    }

    if tree.file_exists(&join_under_root(rel_dir, "src/main.rs"))
        && let Some(package_name) = package_table(parsed)
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
    {
        let _ = names.insert(package_name.to_owned());
    }

    let src_bin_rel = join_under_root(rel_dir, "src/bin");
    if let Some(src_bin) = tree.dir_contents(&src_bin_rel) {
        for file in &src_bin.files {
            if let Some(name) = binary_name_from_path(file) {
                let _ = names.insert(name);
            }
        }
        for dir in &src_bin.dirs {
            let nested = ProjectTree::join_rel(&src_bin_rel, dir);
            if tree.file_exists(&ProjectTree::join_rel(&nested, "main.rs")) {
                let _ = names.insert(dir.clone());
            }
        }
    }

    names
}

fn binary_name_from_path(path: &str) -> Option<String> {
    let path = Path::new(path);

    if path.file_name().and_then(|name| name.to_str()) == Some("main.rs") {
        return path
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .map(str::to_owned);
    }

    path.file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .map(str::to_owned)
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
        .map(normalize_relative_path)
        .unwrap_or_else(|_| relative.to_owned());
    (rel, abs)
}

fn normalize_relative_path(path: &Path) -> String {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = parts.pop();
            }
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::RootDir | Component::Prefix(_) => {}
        }
    }
    parts.join("/")
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
        for (job_key, job) in jobs {
            let job_id = job_key.as_str().unwrap_or_default();
            let job_facts = collect_job(job_id, job, &workflow_env_keys, &mut analysis);
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
        let workspace_value = workspace_versions.get(dep_name);
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
        let dep_package_name = dep_value
            .as_table()
            .and_then(|table| table.get("package"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
            .or_else(|| {
                if workspace_inherited {
                    workspace_value.and_then(extract_workspace_dependency_package)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| dep_name.clone());
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
            dep_package_name,
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

fn extract_workspace_dependency_package(value: &toml::Value) -> Option<String> {
    value
        .as_table()
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

fn collect_job(
    job_id: &str,
    job: &serde_yaml::Value,
    inherited_env_keys: &[String],
    analysis: &mut WorkflowAnalysis,
) -> WorkflowJobFacts {
    let mut effective_env_keys = inherited_env_keys.to_vec();
    effective_env_keys.extend(env_keys_from_value(job));
    let mut effective_env_bindings = BTreeMap::new();
    effective_env_bindings.extend(env_bindings_from_value(job));
    analysis.env_keys.extend(effective_env_keys.iter().cloned());
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
            let step_facts = collect_step(step, &effective_env_keys, &effective_env_bindings);
            analysis.steps.push(step_facts.clone());
            analysis
                .env_keys
                .extend(step_facts.env_keys.iter().cloned());
            job_facts.steps.push(step_facts);
        }
    }
    job_facts
}

fn collect_step(
    step: &serde_yaml::Value,
    inherited_env_keys: &[String],
    inherited_env_bindings: &BTreeMap<String, String>,
) -> WorkflowStepFacts {
    let mut facts = WorkflowStepFacts::default();
    facts.env_keys.extend(inherited_env_keys.iter().cloned());
    facts.env_bindings.extend(inherited_env_bindings.clone());
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
                let value_string = scalar_as_string(value).unwrap_or_default();
                let _ = facts
                    .env_bindings
                    .insert(key.to_owned(), value_string.clone());
            }
            if let Some(value) = scalar_as_string(value) {
                facts.env_values.push(value);
            }
        }
    }
    if let Some(with) = yaml_mapping_value(step, "with").and_then(serde_yaml::Value::as_mapping) {
        for (key, value) in with {
            if let Some(value) = scalar_as_string(value) {
                facts.with_values.push(value.clone());
                if let Some(key) = key.as_str() {
                    let _ = facts.with_bindings.insert(key.to_owned(), value);
                }
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

fn env_bindings_from_value(value: &serde_yaml::Value) -> BTreeMap<String, String> {
    let mut env_bindings = BTreeMap::new();
    if let Some(env) = yaml_mapping_value(value, "env").and_then(serde_yaml::Value::as_mapping) {
        for (key, value) in env {
            if let Some(key) = key.as_str() {
                let _ = env_bindings
                    .insert(key.to_owned(), scalar_as_string(value).unwrap_or_default());
            }
        }
    }
    env_bindings
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
                            axes.entry(entry_axis.to_owned())
                                .or_default()
                                .extend(entry_values);
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

fn collect_yaml_strings(value: &serde_yaml::Value) -> Vec<String> {
    match value {
        serde_yaml::Value::String(value) => vec![value.clone()],
        serde_yaml::Value::Sequence(values) => {
            values.iter().flat_map(collect_yaml_strings).collect()
        }
        _ => Vec::new(),
    }
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

fn is_linux_string(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("ubuntu")
        || lower.contains("linux")
        || lower.contains("x86_64-unknown-linux")
        || lower.contains("unknown-linux-gnu")
        || lower.contains("unknown-linux-musl")
}

fn autodiscovered_bin_exists(tree: &ProjectTree, rel_dir: &str) -> bool {
    let src_bin_rel = join_under_root(rel_dir, "src/bin");
    let Some(src_bin) = tree.dir_contents(&src_bin_rel) else {
        return false;
    };

    if src_bin.files.iter().any(|file| file.ends_with(".rs")) {
        return true;
    }

    src_bin.dirs.iter().any(|dir| {
        let nested = ProjectTree::join_rel(&src_bin_rel, dir);
        tree.file_exists(&ProjectTree::join_rel(&nested, "main.rs"))
    })
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

fn scalar_as_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(value) => Some(value.clone()),
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        serde_yaml::Value::Number(value) => Some(value.to_string()),
        _ => None,
    }
}

pub fn path_file_exists(path: &Path) -> bool {
    guardrail3_shared_fs::metadata(path).is_some()
}
