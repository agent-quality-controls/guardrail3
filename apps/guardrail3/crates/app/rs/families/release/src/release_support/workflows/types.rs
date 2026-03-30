use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct WorkflowStepFacts {
    pub(crate) uses: Option<String>,
    pub(crate) run_lines: Vec<String>,
    pub(crate) env_keys: Vec<String>,
    pub(crate) env_values: Vec<String>,
    pub(crate) with_values: Vec<String>,
    pub(crate) env_bindings: BTreeMap<String, String>,
    pub(crate) with_bindings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkflowJobFacts {
    pub(crate) id: String,
    pub(crate) runs_on: Vec<String>,
    pub(crate) needs: Vec<String>,
    pub(crate) matrix_axes: BTreeMap<String, Vec<String>>,
    pub(crate) steps: Vec<WorkflowStepFacts>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkflowAnalysis {
    pub(crate) jobs: Vec<WorkflowJobFacts>,
    pub(crate) steps: Vec<WorkflowStepFacts>,
    pub(crate) scalar_strings: Vec<String>,
    pub(crate) env_keys: Vec<String>,
}
