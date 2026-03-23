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
pub struct WorkflowAnalysis {
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
    table.and_then(|table| table.get(field))
        .and_then(toml::Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
}

pub fn bool_field_false(table: Option<&toml::Value>, field: &str) -> bool {
    table.and_then(|table| table.get(field))
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
    if let Some(jobs) = yaml_mapping_value(parsed, "jobs").and_then(serde_yaml::Value::as_mapping) {
        for job in jobs.values() {
            collect_job(job, &mut analysis);
        }
    }
    analysis.env_keys.sort();
    analysis.env_keys.dedup();
    analysis.scalar_strings.sort();
    analysis.scalar_strings.dedup();
    analysis
}

pub fn release_plz_step_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(|step| {
        step.uses
            .as_deref()
            .is_some_and(|uses| uses.contains("release-plz/"))
            || step
                .run_lines
                .iter()
                .any(|line| line.contains("release-plz"))
    })
}

pub fn publish_dry_run_step_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.steps.iter().any(|step| {
        step.run_lines
            .iter()
            .any(|line| line.contains("cargo publish --dry-run"))
    })
}

pub fn registry_token_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.env_keys.iter().any(|key| key == "CARGO_REGISTRY_TOKEN")
        || workflow
            .scalar_strings
            .iter()
            .any(|value| value.contains("CARGO_REGISTRY_TOKEN"))
}

pub fn binary_release_present(workflow: &WorkflowAnalysis) -> bool {
    let has_release_build = workflow.steps.iter().any(|step| {
        step.run_lines
            .iter()
            .any(|line| line.contains("--release") && line.contains("build"))
    });
    let has_release_action = workflow.steps.iter().any(|step| {
        step.uses
            .as_deref()
            .is_some_and(|uses| uses.contains("action-gh-release"))
    });
    has_release_build && has_release_action
}

pub fn linux_target_present(workflow: &WorkflowAnalysis) -> bool {
    workflow.scalar_strings.iter().any(|value| {
        let lower = value.to_ascii_lowercase();
        lower.contains("ubuntu")
            || lower.contains("linux")
            || lower.contains("x86_64-unknown-linux")
            || lower.contains("amd64")
    })
}

pub fn dependency_edges(parsed: &toml::Value, workspace_versions: &toml::map::Map<String, toml::Value>) -> Vec<DependencyEdgeFacts> {
    let mut edges = Vec::new();
    collect_dependency_edges_from_table(parsed, "dependencies", None, workspace_versions, &mut edges);
    collect_dependency_edges_from_table(parsed, "build-dependencies", None, workspace_versions, &mut edges);
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
        let has_path = dep_value
            .as_table()
            .and_then(|table| table.get("path"))
            .and_then(toml::Value::as_str)
            .is_some();
        let workspace_inherited = dep_value
            .as_table()
            .and_then(|table| table.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false);
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
    value
        .as_str()
        .map(str::to_owned)
        .or_else(|| {
            value
                .as_table()
                .and_then(|table| table.get("version"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
        })
}

fn collect_job(job: &serde_yaml::Value, analysis: &mut WorkflowAnalysis) {
    collect_env_keys(job, analysis);
    if let Some(steps) = yaml_mapping_value(job, "steps").and_then(serde_yaml::Value::as_sequence) {
        for step in steps {
            analysis.steps.push(collect_step(step));
        }
    }
}

fn collect_step(step: &serde_yaml::Value) -> WorkflowStepFacts {
    let mut facts = WorkflowStepFacts::default();
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

fn collect_env_keys(value: &serde_yaml::Value, analysis: &mut WorkflowAnalysis) {
    if let Some(env) = yaml_mapping_value(value, "env").and_then(serde_yaml::Value::as_mapping) {
        for key in env.keys() {
            if let Some(key) = key.as_str() {
                analysis.env_keys.push(key.to_owned());
            }
        }
    }
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

fn yaml_mapping_value<'a>(value: &'a serde_yaml::Value, key: &str) -> Option<&'a serde_yaml::Value> {
    value
        .as_mapping()
        .and_then(|mapping| mapping.get(serde_yaml::Value::String(key.to_owned())))
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
