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
    /// Per-family findings in run order.
    pub runs: Vec<FamilyRun>,
}
