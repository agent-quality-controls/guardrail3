use g3ts_typecov_types::{G3TsTypecovContractInput, G3TsTypecovPackageSurfaceState};
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(contract: &G3TsTypecovContractInput) -> Option<G3CheckResult> {
    if matches!(
        contract.package,
        G3TsTypecovPackageSurfaceState::Missing { .. }
    ) {
        return Some(crate::common::error(
            "g3ts-typecov/policy-configured",
            "Typecov policy root is missing",
            "`package.json` must exist so G3TS can evaluate typecov policy for this app/package root.".to_owned(),
            crate::common::package_rel_path(&contract.package),
        ));
    }
    None
}

#[cfg(test)]
#[path = "policy_configured_tests/mod.rs"]
mod policy_configured_tests;
