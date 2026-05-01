use g3ts_style_types::G3TsStyleContractInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use std::collections::BTreeSet;

const REQUIRED_SYNCPACK_PINS: [(&str, &str); 1] =
    [("g3ts-eslint-plugin-style-policy", "0.1.3")];
const PIN_DEPENDENCY_TYPES: [&str; 2] = ["prod", "dev"];

pub(crate) fn check_syncpack_style_policy_pin(
    contract: &G3TsStyleContractInput,
    results: &mut Vec<G3CheckResult>,
) {
    let rel_path = syncpack_rel_path(&contract.syncpack_config);
    let Some(snapshot) = parsed_syncpack(&contract.syncpack_config) else {
        results.push(error(
            "g3ts-style/syncpack-style-policy-pin",
            "Style Syncpack policy is missing",
            format!(
                "`{}` must pin `g3ts-eslint-plugin-style-policy` through Syncpack. G3TS does not parse dependency ranges from `package.json`; Syncpack owns the style package floor.",
                rel_path.unwrap_or(".syncpackrc")
            ),
            rel_path,
        ));
        return;
    };

    let source_covers_package_manifest = syncpack_source_covers_package(
        &snapshot.source,
        &snapshot.rel_path,
        &package_rel_path(contract),
    );
    let missing_required_pins = missing_required_pins(snapshot);

    if source_covers_package_manifest && missing_required_pins.is_empty() {
        results.push(info(
            "g3ts-style/syncpack-style-policy-pin",
            "Style package floor is pinned by Syncpack",
            format!(
                "`{}` covers `package.json` and pins `g3ts-eslint-plugin-style-policy` to the required floor.",
                snapshot.rel_path
            ),
            Some(&snapshot.rel_path),
        ));
    } else {
        let missing = missing_required_pins
            .iter()
            .map(|pin| format!("`{} = {}`", pin.dependency, pin.version))
            .collect::<Vec<_>>();
        results.push(error(
            "g3ts-style/syncpack-style-policy-pin",
            "Style package floor is not pinned by Syncpack",
            format!(
                "`{}` must have `source: [\"package.json\"]` and a canonical Syncpack pin group for {}. G3TS does not parse dependency ranges from `package.json`; Syncpack owns the style package floor.",
                snapshot.rel_path,
                if missing.is_empty() {
                    "`g3ts-eslint-plugin-style-policy`".to_owned()
                } else {
                    missing.join(", ")
                }
            ),
            Some(&snapshot.rel_path),
        ));
    }
}

fn missing_required_pins(
    snapshot: &g3ts_style_types::G3TsStyleSyncpackSnapshot,
) -> Vec<g3ts_style_types::G3TsStyleSyncpackRequiredPin> {
    REQUIRED_SYNCPACK_PINS
        .iter()
        .filter(|(dependency, version)| {
            !has_one_canonical_pin_group(
                &snapshot.version_groups,
                dependency,
                version,
                &PIN_DEPENDENCY_TYPES,
            )
        })
        .map(|(dependency, version)| g3ts_style_types::G3TsStyleSyncpackRequiredPin {
            dependency: (*dependency).to_owned(),
            version: (*version).to_owned(),
        })
        .collect()
}

fn package_rel_path(contract: &G3TsStyleContractInput) -> String {
    match &contract.package {
        g3ts_style_types::G3TsStylePackageSurfaceState::Missing { rel_path }
        | g3ts_style_types::G3TsStylePackageSurfaceState::Unreadable { rel_path, .. }
        | g3ts_style_types::G3TsStylePackageSurfaceState::ParseError { rel_path, .. } => {
            rel_path.clone()
        }
        g3ts_style_types::G3TsStylePackageSurfaceState::Parsed { snapshot } => {
            snapshot.rel_path.clone()
        }
    }
}

fn syncpack_source_covers_package(
    source: &[String],
    syncpack_rel_path: &str,
    package_rel_path: &str,
) -> bool {
    source.len() == 1
        && source.first().is_some_and(|entry| entry == "package.json")
        && syncpack_config_is_app_local(syncpack_rel_path, package_rel_path)
}

fn has_one_canonical_pin_group(
    version_groups: &[g3ts_style_types::G3TsStyleSyncpackVersionGroupSnapshot],
    dependency: &str,
    version: &str,
    dependency_types: &[&str],
) -> bool {
    let mut matching_groups = version_groups
        .iter()
        .filter(|group| group_targets_dependency(group, dependency));

    let Some(group) = matching_groups.next() else {
        return false;
    };

    matching_groups.next().is_none() && canonical_pin_group(group, version, dependency_types)
}

fn group_targets_dependency(
    group: &g3ts_style_types::G3TsStyleSyncpackVersionGroupSnapshot,
    dependency: &str,
) -> bool {
    string_sets_match_exactly(&group.dependencies, &[dependency])
}

fn canonical_pin_group(
    group: &g3ts_style_types::G3TsStyleSyncpackVersionGroupSnapshot,
    version: &str,
    dependency_types: &[&str],
) -> bool {
    group.packages.is_none()
        && group.specifier_types.is_none()
        && string_sets_match_exactly(&group.dependency_types, dependency_types)
        && group.is_ignored.is_none()
        && group.is_banned.is_none()
        && group.pin_version.as_deref() == Some(version)
}

fn string_sets_match_exactly(left: &[String], right: &[&str]) -> bool {
    left.len() == right.len()
        && BTreeSet::from_iter(left.iter().map(String::as_str))
            == BTreeSet::from_iter(right.iter().copied())
}

fn syncpack_config_is_app_local(syncpack_rel_path: &str, package_rel_path: &str) -> bool {
    let expected_rel_path = package_rel_path.strip_suffix("/package.json").map_or_else(
        || ".syncpackrc".to_owned(),
        |app_root| format!("{app_root}/.syncpackrc"),
    );

    syncpack_rel_path == expected_rel_path
}

fn parsed_syncpack(
    config: &g3ts_style_types::G3TsStyleSyncpackSurfaceState,
) -> Option<&g3ts_style_types::G3TsStyleSyncpackSnapshot> {
    match config {
        g3ts_style_types::G3TsStyleSyncpackSurfaceState::Parsed { snapshot } => Some(snapshot),
        g3ts_style_types::G3TsStyleSyncpackSurfaceState::Missing { .. }
        | g3ts_style_types::G3TsStyleSyncpackSurfaceState::Unreadable { .. }
        | g3ts_style_types::G3TsStyleSyncpackSurfaceState::ParseError { .. } => None,
    }
}

fn syncpack_rel_path(config: &g3ts_style_types::G3TsStyleSyncpackSurfaceState) -> Option<&str> {
    match config {
        g3ts_style_types::G3TsStyleSyncpackSurfaceState::Missing { rel_path }
        | g3ts_style_types::G3TsStyleSyncpackSurfaceState::Unreadable { rel_path, .. }
        | g3ts_style_types::G3TsStyleSyncpackSurfaceState::ParseError { rel_path, .. } => {
            Some(rel_path)
        }
        g3ts_style_types::G3TsStyleSyncpackSurfaceState::Parsed { snapshot } => {
            Some(&snapshot.rel_path)
        }
    }
}

fn info(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
    .into_inventory()
}

fn error(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
}
