use guardrail3_check_types::G3CheckResult;

use crate::SupportedFamily;

/// Shared result payload returned by one family runner.
pub type FamilyResults = Vec<G3CheckResult>;

/// Findings produced for one family during one validation run.
#[derive(Debug, Clone)]
pub struct FamilyRun {
    /// Family that produced these findings.
    pub family: SupportedFamily,
    /// Findings returned by that family.
    pub results: FamilyResults,
}

/// Full validation report across all selected families.
#[derive(Debug, Clone, Default)]
pub struct ValidateReport {
    /// Scope name shown in CLI output.
    pub scope: Option<&'static str>,
    /// Root path shown in CLI output.
    pub root: Option<std::path::PathBuf>,
    /// Per-family findings in run order.
    pub runs: Vec<FamilyRun>,
}

impl ValidateReport {
    /// Creates a validation report for one command scope.
    #[must_use]
    pub const fn scoped(scope: &'static str, root: std::path::PathBuf) -> Self {
        Self {
            scope: Some(scope),
            root: Some(root),
            runs: Vec::new(),
        }
    }
}
