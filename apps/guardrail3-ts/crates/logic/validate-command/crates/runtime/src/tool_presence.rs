//! Repo-level required-tool presence checks.

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::process::run_env_which;

/// Returns one finding per required host tool that is missing.
#[must_use]
pub(crate) fn check_required_tools_present() -> Vec<G3CheckResult> {
    let mut findings = Vec::new();
    for (label, command) in [("pnpm", "pnpm"), ("gitleaks", "gitleaks")] {
        if which_present(command).is_none() {
            findings.push(G3CheckResult::new(
                format!("g3ts-tools/{label}-not-installed"),
                G3Severity::Error,
                format!("required tool `{command}` not installed"),
                format!("`{command}` was not found on PATH."),
                Some(command.to_owned()),
                None,
            ));
        }
    }
    findings
}

/// Returns Some when a command is available on PATH.
fn which_present(command: &str) -> Option<()> {
    let output = run_env_which(command).ok()?;
    output.status.success().then_some(())
}
