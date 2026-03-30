use std::collections::BTreeMap;

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
