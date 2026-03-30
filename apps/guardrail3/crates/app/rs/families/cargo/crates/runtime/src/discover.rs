use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsCargoRoute;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;

use super::facts::{
    CargoFamilyFacts, InputFailureFacts, MissingMemberCargoFacts, PolicyRootCargoFacts,
    PolicyRootKind, WorkspaceMemberCargoFacts,
};

#[derive(Debug, Clone)]
struct CargoSnapshot {
    rel_dir: String,
    cargo_rel_path: String,
    parsed: Option<toml::Value>,
    parse_error: Option<String>,
    has_workspace: bool,
    has_package: bool,
    edition: Option<String>,
    rust_version: Option<String>,
    resolver: Option<String>,
    declared_members: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct GuardrailSnapshot {
    profile_name: Option<String>,
    parse_error: Option<String>,
}

pub fn collect(tree: &ProjectTree, route: &RsCargoRoute) -> CargoFamilyFacts {
    let snapshots = collect_cargo_snapshots(tree, route);
    let workspace_roots: Vec<_> = snapshots
        .values()
        .filter(|snapshot| snapshot.has_workspace)
        .map(|snapshot| snapshot.rel_dir.clone())
        .collect();
    let workspace_members: BTreeSet<_> = snapshots
        .values()
        .filter(|snapshot| snapshot.has_workspace)
        .flat_map(|snapshot| snapshot.declared_members.iter().cloned())
        .collect();

    let mut policy_roots = Vec::new();
    let mut member_facts = Vec::new();
    let mut missing_members = Vec::new();
    let mut input_failures = Vec::new();

    for workspace_root_rel in &workspace_roots {
        let Some(workspace_snapshot) = snapshots.get(workspace_root_rel) else {
            continue;
        };
        push_policy_root(
            workspace_snapshot,
            PolicyRootKind::WorkspaceRoot,
            tree,
            &mut policy_roots,
            &mut input_failures,
        );

        for member_rel in &workspace_snapshot.declared_members {
            if let Some(member_snapshot) = snapshots.get(member_rel) {
                member_facts.push(build_member_facts(workspace_snapshot, member_snapshot));
                if let Some(parse_error) = &member_snapshot.parse_error {
                    input_failures.push(InputFailureFacts {
                        rel_path: member_snapshot.cargo_rel_path.clone(),
                        message: format!(
                            "Failed to parse workspace member Cargo.toml for cargo lint policy checks: {parse_error}"
                        ),
                    });
                }
            } else {
                missing_members.push(MissingMemberCargoFacts {
                    workspace_root_rel: workspace_root_rel.clone(),
                    workspace_cargo_rel_path: workspace_snapshot.cargo_rel_path.clone(),
                    member_rel: member_rel.clone(),
                });
            }
        }
    }

    for snapshot in snapshots.values() {
        if snapshot.has_workspace {
            continue;
        }
        if workspace_members.contains(&snapshot.rel_dir) {
            continue;
        }
        if snapshot.has_package || snapshot.parse_error.is_some() {
            push_policy_root(
                snapshot,
                PolicyRootKind::StandalonePackageRoot,
                tree,
                &mut policy_roots,
                &mut input_failures,
            );
        }
    }

    member_facts.sort_by(|a, b| {
        a.workspace_root_rel
            .cmp(&b.workspace_root_rel)
            .then(a.member_rel.cmp(&b.member_rel))
    });
    missing_members.sort_by(|a, b| {
        a.workspace_root_rel
            .cmp(&b.workspace_root_rel)
            .then(a.member_rel.cmp(&b.member_rel))
    });
    policy_roots.sort_by(|a, b| a.cargo_rel_path.cmp(&b.cargo_rel_path));
    input_failures.sort_by(|a, b| a.rel_path.cmp(&b.rel_path).then(a.message.cmp(&b.message)));
    input_failures.dedup_by(|a, b| a.rel_path == b.rel_path && a.message == b.message);

    CargoFamilyFacts {
        policy_roots,
        workspace_members: member_facts,
        missing_members,
        input_failures,
    }
}

fn collect_cargo_snapshots(
    tree: &ProjectTree,
    route: &RsCargoRoute,
) -> BTreeMap<String, CargoSnapshot> {
    route
        .roots()
        .iter()
        .map(|root| {
            let rel_dir = normalize_member_rel(root.rel_dir());
            let cargo_rel_path = root.cargo_rel_path().to_owned();
            let snapshot = snapshot_for(tree, &rel_dir, &cargo_rel_path);
            (rel_dir, snapshot)
        })
        .collect()
}

fn snapshot_for(tree: &ProjectTree, rel_dir: &str, cargo_rel_path: &str) -> CargoSnapshot {
    let Some(content) = tree.file_content(cargo_rel_path) else {
        return CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            parsed: None,
            parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
            has_workspace: false,
            has_package: false,
            edition: None,
            rust_version: None,
            resolver: None,
            declared_members: Vec::new(),
        };
    };

    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => {
            let has_workspace = parsed.get("workspace").is_some();
            CargoSnapshot {
                rel_dir: rel_dir.to_owned(),
                cargo_rel_path: cargo_rel_path.to_owned(),
                parsed: Some(parsed.clone()),
                parse_error: None,
                has_workspace,
                has_package: parsed.get("package").is_some(),
                edition: root_package_field(&parsed, has_workspace, "edition"),
                rust_version: root_package_field(&parsed, has_workspace, "rust-version"),
                resolver: parsed
                    .get("workspace")
                    .and_then(|value| value.get("resolver"))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
                declared_members: if has_workspace {
                    parse_workspace_members(tree, rel_dir, &parsed)
                } else {
                    Vec::new()
                },
            }
        }
        Err(parse_error) => CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            parsed: None,
            parse_error: Some(parse_error.to_string()),
            has_workspace: false,
            has_package: false,
            edition: None,
            rust_version: None,
            resolver: None,
            declared_members: Vec::new(),
        },
    }
}

