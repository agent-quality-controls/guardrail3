use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsCargoRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_domain_config::types::{EscapeHatchConfig, GuardrailConfig};
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::facts::{
    CargoFamilyFacts, InputFailureFacts, MissingMemberCargoFacts, PolicyRootCargoFacts,
    PolicyRootKind, WorkspaceMemberCargoFacts,
};

#[derive(Debug, Clone)]
struct CargoSnapshot {
    rel_dir: String,
    cargo_rel_path: String,
    parsed: Option<toml::Value>,
    parsed_typed: Option<cargo_toml_parser::CargoToml>,
    parse_error: Option<String>,
    typed_parse_error: Option<String>,
    has_workspace: bool,
    has_package: bool,
    edition: Option<String>,
    edition_invalid: bool,
    rust_version: Option<String>,
    rust_version_invalid: bool,
    declared_members: Vec<String>,
    members_parse_error: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct GuardrailSnapshot {
    profile_name: Option<String>,
    escape_hatches: Vec<EscapeHatchConfig>,
    parse_error: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct StringFieldSnapshot {
    value: Option<String>,
    invalid: bool,
}

#[derive(Debug, Clone, Default)]
struct RouteFileIndex {
    cargo_by_owner: BTreeMap<String, String>,
    guardrail_by_owner: BTreeMap<String, String>,
}

pub fn collect(tree: &ProjectTree, route: &RsCargoRoute) -> CargoFamilyFacts {
    let route_files = collect_route_file_index(route);
    let snapshots = collect_cargo_snapshots(tree, route, &route_files);
    let workspace_roots: Vec<_> = snapshots
        .values()
        .filter(|snapshot| snapshot.has_workspace)
        .map(|snapshot| snapshot.rel_dir.clone())
        .collect();
    let mut policy_roots = Vec::new();
    let mut member_facts = Vec::new();
    let mut missing_members = Vec::new();
    let mut input_failures = Vec::new();
    let workspace_members: BTreeSet<_> = snapshots
        .values()
        .filter(|snapshot| snapshot.has_workspace)
        .flat_map(|snapshot| snapshot.declared_members.iter().cloned())
        .collect();
    let invalid_workspace_member_roots: BTreeSet<_> = snapshots
        .values()
        .filter(|snapshot| snapshot.members_parse_error.is_some())
        .map(|snapshot| snapshot.rel_dir.clone())
        .collect();

    for workspace_root_rel in &workspace_roots {
        let Some(workspace_snapshot) = snapshots.get(workspace_root_rel) else {
            continue;
        };
        if let Some(parse_error) = &workspace_snapshot.members_parse_error {
            input_failures.push(InputFailureFacts {
                rel_path: workspace_snapshot.cargo_rel_path.clone(),
                message: format!(
                    "Failed to parse `[workspace].members` for cargo workspace membership checks: {parse_error}"
                ),
            });
        }
        push_policy_root(
            workspace_snapshot,
            PolicyRootKind::WorkspaceRoot,
            tree,
            &route_files,
            &mut policy_roots,
            &mut input_failures,
        );

        for member_rel in &workspace_snapshot.declared_members {
            let member_snapshot = snapshots.get(member_rel).cloned().or_else(|| {
                route_files
                    .cargo_rel_path(member_rel)
                    .map(|member_cargo_rel_path| {
                        snapshot_for(tree, member_rel, member_cargo_rel_path)
                    })
                    .or_else(|| {
                        let member_cargo_rel_path = rel_path(member_rel, "Cargo.toml");
                        tree.file_exists(&member_cargo_rel_path)
                            .then(|| snapshot_for(tree, member_rel, &member_cargo_rel_path))
                    })
            });

            if let Some(member_snapshot) = member_snapshot {
                member_facts.push(build_member_facts(workspace_snapshot, &member_snapshot));
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
        if invalid_workspace_member_roots
            .iter()
            .any(|workspace_root| is_descendant_of(&snapshot.rel_dir, workspace_root))
        {
            continue;
        }
        if snapshot.has_package || snapshot.parse_error.is_some() {
            if workspace_roots
                .iter()
                .any(|workspace_root| is_descendant_of(&snapshot.rel_dir, workspace_root))
            {
                continue;
            }
            push_policy_root(
                snapshot,
                PolicyRootKind::StandalonePackageRoot,
                tree,
                &route_files,
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
    route_files: &RouteFileIndex,
) -> BTreeMap<String, CargoSnapshot> {
    let mut candidate_rels = route
        .roots()
        .iter()
        .map(|root| normalize_member_rel(root.rel_dir()))
        .collect::<BTreeSet<_>>();
    candidate_rels.extend(route_files.cargo_by_owner.keys().cloned());

    candidate_rels
        .into_iter()
        .map(|rel_dir| {
            let cargo_rel_path = route_files
                .cargo_rel_path(&rel_dir)
                .map_or_else(|| rel_path(&rel_dir, "Cargo.toml"), ToOwned::to_owned);
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
            parsed_typed: None,
            parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
            typed_parse_error: None,
            has_workspace: false,
            has_package: false,
            edition: None,
            edition_invalid: false,
            rust_version: None,
            rust_version_invalid: false,
            declared_members: Vec::new(),
            members_parse_error: None,
        };
    };

    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => {
            let (parsed_typed, typed_parse_error) = match cargo_toml_parser::parse(content) {
                Ok(parsed_typed) => (Some(parsed_typed), None),
                Err(error) => (None, Some(error.to_string())),
            };
            let has_workspace = parsed.get("workspace").is_some();
            let edition = root_package_field(&parsed, has_workspace, "edition");
            let rust_version = root_package_field(&parsed, has_workspace, "rust-version");
            let (declared_members, members_parse_error) = if has_workspace {
                match parse_workspace_members(tree, rel_dir, &parsed) {
                    Ok(members) => (members, None),
                    Err(parse_error) => (Vec::new(), Some(parse_error)),
                }
            } else {
                (Vec::new(), None)
            };
            CargoSnapshot {
                rel_dir: rel_dir.to_owned(),
                cargo_rel_path: cargo_rel_path.to_owned(),
                parsed: Some(parsed.clone()),
                parsed_typed,
                parse_error: None,
                typed_parse_error,
                has_workspace,
                has_package: parsed.get("package").is_some(),
                edition: edition.value,
                edition_invalid: edition.invalid,
                rust_version: rust_version.value,
                rust_version_invalid: rust_version.invalid,
                declared_members,
                members_parse_error,
            }
        }
        Err(parse_error) => CargoSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            parsed: None,
            parsed_typed: None,
            parse_error: Some(parse_error.to_string()),
            typed_parse_error: None,
            has_workspace: false,
            has_package: false,
            edition: None,
            edition_invalid: false,
            rust_version: None,
            rust_version_invalid: false,
            declared_members: Vec::new(),
            members_parse_error: None,
        },
    }
}

fn push_policy_root(
    snapshot: &CargoSnapshot,
    kind: PolicyRootKind,
    tree: &ProjectTree,
    route_files: &RouteFileIndex,
    policy_roots: &mut Vec<PolicyRootCargoFacts>,
    input_failures: &mut Vec<InputFailureFacts>,
) {
    let guardrail_rel_path = route_files
        .guardrail_rel_path(&snapshot.rel_dir)
        .map(ToOwned::to_owned);
    let guardrail = guardrail_snapshot(tree, guardrail_rel_path.as_deref());
    if let Some(parse_error) = &snapshot.parse_error {
        input_failures.push(InputFailureFacts {
            rel_path: snapshot.cargo_rel_path.clone(),
            message: format!(
                "Failed to parse owned policy-root Cargo.toml for cargo lint policy checks: {parse_error}"
            ),
        });
    }
    if let Some(parse_error) = &snapshot.typed_parse_error
        && snapshot.members_parse_error.is_none()
    {
        input_failures.push(InputFailureFacts {
            rel_path: snapshot.cargo_rel_path.clone(),
            message: format!(
                "Failed to parse owned policy-root Cargo.toml against cargo-toml-parser for cargo config checks: {parse_error}"
            ),
        });
    }
    if let Some(parse_error) = &guardrail.parse_error {
        input_failures.push(InputFailureFacts {
            rel_path: guardrail_rel_path
                .clone()
                .unwrap_or_else(|| rel_path(&snapshot.rel_dir, "guardrail3.toml")),
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
        parsed_typed: snapshot.parsed_typed.clone(),
        parse_error: snapshot.parse_error.clone(),
        guardrail_parse_error: guardrail.parse_error.is_some(),
        members_parse_error: snapshot.members_parse_error.is_some(),
        edition: snapshot.edition.clone(),
        edition_invalid: snapshot.edition_invalid,
        rust_version: snapshot.rust_version.clone(),
        rust_version_invalid: snapshot.rust_version_invalid,
        profile_name: guardrail.profile_name,
        escape_hatches: guardrail.escape_hatches,
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
        edition: package_field(parsed, "edition").value,
        edition_invalid: package_field(parsed, "edition").invalid,
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
) -> Result<Vec<String>, String> {
    let Some(raw_members) = parsed
        .get("workspace")
        .and_then(|value| value.get("members"))
    else {
        return Ok(Vec::new());
    };
    let Some(raw_members) = raw_members.as_array() else {
        return Err("`[workspace].members` must be an array of strings.".to_owned());
    };

    let mut members = BTreeSet::new();
    for pattern in raw_member_patterns(raw_members)? {
        for member_rel in expand_member_pattern(tree, workspace_rel, &pattern) {
            let _ = members.insert(member_rel);
        }
    }
    Ok(members.into_iter().collect())
}

fn raw_member_patterns(members: &[toml::Value]) -> Result<Vec<String>, String> {
    members
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "`[workspace].members` must contain only string entries.".to_owned())
        })
        .collect()
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

fn guardrail_snapshot(tree: &ProjectTree, rel_path: Option<&str>) -> GuardrailSnapshot {
    let Some(rel_path) = rel_path else {
        return GuardrailSnapshot::default();
    };
    let Some(content) = tree.file_content(rel_path) else {
        return GuardrailSnapshot::default();
    };
    match toml::from_str::<GuardrailConfig>(content) {
        Ok(parsed) => GuardrailSnapshot {
            profile_name: parsed.profile().map(|profile| profile.name().to_owned()),
            escape_hatches: parsed.escape_hatches().to_vec(),
            parse_error: None,
        },
        Err(parse_error) => GuardrailSnapshot {
            profile_name: None,
            escape_hatches: Vec::new(),
            parse_error: Some(parse_error.to_string()),
        },
    }
}

fn is_descendant_of(rel: &str, ancestor: &str) -> bool {
    if ancestor.is_empty() {
        return !rel.is_empty();
    }

    rel.strip_prefix(ancestor)
        .is_some_and(|suffix| suffix.starts_with('/'))
}

fn root_package_field(
    parsed: &toml::Value,
    is_workspace_root: bool,
    field: &str,
) -> StringFieldSnapshot {
    if is_workspace_root {
        let workspace_package = string_field(
            parsed
                .get("workspace")
                .and_then(|value| value.get("package")),
            field,
        );
        if workspace_package.value.is_some() || workspace_package.invalid {
            workspace_package
        } else {
            package_field(Some(parsed), field)
        }
    } else {
        package_field(Some(parsed), field)
    }
}

fn package_field(parsed: Option<&toml::Value>, field: &str) -> StringFieldSnapshot {
    string_field(parsed.and_then(|value| value.get("package")), field)
}

fn string_field(table: Option<&toml::Value>, field: &str) -> StringFieldSnapshot {
    let Some(value) = table.and_then(|table| table.get(field)) else {
        return StringFieldSnapshot::default();
    };

    match value.as_str() {
        Some(field_value) => StringFieldSnapshot {
            value: Some(field_value.to_owned()),
            invalid: false,
        },
        None => StringFieldSnapshot {
            value: None,
            invalid: true,
        },
    }
}

fn rel_path(rel_dir: &str, file_name: &str) -> String {
    if rel_dir.is_empty() {
        file_name.to_owned()
    } else {
        ProjectTree::join_rel(rel_dir, file_name)
    }
}

fn collect_route_file_index(route: &RsCargoRoute) -> RouteFileIndex {
    let mut cargo_by_owner = BTreeMap::new();
    let mut guardrail_by_owner = BTreeMap::new();

    for file in route.family_files() {
        match file.kind() {
            RustFamilyFileKind::CargoToml => {
                let _ = cargo_by_owner.insert(
                    file.logical_owner_rel().to_owned(),
                    file.rel_path().to_owned(),
                );
            }
            RustFamilyFileKind::GuardrailToml if file.exact_rust_root_owner() => {
                let _ = guardrail_by_owner.insert(
                    file.logical_owner_rel().to_owned(),
                    file.rel_path().to_owned(),
                );
            }
            RustFamilyFileKind::GuardrailToml => {
                if let Some(root_rels) = file.ancestor_rust_root_rels() {
                    for root_rel in root_rels {
                        let _ = guardrail_by_owner
                            .entry(root_rel.clone())
                            .or_insert_with(|| file.rel_path().to_owned());
                    }
                }
            }
            _ => {}
        }
    }

    RouteFileIndex {
        cargo_by_owner,
        guardrail_by_owner,
    }
}

impl RouteFileIndex {
    fn cargo_rel_path(&self, owner_rel: &str) -> Option<&str> {
        self.cargo_by_owner.get(owner_rel).map(String::as_str)
    }

    fn guardrail_rel_path(&self, owner_rel: &str) -> Option<&str> {
        self.guardrail_by_owner.get(owner_rel).map(String::as_str)
    }
}
