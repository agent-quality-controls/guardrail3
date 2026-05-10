use g3ts_spelling_types::{G3TsSpellingConfigSurfaceState, G3TsSpellingContractInput};
use guardrail3_check_types::G3CheckResult;

/// `check`: check.
pub(crate) fn check(contract: &G3TsSpellingContractInput) -> G3CheckResult {
    match &contract.cspell_config {
        G3TsSpellingConfigSurfaceState::Parsed { rel_path } => crate::common::info(
            "g3ts-spelling/cspell-config-present",
            "cspell config is present",
            format!("`{rel_path}` defines the local spelling policy."),
            Some(rel_path),
        ),
        G3TsSpellingConfigSurfaceState::Missing { rel_path } => crate::common::error(
            "g3ts-spelling/cspell-config-present",
            "cspell config is missing",
            format!(
                "`{rel_path}` must exist. Add a local cspell config instead of relying on implicit spelling defaults."
            ),
            Some(rel_path),
        ),
        G3TsSpellingConfigSurfaceState::Unreadable { rel_path, reason }
        | G3TsSpellingConfigSurfaceState::ParseError { rel_path, reason } => crate::common::error(
            "g3ts-spelling/cspell-config-present",
            "cspell config is not usable",
            format!("`{rel_path}` must be readable and delegated to cspell. Reason: {reason}."),
            Some(rel_path),
        ),
    }
}

#[cfg(test)]
#[path = "cspell_config_present_tests/mod.rs"]
mod cspell_config_present_tests;
