use devctl_domain_types::{DoctorReport, HookMode, HookStatus, WorkspaceConfig};
use devctl_ports_outbound_traits::WorkspaceProbe;

pub struct FixtureWorkspaceProbe;

impl WorkspaceProbe for FixtureWorkspaceProbe {
    fn read_config(&self, workspace_root: &str) -> Option<WorkspaceConfig> {
        WorkspaceConfig::new(workspace_root, HookMode::ValidateAndFix).ok()
    }

    fn run_doctor(&self, workspace_root: &str) -> DoctorReport {
        DoctorReport {
            workspace_root: workspace_root.to_owned(),
            missing_paths: vec!["apps/backend/Cargo.toml".to_owned()],
            stale_generated_files: vec![".githooks/pre-push.generated".to_owned()],
            hook_status: HookStatus::Stale,
        }
    }
}
