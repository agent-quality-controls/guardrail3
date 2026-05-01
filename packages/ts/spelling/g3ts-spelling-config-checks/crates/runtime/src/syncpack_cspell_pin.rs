use g3ts_spelling_types::G3TsSpellingContractInput;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(contract: &G3TsSpellingContractInput) -> G3CheckResult {
    let rel_path = crate::common::syncpack_rel_path(&contract.syncpack_config);
    let Some(snapshot) = crate::common::parsed_syncpack(&contract.syncpack_config) else {
        return crate::common::error(
            "g3ts-spelling/syncpack-cspell-pin",
            "cspell Syncpack pin cannot be checked",
            format!(
                "`{}` must be readable and parseable so G3TS can prove `cspell` is pinned by Syncpack.",
                rel_path.unwrap_or(".syncpackrc")
            ),
            rel_path,
        );
    };
    if snapshot.version_groups.iter().any(|group| {
        group.is_ignored != Some(true)
            && group.is_banned != Some(true)
            && group
                .dependencies
                .iter()
                .any(|dependency| dependency == "cspell")
            && group.pin_version.is_some()
    }) {
        crate::common::info(
            "g3ts-spelling/syncpack-cspell-pin",
            "cspell is pinned by Syncpack",
            format!(
                "`{}` pins `cspell` in a non-ignored version group.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-spelling/syncpack-cspell-pin",
            "cspell is not pinned by Syncpack",
            format!(
                "`{}` must pin `cspell` in a non-ignored Syncpack version group.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        )
    }
}

#[cfg(test)]
#[path = "syncpack_cspell_pin_tests/mod.rs"]
mod syncpack_cspell_pin_tests;
