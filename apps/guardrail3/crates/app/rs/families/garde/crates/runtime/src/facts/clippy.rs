use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsGardeRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::policy::policy_settings_for;
use super::{ClippyConfigCandidate, GardeRootFacts, PolicyRootKind, PolicySettings};

pub(super) fn collect_clippy_configs(
    tree: &ProjectTree,
    route: &RsGardeRoute,
    workspace_roots: &BTreeSet<String>,
    input_failures: &mut Vec<super::GardeInputFailureFacts>,
) -> Vec<ClippyConfigCandidate> {
    let mut allowed_policy_roots = BTreeSet::new();
    allowed_policy_roots.extend(workspace_roots.iter().cloned());

    let candidates = route
        .family_files()
        .iter()
        .filter_map(|file| match file.kind() {
            RustFamilyFileKind::ClippyToml | RustFamilyFileKind::ClippyDotToml => Some((
                file.logical_owner_rel().to_owned(),
                file.rel_path().to_owned(),
            )),
            _ => None,
        })
        .filter(|(rel_dir, _)| allowed_policy_roots.contains(rel_dir))
        .map(|(rel_dir, rel_path)| {
            parse_clippy_candidate(tree, &rel_dir, &rel_path, input_failures)
        })
        .collect::<Vec<_>>();

    let mut by_dir = BTreeMap::<String, Vec<ClippyConfigCandidate>>::new();
    for candidate in candidates {
        by_dir
            .entry(candidate.rel_dir.clone())
            .or_default()
            .push(candidate);
    }

    let mut deduped = Vec::new();
    for (_rel_dir, mut same_root) in by_dir {
        same_root.sort_by_key(|candidate| config_precedence(&candidate.rel_path));
        if let Some(preferred) = same_root.into_iter().next() {
            deduped.push(preferred);
        }
    }
    deduped
}

fn parse_clippy_candidate(
    tree: &ProjectTree,
    rel_dir: &str,
    rel_path: &str,
    input_failures: &mut Vec<super::GardeInputFailureFacts>,
) -> ClippyConfigCandidate {
    let (parsed, parse_error) = match tree.file_content(&rel_path) {
        Some(content) => match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => (Some(parsed), None),
            Err(parse_error) => {
                let message = format!(
                    "Failed to parse `{rel_path}` for garde clippy-ban validation: {parse_error}"
                );
                input_failures.push(super::GardeInputFailureFacts {
                    rel_path: rel_path.to_owned(),
                    message: message.clone(),
                });
                (None, Some(message))
            }
        },
        None => (None, None),
    };

    ClippyConfigCandidate {
        rel_dir: rel_dir.to_owned(),
        rel_path: rel_path.to_owned(),
        parsed,
        parse_error,
    }
}

fn config_precedence(rel_path: &str) -> usize {
    if rel_path.ends_with("clippy.toml") && !rel_path.ends_with(".clippy.toml") {
        return 0;
    }
    if rel_path.ends_with(".clippy.toml") {
        return 1;
    }
    2
}

pub(super) fn push_root_facts(
    tree: &ProjectTree,
    rel_dir: &str,
    kind: PolicyRootKind,
    policy_map: &BTreeMap<String, PolicySettings>,
    clippy_configs: &[ClippyConfigCandidate],
    out: &mut Vec<GardeRootFacts>,
) {
    let settings = policy_settings_for(rel_dir, policy_map);
    if !settings.garde_enabled {
        return;
    }
    let cargo_rel_path = if rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        ProjectTree::join_rel(rel_dir, "Cargo.toml")
    };
    let cargo_parsed = tree
        .file_content(&cargo_rel_path)
        .and_then(|content| toml::from_str::<toml::Value>(content).ok());
    let garde_dependency_present = cargo_parsed
        .as_ref()
        .is_some_and(content_has_garde_dependency);

    let covering_config = nearest_covering_clippy(rel_dir, clippy_configs);
    out.push(GardeRootFacts {
        rel_dir: rel_dir.to_owned(),
        cargo_rel_path,
        kind,
        garde_dependency_present,
        garde_applicable: garde_dependency_present,
        clippy_rel_path: covering_config.map(|config| config.rel_path.clone()),
        clippy_parsed: covering_config.and_then(|config| config.parsed.clone()),
        clippy_parse_error: covering_config.and_then(|config| config.parse_error.clone()),
    });
}

fn nearest_covering_clippy<'a>(
    rel_dir: &str,
    configs: &'a [ClippyConfigCandidate],
) -> Option<&'a ClippyConfigCandidate> {
    configs
        .iter()
        .filter(|config| {
            config.rel_dir.is_empty()
                || rel_dir == config.rel_dir
                || rel_dir
                    .strip_prefix(&config.rel_dir)
                    .is_some_and(|rest| rest.starts_with('/'))
        })
        .max_by_key(|config| config.rel_dir.len())
}

pub(super) fn owning_root_dir<'a>(rel_path: &str, root_dirs: &'a [String]) -> Option<&'a str> {
    let parent = file_parent_rel(rel_path);
    root_dirs
        .iter()
        .filter(|root| {
            root.is_empty()
                || parent == root.as_str()
                || parent
                    .strip_prefix(root.as_str())
                    .is_some_and(|rest| rest.starts_with('/'))
        })
        .max_by_key(|root| root.len())
        .map(String::as_str)
}

fn file_parent_rel(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

fn content_has_garde_dependency(parsed: &toml::Value) -> bool {
    parsed
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(toml::Value::as_table)
        .is_some_and(|deps| deps.contains_key("garde"))
        || parsed
            .get("dependencies")
            .and_then(toml::Value::as_table)
            .is_some_and(|deps| deps.contains_key("garde"))
}
