use g3ts_spelling_types::G3TsSpellingContractInput;
use guardrail3_check_types::G3CheckResult;
use syncpack_config_parser::types::SyncpackDependencyDeclarationRef;

/// `check`: check.
pub(crate) fn check(contract: &G3TsSpellingContractInput) -> G3CheckResult {
    let rel_path = crate::common::syncpack_rel_path(&contract.syncpack_config);
    let Some(snapshot) = crate::common::parsed_syncpack(&contract.syncpack_config) else {
        return crate::common::error(
            "g3ts-spelling/syncpack-cspell-pin",
            "cspell Syncpack pin cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `cspell` is pinned by Syncpack."
            ),
            Some(rel_path),
        );
    };
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-spelling/syncpack-cspell-pin",
            "cspell Syncpack pin cannot be checked",
            format!(
                "`{}` must be readable and parseable so G3TS can prove `cspell` is pinned by Syncpack.",
                crate::common::package_rel_path(&contract.package)
            ),
            Some(crate::common::package_rel_path(&contract.package)),
        );
    };
    let declarations = crate::common::package_dependency_declarations(package, "cspell")
        .into_iter()
        .map(|declaration| SyncpackDependencyDeclarationRef {
            name: &declaration.name,
            lane: &declaration.lane,
            specifier_type: &declaration.specifier_type,
        })
        .collect::<Vec<_>>();
    if syncpack_config_parser::first_matching_group_pins_dependency(
        &snapshot.version_groups,
        package.name.as_deref(),
        &declarations,
        "cspell",
    ) {
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
