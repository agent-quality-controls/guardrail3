use devctl_domain_types::{DoctorReport, WorkspaceConfig};

pub trait WorkspaceProbe {
    fn read_config(&self, workspace_root: &str) -> Option<WorkspaceConfig>;
    fn run_doctor(&self, workspace_root: &str) -> DoctorReport;
}