fn push_policy_root(
    snapshot: &CargoSnapshot,
    kind: PolicyRootKind,
    tree: &ProjectTree,
    policy_roots: &mut Vec<PolicyRootCargoFacts>,
    input_failures: &mut Vec<InputFailureFacts>,
) {
    let guardrail_rel_path = rel_path(&snapshot.rel_dir, "guardrail3.toml");
    let guardrail = guardrail_snapshot(tree, &guardrail_rel_path);
    if let Some(parse_error) = &snapshot.parse_error {
        input_failures.push(InputFailureFacts {
            rel_path: snapshot.cargo_rel_path.clone(),
            message: format!(
                "Failed to parse owned policy-root Cargo.toml for cargo lint policy checks: {parse_error}"
            ),
        });
    }
    if let Some(parse_error) = &guardrail.parse_error {
        input_failures.push(InputFailureFacts {
            rel_path: guardrail_rel_path.clone(),
            message: format!(
                "Failed to parse root-local guardrail3.toml for cargo profile resolution: {parse_error}"
            ),
        });
    }

    policy_roots.push(PolicyRootCargoFacts {
        kind,
        rel_dir: snapshot.rel_dir.clone(),
        cargo_rel_path: snapshot.cargo_rel_path.clone(),
        parsed: snapshot.parsed.clone(),
        parse_error: snapshot.parse_error.clone(),
        edition: snapshot.edition.clone(),
        rust_version: snapshot.rust_version.clone(),
        resolver: snapshot.resolver.clone(),
        profile_name: guardrail.profile_name,
    });
}

fn build_member_facts(
    workspace_snapshot: &CargoSnapshot,
    member_snapshot: &CargoSnapshot,
) -> WorkspaceMemberCargoFacts {
    let parsed = member_snapshot.parsed.as_ref();
    WorkspaceMemberCargoFacts {
        workspace_root_rel: workspace_snapshot.rel_dir.clone(),
        member_rel: member_snapshot.rel_dir.clone(),
        cargo_rel_path: member_snapshot.cargo_rel_path.clone(),
        parsed: member_snapshot.parsed.clone(),
        package_name: parsed
            .and_then(|value| value.get("package"))
            .and_then(|value| value.get("name"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
        edition: parsed
            .and_then(|value| value.get("package"))
            .and_then(|value| value.get("edition"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
        lint_workspace_true: parsed
            .and_then(|value| value.get("lints"))
            .and_then(|value| value.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false),
        parse_error: member_snapshot.parse_error.clone(),
    }
}

fn parse_workspace_members(
    tree: &ProjectTree,
    workspace_rel: &str,
    parsed: &toml::Value,
) -> Vec<String> {
    let mut members = BTreeSet::new();
    for pattern in raw_member_patterns(parsed) {
        for member_rel in expand_member_pattern(tree, workspace_rel, &pattern) {
            let _ = members.insert(member_rel);
        }
    }
    members.into_iter().collect()
}

fn raw_member_patterns(parsed: &toml::Value) -> Vec<String> {
    parsed
        .get("workspace")
        .and_then(|value| value.get("members"))
        .and_then(toml::Value::as_array)
        .map(|members| {
            members
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn expand_member_pattern(tree: &ProjectTree, workspace_rel: &str, pattern: &str) -> Vec<String> {
    let normalized = normalize_member_rel(pattern);
    let rel_pattern = if workspace_rel.is_empty() {
        normalized.clone()
    } else {
        ProjectTree::join_rel(workspace_rel, &normalized)
    };

    if looks_like_glob(&normalized) {
        tree.matching_dir_rels(&rel_pattern)
            .into_iter()
            .map(|rel| normalize_member_rel(&rel))
            .collect()
    } else {
        vec![normalize_member_rel(&rel_pattern)]
    }
}

fn looks_like_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

fn normalize_member_rel(pattern: &str) -> String {
    pattern
        .trim_matches('/')
        .strip_prefix("./")
        .unwrap_or(pattern.trim_matches('/'))
        .trim_matches('/')
        .to_owned()
}

fn guardrail_snapshot(tree: &ProjectTree, rel_path: &str) -> GuardrailSnapshot {
    let Some(content) = tree.file_content(rel_path) else {
        return GuardrailSnapshot::default();
    };
    match toml::from_str::<GuardrailConfig>(content) {
        Ok(parsed) => GuardrailSnapshot {
            profile_name: parsed.profile().map(|profile| profile.name().to_owned()),
            parse_error: None,
        },
        Err(parse_error) => GuardrailSnapshot {
            profile_name: None,
            parse_error: Some(parse_error.to_string()),
        },
    }
}

fn root_package_field(
    parsed: &toml::Value,
    is_workspace_root: bool,
    field: &str,
) -> Option<String> {
    if is_workspace_root {
        parsed
            .get("workspace")
            .and_then(|value| value.get("package"))
            .and_then(|value| value.get(field))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
            .or_else(|| {
                parsed
                    .get("package")
                    .and_then(|value| value.get(field))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
            })
    } else {
        parsed
            .get("package")
            .and_then(|value| value.get(field))
            .and_then(toml::Value::as_str)
            .map(str::to_owned)
    }
}

fn rel_path(rel_dir: &str, file_name: &str) -> String {
    if rel_dir.is_empty() {
        file_name.to_owned()
    } else {
        ProjectTree::join_rel(rel_dir, file_name)
    }
}
