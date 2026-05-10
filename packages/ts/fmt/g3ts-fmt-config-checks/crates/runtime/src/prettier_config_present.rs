use g3ts_fmt_types::{G3TsFmtConfigSurfaceState, G3TsFmtContractInput};
use guardrail3_check_types::G3CheckResult;

/// Runs the corresponding fmt config check.
pub(crate) fn check(contract: &G3TsFmtContractInput) -> G3CheckResult {
    match &contract.prettier_config {
        G3TsFmtConfigSurfaceState::Parsed { rel_path } => crate::common::info(
            "g3ts-fmt/prettier-config-present",
            "Prettier config is present",
            format!("`{rel_path}` defines the local formatter policy."),
            Some(rel_path),
        ),
        G3TsFmtConfigSurfaceState::Missing { rel_path } => crate::common::error(
            "g3ts-fmt/prettier-config-present",
            "Prettier config is missing",
            format!(
                "`{rel_path}` must exist. Add a local Prettier config instead of relying on implicit formatter defaults."
            ),
            Some(rel_path),
        ),
        G3TsFmtConfigSurfaceState::Unreadable { rel_path, reason }
        | G3TsFmtConfigSurfaceState::ParseError { rel_path, reason } => crate::common::error(
            "g3ts-fmt/prettier-config-present",
            "Prettier config is not usable",
            format!("`{rel_path}` must be readable and delegated to Prettier. Reason: {reason}."),
            Some(rel_path),
        ),
    }
}
