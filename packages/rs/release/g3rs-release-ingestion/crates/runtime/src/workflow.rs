use g3rs_release_types::{
    G3RsReleaseWorkflowAnalysis as WorkflowAnalysis, G3RsReleaseWorkflowJob as WorkflowJobFacts,
    G3RsReleaseWorkflowStep as WorkflowStepFacts,
};
use std::collections::BTreeMap;

/// Map of matrix axis name to declared axis values.
type MatrixAxes = BTreeMap<String, Vec<String>>;

/// `extract_workflow_analysis` function.
pub(crate) fn extract_workflow_analysis(parsed: &serde_yaml::Value) -> WorkflowAnalysis {
    let mut analysis = WorkflowAnalysis::default();
    if let Some(env) = yaml_mapping_value(parsed, "env").and_then(serde_yaml::Value::as_mapping) {
        for (key, value) in env {
            if let Some(key) = key.as_str() {
                analysis.env_keys.push(key.to_owned());
                let _ = analysis
                    .env_bindings
                    .insert(key.to_owned(), scalar_as_string(value).unwrap_or_default());
            }
        }
    }
    if let Some(jobs) = yaml_mapping_value(parsed, "jobs").and_then(serde_yaml::Value::as_mapping) {
        for (job_key, job) in jobs {
            let job_id = job_key.as_str().unwrap_or_default();
            let job_facts = collect_job(job_id, job, &mut analysis);
            analysis.jobs.push(job_facts);
        }
    }
    analysis
}

/// `collect_job` function.
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
        env_keys: Vec::new(),
        env_bindings: BTreeMap::new(),
        steps: Vec::new(),
    };
    if let Some(env) = yaml_mapping_value(job, "env").and_then(serde_yaml::Value::as_mapping) {
        for (key, value) in env {
            if let Some(key) = key.as_str() {
                job_facts.env_keys.push(key.to_owned());
                let _ = job_facts
                    .env_bindings
                    .insert(key.to_owned(), scalar_as_string(value).unwrap_or_default());
            }
        }
    }
    if let Some(steps) = yaml_mapping_value(job, "steps").and_then(serde_yaml::Value::as_sequence) {
        for step in steps {
            let step_facts = collect_step(step);
            analysis.steps.push(step_facts.clone());
            job_facts.steps.push(step_facts);
        }
    }
    job_facts
}

/// `collect_step` function.
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
            let Some(key) = key.as_str() else {
                continue;
            };
            let Some(value) = scalar_as_string(value) else {
                continue;
            };
            let _ = facts.with_bindings.insert(key.to_owned(), value);
        }
    }
    facts
}

/// `yaml_mapping_value` function.
fn yaml_mapping_value<'a>(
    value: &'a serde_yaml::Value,
    key: &str,
) -> Option<&'a serde_yaml::Value> {
    value
        .as_mapping()
        .and_then(|mapping| mapping.get(serde_yaml::Value::String(key.to_owned())))
}

/// `collect_scalar_strings` function.
fn collect_scalar_strings(value: &serde_yaml::Value, output: &mut Vec<String>) {
    match value {
        serde_yaml::Value::String(text) => output.push(text.clone()),
        serde_yaml::Value::Sequence(values) => {
            for nested in values {
                collect_scalar_strings(nested, output);
            }
        }
        serde_yaml::Value::Null
        | serde_yaml::Value::Bool(_)
        | serde_yaml::Value::Number(_)
        | serde_yaml::Value::Mapping(_)
        | serde_yaml::Value::Tagged(_) => {}
    }
}

/// `collect_yaml_strings` function.
fn collect_yaml_strings(value: &serde_yaml::Value) -> Vec<String> {
    match value {
        serde_yaml::Value::String(value) => vec![value.clone()],
        serde_yaml::Value::Sequence(values) => {
            values.iter().flat_map(collect_yaml_strings).collect()
        }
        serde_yaml::Value::Null
        | serde_yaml::Value::Bool(_)
        | serde_yaml::Value::Number(_)
        | serde_yaml::Value::Mapping(_)
        | serde_yaml::Value::Tagged(_) => Vec::new(),
    }
}

/// `matrix_axes_from_job` function.
fn matrix_axes_from_job(job: &serde_yaml::Value) -> MatrixAxes {
    let mut collected = MatrixAxes::new();
    let Some(matrix) = yaml_mapping_value(job, "strategy")
        .and_then(|strategy| yaml_mapping_value(strategy, "matrix"))
        .and_then(serde_yaml::Value::as_mapping)
    else {
        return collected;
    };
    for (key, value) in matrix {
        let Some(axis_name) = key.as_str() else {
            continue;
        };
        if axis_name == "include" {
            collect_matrix_include_axes(value, &mut collected);
            continue;
        }
        let values = collect_yaml_strings(value);
        if !values.is_empty() {
            let _ = collected.insert(axis_name.to_owned(), values);
        }
    }
    collected
}

/// Collect axis values from `matrix.include` entries.
fn collect_matrix_include_axes(value: &serde_yaml::Value, collected: &mut MatrixAxes) {
    let Some(include_seq) = value.as_sequence() else {
        return;
    };
    for include_entry in include_seq {
        let Some(entry_map) = include_entry.as_mapping() else {
            continue;
        };
        for (entry_key, entry_value) in entry_map {
            let Some(entry_axis) = entry_key.as_str() else {
                continue;
            };
            let entry_values = collect_yaml_strings(entry_value);
            if !entry_values.is_empty() {
                collected
                    .entry(entry_axis.to_owned())
                    .or_default()
                    .extend(entry_values);
            }
        }
    }
}

/// `normalize_run_lines` function.
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

/// `scalar_as_string` function.
fn scalar_as_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(value) => Some(value.clone()),
        serde_yaml::Value::Bool(value) => Some(value.to_string()),
        serde_yaml::Value::Number(value) => Some(value.to_string()),
        serde_yaml::Value::Null
        | serde_yaml::Value::Sequence(_)
        | serde_yaml::Value::Mapping(_)
        | serde_yaml::Value::Tagged(_) => None,
    }
}
