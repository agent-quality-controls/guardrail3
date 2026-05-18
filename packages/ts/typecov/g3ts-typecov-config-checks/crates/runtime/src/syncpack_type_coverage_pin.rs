use g3ts_typecov_types::G3TsTypecovContractInput;
use guardrail3_check_types::G3CheckResult;
use syncpack_config_parser::types::SyncpackDependencyDeclarationRef;

/// Runs the typecov `syncpack-type-coverage-pin` config check.
pub(crate) fn check(contract: &G3TsTypecovContractInput) -> G3CheckResult {
    let rel_path = crate::common::syncpack_rel_path(&contract.syncpack_config);
    let Some(snapshot) = crate::common::parsed_syncpack(&contract.syncpack_config) else {
        return crate::common::error(
            "g3ts-typecov/syncpack-type-coverage-pin",
            "type-coverage Syncpack pin cannot be checked",
            format!(
                "`{rel_path}` must be readable and parseable so G3TS can prove `type-coverage` is pinned by Syncpack."
            ),
            Some(rel_path),
        );
    };
    let Some(package) = crate::common::parsed_package(&contract.package) else {
        return crate::common::error(
            "g3ts-typecov/syncpack-type-coverage-pin",
            "type-coverage Syncpack pin cannot be checked",
            format!(
                "`{}` must be readable and parseable so G3TS can prove `type-coverage` is pinned by Syncpack.",
                crate::common::package_rel_path(&contract.package)
            ),
            Some(crate::common::package_rel_path(&contract.package)),
        );
    };
    let declarations = crate::common::package_dependency_declarations(package, "type-coverage")
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
        "type-coverage",
    ) {
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
