mod analysis;
mod detection;
mod types;

pub use analysis::extract_workflow_analysis;
pub use detection::{
    binary_release_present, linux_target_present, publish_dry_run_step_present,
    registry_token_present, release_plz_step_present,
};
pub use types::WorkflowAnalysis;
