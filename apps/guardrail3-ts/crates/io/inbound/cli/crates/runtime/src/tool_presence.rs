//! Required-tool presence checks: ensure the host has the binaries that the
//! validate repo command relies on (e.g. `pnpm`, `gitleaks`).

use crate::process::run_env_which;

/// Returns one finding line per required tool that is missing from PATH.
/// Used by `validate repo` to surface host-setup failures before the
/// repo-level checks run.
pub(crate) fn check_required_tools_present() -> Vec<String> {
    let mut findings = Vec::new();
    for (label, command) in [("pnpm", "pnpm"), ("gitleaks", "gitleaks")] {
        if which_present(command).is_none() {
            findings.push(format!(
                "g3ts-tools/{label}-not-installed - required tool `{command}` not found on PATH"
            ));
        }
    }
    findings
}

/// Returns `Some(())` when `/usr/bin/env which <command>` succeeds. Used as
/// a portable PATH-lookup probe.
fn which_present(command: &str) -> Option<()> {
    let output = run_env_which(command).ok()?;
    output.status.success().then_some(())
}
