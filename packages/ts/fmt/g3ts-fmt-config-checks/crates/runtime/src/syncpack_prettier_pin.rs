use g3ts_fmt_types::G3TsFmtContractInput;
use guardrail3_check_types::G3CheckResult;

/// Runs the corresponding fmt config check.
pub(crate) fn check(contract: &G3TsFmtContractInput) -> G3CheckResult {
    let rel_path = crate::common::syncpack_rel_path(&contract.syncpack_config);
    let Some(snapshot) = crate::common::parsed_syncpack(&contract.syncpack_config) else {
        return crate::common::error(
            "g3ts-fmt/syncpack-prettier-pin",
            "Prettier Syncpack pin cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `prettier` is pinned by Syncpack."
            ),
            Some(rel_path),
        );
    };
    if snapshot.version_groups.iter().any(|group| {
        group.is_ignored != Some(true)
            && group.is_banned != Some(true)
            && group
                .dependencies
                .iter()
                .any(|dependency| dependency == "prettier")
            && group.pin_version.is_some()
    }) {
        crate::common::info(
            "g3ts-fmt/syncpack-prettier-pin",
            "Prettier is pinned by Syncpack",
            format!(
                "`{}` pins `prettier` in a non-ignored version group.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        )
    } else {
        crate::common::error(
            "g3ts-fmt/syncpack-prettier-pin",
            "Prettier is not pinned by Syncpack",
            format!(
                "`{}` must pin `prettier` in a non-ignored Syncpack version group.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        )
    }
}
