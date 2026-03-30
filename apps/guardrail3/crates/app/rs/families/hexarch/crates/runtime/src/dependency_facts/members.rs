use std::collections::{BTreeMap, BTreeSet};

use guardrail3_domain_project_tree::ProjectTree;

use super::{
    Layer, MemberDependencyFacts, MemberManifestFailureFacts, ParsedGuardrailConfig,
    WorkspaceFacts, app_root_for_dir, layer_from_config, layer_from_path,
};

pub(super) fn collect_members(
    tree: &ProjectTree,
    workspaces: &[WorkspaceFacts],
    workspace_for_member: &BTreeMap<String, String>,
    owned_app_roots: &BTreeSet<String>,
    guardrail: Option<&ParsedGuardrailConfig>,
) -> (Vec<MemberDependencyFacts>, Vec<MemberManifestFailureFacts>) {
    let mut member_dirs = BTreeSet::new();
    for workspace in workspaces {
        member_dirs.extend(
            workspace
                .member_dirs
                .iter()
                .filter(|rel_dir| dir_is_within_owned_app(rel_dir, owned_app_roots))
                .cloned(),
        );
    }

    let mut members = Vec::new();
    let mut failures = Vec::new();
    for rel_dir in member_dirs {
        let cargo_rel_path = format!("{rel_dir}/Cargo.toml");
        let cargo_content = tree.file_content(&cargo_rel_path);
        let fallback_name = rel_dir.rsplit('/').next().unwrap_or(&rel_dir).to_owned();
        let (name, cargo_parse_error) = match cargo_content {
            Some(content) => match toml::from_str::<toml::Value>(content) {
                Ok(parsed) => (
                    parsed
                        .get("package")
                        .and_then(|value| value.get("name"))
                        .and_then(toml::Value::as_str)
                        .map(str::to_owned)
                        .unwrap_or_else(|| fallback_name.clone()),
                    None,
                ),
                Err(parse_error) => (fallback_name.clone(), Some(parse_error.to_string())),
            },
            None => (fallback_name.clone(), None),
        };

        let app_root_rel_dir = app_root_for_dir(&rel_dir);
        let (_profile_name, allowed_deps) =
            profile_and_allowed_deps_for_member(&rel_dir, guardrail);
        let layer = layer_for_member(&rel_dir, guardrail);

        if let Some(parse_error) = &cargo_parse_error {
            failures.push(MemberManifestFailureFacts {
                name: name.clone(),
                rel_dir: rel_dir.clone(),
                cargo_rel_path: cargo_rel_path.clone(),
                parse_error: parse_error.clone(),
            });
        }

        members.push(MemberDependencyFacts {
            name,
            rel_dir: rel_dir.clone(),
            cargo_rel_path,
            cargo_parse_error,
            workspace_root_rel_dir: workspace_for_member.get(&rel_dir).cloned(),
            app_root_rel_dir,
            layer,
            allowed_deps,
        });
    }

    members.sort_by(|left, right| left.rel_dir.cmp(&right.rel_dir));
    failures.sort_by(|left, right| left.rel_dir.cmp(&right.rel_dir));
    (members, failures)
}

pub(super) fn dir_is_within_owned_app(rel_dir: &str, owned_app_roots: &BTreeSet<String>) -> bool {
    owned_app_roots.iter().any(|app_root| {
        rel_dir == app_root
            || rel_dir
                .strip_prefix(app_root.as_str())
                .is_some_and(|rest| rest.starts_with('/'))
    })
}

pub(super) fn dir_is_within_owned_hex_scope(
    rel_dir: &str,
    owned_app_roots: &BTreeSet<String>,
) -> bool {
    owned_app_roots.iter().any(|app_root| {
        rel_dir == app_root
            || rel_dir
                .strip_prefix(&format!("{app_root}/crates"))
                .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
    })
}

fn profile_and_allowed_deps_for_member(
    rel_dir: &str,
    guardrail: Option<&ParsedGuardrailConfig>,
) -> (Option<String>, BTreeSet<String>) {
    let Some(guardrail) = guardrail else {
        return (None, BTreeSet::new());
    };

    if let Some(app_root) = app_root_for_dir(rel_dir) {
        let app_name = app_root.rsplit('/').next().unwrap_or(&app_root);
        if let Some(config) = guardrail.app_configs.get(app_name) {
            return (
                config
                    .profile()
                    .map(str::to_owned)
                    .or_else(|| config.type_().map(str::to_owned)),
                config
                    .allowed_deps()
                    .map(|deps| deps.iter().cloned().collect())
                    .unwrap_or_default(),
            );
        }
    }

    if rel_dir.starts_with("packages/") {
        if let Some(config) = &guardrail.packages_config {
            return (
                config
                    .profile()
                    .map(str::to_owned)
                    .or_else(|| config.type_().map(str::to_owned)),
                config
                    .allowed_deps()
                    .map(|deps| deps.iter().cloned().collect())
                    .unwrap_or_default(),
            );
        }
    }

    (guardrail.root_profile_name.clone(), BTreeSet::new())
}

fn layer_for_member(rel_dir: &str, guardrail: Option<&ParsedGuardrailConfig>) -> Option<Layer> {
    layer_from_path(rel_dir).or_else(|| {
        if rel_dir.starts_with("packages/") {
            guardrail
                .and_then(|guardrail| guardrail.packages_config.as_ref())
                .and_then(|config| config.layer())
                .and_then(layer_from_config)
        } else {
            None
        }
    })
}
