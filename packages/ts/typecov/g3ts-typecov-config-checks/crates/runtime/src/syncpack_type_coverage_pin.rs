use g3ts_typecov_types::G3TsTypecovContractInput;
use guardrail3_check_types::G3CheckResult;

pub(crate) fn check(contract: &G3TsTypecovContractInput) -> G3CheckResult {
    let rel_path = crate::common::syncpack_rel_path(&contract.syncpack_config);
    let Some(snapshot) = crate::common::parsed_syncpack(&contract.syncpack_config) else {
        return crate::common::error(
            "g3ts-typecov/syncpack-type-coverage-pin",
            "type-coverage Syncpack pin cannot be checked",
            format!(
                "`{}` must be readable and parseable so G3TS can prove `type-coverage` is pinned by Syncpack.",
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
                .any(|dependency| dependency == "type-coverage")
            && group.pin_version.is_some()
    }) {
        crate::common::info(
            "g3ts-typecov/syncpack-type-coverage-pin",
            "type-coverage is pinned by Syncpack",
            format!(
                "`{}` pins `type-coverage` in a non-ignored version group.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-typecov/syncpack-type-coverage-pin",
            "type-coverage is not pinned by Syncpack",
            format!(
                "`{}` must pin `type-coverage` in a non-ignored Syncpack version group.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        )
    }
}

#[cfg(test)]
#[path = "syncpack_type_coverage_pin_tests/mod.rs"]
mod syncpack_type_coverage_pin_tests;
