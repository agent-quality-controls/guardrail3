use g3ts_fmt_types::{G3TsFmtContractInput, G3TsFmtPackageSurfaceState};
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(contract: &G3TsFmtContractInput) -> Option<G3CheckResult> {
    if matches!(contract.package, G3TsFmtPackageSurfaceState::Missing { .. }) {
        return Some(crate::common::error(
            "g3ts-fmt/policy-configured",
            "Formatter policy root is missing",
            "`package.json` must exist so G3TS can evaluate formatter policy for this app/package root.".to_owned(),
            crate::common::package_rel_path(&contract.package),
        ));
    }
    None
}

#[cfg(test)]
#[path = "policy_configured_tests/mod.rs"]
mod policy_configured_tests;
