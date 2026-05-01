use g3ts_spelling_types::{G3TsSpellingContractInput, G3TsSpellingPackageSurfaceState};
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(contract: &G3TsSpellingContractInput) -> Option<G3CheckResult> {
    if matches!(
        contract.package,
        G3TsSpellingPackageSurfaceState::Missing { .. }
    ) {
        return Some(crate::common::error(
            "g3ts-spelling/policy-configured",
            "Spelling policy root is missing",
            "`package.json` must exist so G3TS can evaluate spelling policy for this app/package root.".to_owned(),
            crate::common::package_rel_path(&contract.package),
        ));
    }
    None
}

#[cfg(test)]
#[path = "policy_configured_tests/mod.rs"]
mod policy_configured_tests;
