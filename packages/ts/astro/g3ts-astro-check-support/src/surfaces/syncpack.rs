use super::prelude::*;
use super::constants::*;
use super::roots::scoped_rel_path;

pub fn ingest_syncpack_config_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    package: &G3TsAstroPackageSurfaceState,
) -> G3TsAstroSyncpackConfigState {
    let Some(entry) = select_syncpack_config(crawl, app_root_rel_path) else {
        return G3TsAstroSyncpackConfigState::Missing {
            rel_path: missing_syncpack_config_rel_path(app_root_rel_path),
        };
    };

    if !entry.readable {
        return G3TsAstroSyncpackConfigState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Syncpack config unreadable".to_owned(),
        };
    }

    let document = match syncpack_from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsAstroSyncpackConfigState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = syncpack_parse_error_reason(&document) {
        return G3TsAstroSyncpackConfigState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let typed = syncpack_config_parser::typed(&document)
        .expect("parsed Syncpack config document should stay typed");
    let package_rel_path = package_rel_path_for_app(app_root_rel_path, package);
    let source_covers_package_manifest =
        syncpack_source_covers_package(&typed.source, &entry.path.rel_path, &package_rel_path);
    let missing_required_stack_pins = REQUIRED_SYNCPACK_PINS
        .iter()
        .filter(|(dependency, version)| {
            !has_one_canonical_pin_group(
                &typed.version_groups,
                dependency,
                version,
                &PIN_DEPENDENCY_TYPES,
            )
        })
        .map(|(dependency, version)| G3TsAstroSyncpackRequiredPin {
            dependency: (*dependency).to_owned(),
            version: (*version).to_owned(),
        })
        .collect();
    let missing_forbidden_bans = FORBIDDEN_SYNCPACK_DEPS
        .iter()
        .filter(|dependency| {
            !has_one_canonical_ban_group(&typed.version_groups, dependency, &BAN_DEPENDENCY_TYPES)
        })
        .map(|dependency| (*dependency).to_owned())
        .collect();
    G3TsAstroSyncpackConfigState::Parsed {
        snapshot: G3TsAstroSyncpackConfigSnapshot {
            rel_path: entry.path.rel_path.clone(),
            source_covers_package_manifest,
            missing_required_stack_pins,
            missing_forbidden_bans,
        },
    }
}

fn package_rel_path_for_app(
    app_root_rel_path: &str,
    package: &G3TsAstroPackageSurfaceState,
) -> String {
    let rel_path = match package {
        G3TsAstroPackageSurfaceState::Missing { rel_path }
        | G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPackageSurfaceState::ParseError { rel_path, .. } => rel_path.clone(),
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => snapshot.rel_path.clone(),
    };

    if rel_path.is_empty() {
        scoped_rel_path(app_root_rel_path, PACKAGE_JSON_REL_PATH)
    } else {
        rel_path
    }
}

pub fn required_syncpack_pins() -> Vec<G3TsAstroSyncpackRequiredPin> {
    REQUIRED_SYNCPACK_PINS
        .into_iter()
        .map(|(dependency, version)| G3TsAstroSyncpackRequiredPin {
            dependency: dependency.to_owned(),
            version: version.to_owned(),
        })
        .collect()
}

#[must_use]
pub fn forbidden_syncpack_deps() -> Vec<String> {
    FORBIDDEN_SYNCPACK_DEPS
        .into_iter()
        .map(str::to_owned)
        .collect()
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
    version_groups: &[syncpack_config_parser::types::SyncpackVersionGroup],
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

fn has_one_canonical_ban_group(
    version_groups: &[syncpack_config_parser::types::SyncpackVersionGroup],
    dependency: &str,
    dependency_types: &[&str],
) -> bool {
    let mut matching_groups = version_groups
        .iter()
        .filter(|group| group_targets_dependency(group, dependency));

    let Some(group) = matching_groups.next() else {
        return false;
    };

    matching_groups.next().is_none() && canonical_ban_group(group, dependency_types)
}

fn group_targets_dependency(
    group: &syncpack_config_parser::types::SyncpackVersionGroup,
    dependency: &str,
) -> bool {
    string_sets_match_exactly(&group.dependencies, &[dependency])
}

fn canonical_pin_group(
    group: &syncpack_config_parser::types::SyncpackVersionGroup,
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

fn canonical_ban_group(
    group: &syncpack_config_parser::types::SyncpackVersionGroup,
    dependency_types: &[&str],
) -> bool {
    group.packages.is_none()
        && group.specifier_types.is_none()
        && string_sets_match_exactly(&group.dependency_types, dependency_types)
        && group.is_ignored.is_none()
        && group.is_banned == Some(true)
        && group.pin_version.is_none()
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

fn select_syncpack_config<'crawl>(
    crawl: &'crawl G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'crawl g3_workspace_crawl::G3RsWorkspaceEntry> {
    let app_config = scoped_rel_path(app_root_rel_path, SYNCPACK_CONFIG_REL_PATH);
    exact_included_file(crawl, &app_config)
}

pub(super) fn exact_included_file<'crawl>(
    crawl: &'crawl G3WorkspaceCrawl,
    rel_path: &str,
) -> Option<&'crawl g3_workspace_crawl::G3RsWorkspaceEntry> {
    crawl.entries.iter().find(|entry| {
        entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
            && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
            && entry.path.rel_path == rel_path
    })
}

fn missing_syncpack_config_rel_path(app_root_rel_path: &str) -> String {
    if app_root_rel_path == "." {
        SYNCPACK_CONFIG_REL_PATH.to_owned()
    } else {
        format!("{app_root_rel_path}/{SYNCPACK_CONFIG_REL_PATH}")
    }
}
