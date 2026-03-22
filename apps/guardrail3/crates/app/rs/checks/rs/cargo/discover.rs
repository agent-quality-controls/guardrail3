use std::collections::BTreeSet;

use glob::Pattern;

use crate::domain::project_tree::ProjectTree;

use super::facts::{CargoFamilyFacts, MemberCargoFacts, WorkspaceCargoFacts};

pub fn collect(tree: &ProjectTree) -> Option<CargoFamilyFacts> {
    let workspace_content = tree.file_content("Cargo.toml")?;
    let discovered_member_rels = discover_member_dirs(tree);
    let workspace_parsed = match toml::from_str::<toml::Value>(workspace_content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return Some(CargoFamilyFacts {
                workspace: WorkspaceCargoFacts {
                    rel_path: "Cargo.toml".to_owned(),
                    parsed: None,
                    declared_members: BTreeSet::new(),
                    workspace_edition: None,
                    workspace_rust_version: None,
                    resolver: None,
                    has_package: false,
                    parse_error: Some(err.to_string()),
                },
                members: Vec::new(),
                discovered_member_rels,
            });
        }
    };

    if workspace_parsed.get("workspace").is_none() {
        return None;
    }

    let declared_members = resolve_declared_members(&workspace_parsed, &discovered_member_rels);
    let members = declared_members
        .iter()
        .filter(|member_rel| discovered_member_rels.contains(*member_rel))
        .map(|member_rel| build_member_facts(tree, member_rel))
        .collect();

    Some(CargoFamilyFacts {
        workspace: WorkspaceCargoFacts {
            rel_path: "Cargo.toml".to_owned(),
            parsed: Some(workspace_parsed.clone()),
            declared_members: declared_members.clone(),
            workspace_edition: workspace_package_field(&workspace_parsed, "edition"),
            workspace_rust_version: workspace_package_field(&workspace_parsed, "rust-version"),
            resolver: workspace_parsed
                .get("workspace")
                .and_then(|value| value.get("resolver"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned),
            has_package: workspace_parsed.get("package").is_some(),
            parse_error: None,
        },
        members,
        discovered_member_rels,
    })
}

fn discover_member_dirs(tree: &ProjectTree) -> BTreeSet<String> {
    tree.structure
        .iter()
        .filter_map(|(dir_rel, entry)| {
            if dir_rel.is_empty() || !entry.has_file("Cargo.toml") {
                None
            } else {
                Some(dir_rel.clone())
            }
        })
        .collect()
}

fn resolve_declared_members(
    workspace_parsed: &toml::Value,
    discovered_member_rels: &BTreeSet<String>,
) -> BTreeSet<String> {
    raw_member_patterns(workspace_parsed)
        .into_iter()
        .flat_map(|pattern| expand_member_pattern(&pattern, discovered_member_rels))
        .collect()
}

fn raw_member_patterns(workspace_parsed: &toml::Value) -> Vec<String> {
    workspace_parsed
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

fn expand_member_pattern(pattern: &str, discovered_member_rels: &BTreeSet<String>) -> Vec<String> {
    let normalized = normalize_member_rel(pattern);
    if !looks_like_glob(&normalized) {
        return vec![normalized];
    }

    let Ok(pattern) = Pattern::new(&normalized) else {
        return Vec::new();
    };

    discovered_member_rels
        .iter()
        .filter(|member_rel| pattern.matches(member_rel))
        .cloned()
        .collect()
}

fn looks_like_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

fn normalize_member_rel(pattern: &str) -> String {
    pattern.trim_matches('/').to_owned()
}

fn build_member_facts(tree: &ProjectTree, member_rel: &str) -> MemberCargoFacts {
    let cargo_rel = ProjectTree::join_rel(member_rel, "Cargo.toml");
    let Some(content) = tree.file_content(&cargo_rel) else {
        return MemberCargoFacts {
            rel_path: cargo_rel,
            member_rel: member_rel.to_owned(),
            parsed: None,
            package_name: None,
            edition: None,
            lint_workspace_true: false,
            parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
        };
    };

    let parsed = match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return MemberCargoFacts {
                rel_path: cargo_rel,
                member_rel: member_rel.to_owned(),
                parsed: None,
                package_name: None,
                edition: None,
                lint_workspace_true: false,
                parse_error: Some(err.to_string()),
            };
        }
    };

    MemberCargoFacts {
        rel_path: cargo_rel,
        member_rel: member_rel.to_owned(),
        parsed: Some(parsed.clone()),
        package_name: parsed
            .get("package")
            .and_then(|value| value.get("name"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
        edition: parsed
            .get("package")
            .and_then(|value| value.get("edition"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
        lint_workspace_true: parsed
            .get("lints")
            .and_then(|value| value.get("workspace"))
            .and_then(toml::Value::as_bool)
            .unwrap_or(false),
        parse_error: None,
    }
}

fn workspace_package_field(workspace_parsed: &toml::Value, field: &str) -> Option<String> {
    workspace_parsed
        .get("workspace")
        .and_then(|value| value.get("package"))
        .and_then(|value| value.get(field))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
        .or_else(|| {
            workspace_parsed
                .get("package")
                .and_then(|value| value.get(field))
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
        })
}
