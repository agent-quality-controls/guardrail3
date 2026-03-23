use devctl_domain_types::{DoctorReport, HookMode, HookStatus, WorkspaceConfig};
use devctl_ports_outbound_traits::WorkspaceProbe;

pub fn bootstrap_workspace(probe: &impl WorkspaceProbe, workspace_root: &str) -> DoctorReport {
    match probe.read_config(workspace_root) {
        Some(config) => probe.run_doctor(&config.root),
        None => DoctorReport {
            workspace_root: workspace_root.to_owned(),
            missing_paths: vec!["guardrail3.toml".to_owned(), ".githooks/pre-commit".to_owned()],
            stale_generated_files: Vec::new(),
            hook_status: HookStatus::Missing,
        },
    }
}

pub fn recommended_fixups(report: &DoctorReport) -> Vec<String> {
    let mut fixes = Vec::new();
    if !report.missing_paths.is_empty() {
        fixes.push("restore workspace guardrail config".to_owned());
    }
    if !report.stale_generated_files.is_empty() {
        fixes.push("regenerate derived artifacts".to_owned());
    }
    if matches!(report.hook_status, HookStatus::Missing | HookStatus::Stale) {
        fixes.push("reinstall git hooks".to_owned());
    }
    fixes
}

pub fn default_config(workspace_root: &str) -> WorkspaceConfig {
    WorkspaceConfig::new(workspace_root, HookMode::ValidateAndFix).expect("fixture config")
}

#[cfg(test)]
mod tests {
    use devctl_domain_types::{DoctorReport, HookStatus, WorkspaceConfig};
    use devctl_ports_outbound_traits::WorkspaceProbe;

    use super::{bootstrap_workspace, recommended_fixups};

    struct FakeProbe;

    impl WorkspaceProbe for FakeProbe {
        fn read_config(&self, workspace_root: &str) -> Option<WorkspaceConfig> {
            WorkspaceConfig::new(workspace_root, devctl_domain_types::HookMode::ValidateOnly).ok()
        }

        fn run_doctor(&self, workspace_root: &str) -> DoctorReport {
            DoctorReport {
                workspace_root: workspace_root.to_owned(),
                missing_paths: vec!["guardrail3.toml".to_owned()],
                stale_generated_files: vec!["target/generated-hook.txt".to_owned()],
                hook_status: HookStatus::Stale,
            }
        }
    }

    #[test]
    fn surfaces_fixups_for_stale_workspace() {
        let report = bootstrap_workspace(&FakeProbe, "/repo");
        let fixes = recommended_fixups(&report);

        assert_eq!(fixes.len(), 3);
    }
}
