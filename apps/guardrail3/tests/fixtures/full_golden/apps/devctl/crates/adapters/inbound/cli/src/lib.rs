use devctl_adapters_outbound_fs::FixtureWorkspaceProbe;
use devctl_app_core::{bootstrap_workspace, recommended_fixups};

pub fn render_doctor_summary(workspace_root: &str) -> String {
    let probe = FixtureWorkspaceProbe;
    let report = bootstrap_workspace(&probe, workspace_root);
    let fixes = recommended_fixups(&report);

    format!(
        "workspace={} hook_status={:?} fixes={}",
        report.workspace_root,
        report.hook_status,
        fixes.join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::render_doctor_summary;

    #[test]
    fn includes_workspace_root_and_fixups() {
        let summary = render_doctor_summary("/repo");
        assert!(summary.contains("/repo"));
        assert!(summary.contains("reinstall git hooks"));
    }
}
