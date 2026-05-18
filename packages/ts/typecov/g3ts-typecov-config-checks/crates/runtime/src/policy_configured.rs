use g3ts_typecov_types::{
    G3TsTypecovContractInput, G3TsTypecovPackageSurfaceState, G3TsTypecovPolicySurfaceState,
};
use guardrail3_check_types::G3CheckResult;

/// Runs the typecov `policy-configured` config check.
pub(crate) fn check(contract: &G3TsTypecovContractInput) -> Option<G3CheckResult> {
    if matches!(
        contract.package,
        G3TsTypecovPackageSurfaceState::Missing { .. }
    ) {
        return Some(crate::common::error(
            "g3ts-typecov/policy-configured",
            "Typecov policy root is missing",
            "`package.json` must exist so G3TS can evaluate typecov policy for this app/package root.".to_owned(),
            Some(crate::common::package_rel_path(&contract.package)),
        ));
    }
    match &contract.typecov_policy {
        G3TsTypecovPolicySurfaceState::Parsed { .. } => None,
        G3TsTypecovPolicySurfaceState::Missing { .. } => Some(crate::common::error(
            "g3ts-typecov/policy-configured",
            "Typecov policy config is missing",
            format!(
                "`{}` must exist and define `[typecov] minimum = <integer>`.",
                crate::common::policy_rel_path(&contract.typecov_policy)
            ),
            Some(crate::common::policy_rel_path(&contract.typecov_policy)),
        )),
        G3TsTypecovPolicySurfaceState::Unreadable { reason, .. } => Some(crate::common::error(
            "g3ts-typecov/policy-configured",
            "Typecov policy config is unreadable",
            format!(
                "`{}` must be readable so G3TS can evaluate `[typecov].minimum`: {reason}.",
                crate::common::policy_rel_path(&contract.typecov_policy)
            ),
            Some(crate::common::policy_rel_path(&contract.typecov_policy)),
        )),
        G3TsTypecovPolicySurfaceState::ParseError { reason, .. } => Some(crate::common::error(
            "g3ts-typecov/policy-configured",
            "Typecov policy config is invalid",
            format!(
                "`{}` must define `[typecov] minimum` as an integer in 0..=100: {reason}.",
                crate::common::policy_rel_path(&contract.typecov_policy)
            ),
            Some(crate::common::policy_rel_path(&contract.typecov_policy)),
        )),
        G3TsTypecovPolicySurfaceState::MissingTypecovPolicy { .. } => Some(crate::common::error(
            "g3ts-typecov/policy-configured",
            "Typecov policy is missing",
            format!(
                "`{}` must define `[typecov] minimum = <integer>`.",
                crate::common::policy_rel_path(&contract.typecov_policy)
            ),
            Some(crate::common::policy_rel_path(&contract.typecov_policy)),
        )),
    }
}
