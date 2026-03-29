use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

use guardrail3_app_rs_family_mapper::RsDepsRoute;
use guardrail3_domain_config::types::{CrateConfig, GuardrailConfig};
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::ToolChecker;

#[derive(Debug, Clone)]
pub struct ToolFacts {
    pub tool_name: String,
    pub installed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LockfileFacts {
    pub root_rel_dir: String,
    pub cargo_lock_rel_path: String,
    pub cargo_lock_exists: bool,
    pub cargo_lock_ignored: bool,
    pub gitignore_rel_path: Option<String>,
    pub profile_name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencySectionKind {
    Dependencies,
    BuildDependencies,
    DevDependencies,
}

impl DependencySectionKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Dependencies => "dependencies",
            Self::BuildDependencies => "build-dependencies",
            Self::DevDependencies => "dev-dependencies",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DependencyEntryFacts {
    pub crate_name: String,
    pub cargo_rel_path: String,
    pub section_kind: DependencySectionKind,
    pub dep_alias: String,
    pub dep_package_name: String,
    pub allowlist_present: bool,
    pub allowlisted: bool,
}

#[derive(Debug, Clone)]
pub struct AllowlistCoverageFacts {
    pub crate_name: String,
    pub cargo_rel_path: String,
    pub profile_name: Option<String>,
    pub has_allowlist: bool,
}

#[derive(Debug, Clone)]
pub struct InputFailureFacts {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct DepsFacts {
    pub tools: Vec<ToolFacts>,
    pub lockfiles: Vec<LockfileFacts>,
    pub dependency_entries: Vec<DependencyEntryFacts>,
    pub allowlist_coverage: Vec<AllowlistCoverageFacts>,
    pub input_failures: Vec<InputFailureFacts>,
}

pub fn collect(tree: &ProjectTree, route: &RsDepsRoute, tc: &dyn ToolChecker) -> DepsFacts {
    let parsed_guardrail = parse_guardrail(tree);
    let mut input_failures = parsed_guardrail
        .as_ref()
        .and_then(|guardrail| guardrail.parse_error.clone())
        .map(|message| {
            vec![InputFailureFacts {
                rel_path: "guardrail3.toml".to_owned(),
                message,
            }]
        })
        .unwrap_or_default();
    let routed_root_rels = route
        .roots
        .iter()
        .map(|root| root.rel_dir.clone())
        .collect::<BTreeSet<_>>();
    let workspaces = discover_workspaces(tree, route, &mut input_failures);
    let workspace_by_member = workspace_by_member(&workspaces);
    let members = discover_members(
        tree,
        &routed_root_rels,
        &workspaces,
        &workspace_by_member,
        &parsed_guardrail,
        &mut input_failures,
    );

    let dependency_entries =
        collect_dependency_entries(tree, &members, &workspaces, &mut input_failures);
    let lockfiles = collect_lockfiles(
        tree,
        &workspaces,
        &members,
        parsed_guardrail.as_ref(),
        &mut input_failures,
    );
    let allowlist_coverage = members
        .into_iter()
        .map(|member| AllowlistCoverageFacts {
            crate_name: member.crate_name,
            cargo_rel_path: member.cargo_rel_path,
            profile_name: member.profile_name,
            has_allowlist: member.allowed_deps.is_some(),
        })
        .collect();

    DepsFacts {
        tools: vec![
            ToolFacts {
                tool_name: "cargo-deny".to_owned(),
                installed: tc.is_installed("cargo-deny"),
            },
            ToolFacts {
                tool_name: "cargo-machete".to_owned(),
                installed: tc.is_installed("cargo-machete"),
            },
            ToolFacts {
                tool_name: "cargo-dupes".to_owned(),
                installed: tc.is_installed("cargo-dupes"),
            },
            ToolFacts {
                tool_name: "gitleaks".to_owned(),
                installed: tc.is_installed("gitleaks"),
            },
        ],
        lockfiles,
        dependency_entries,
        allowlist_coverage,
        input_failures,
    }
}

#[derive(Debug, Clone)]
struct ParsedGuardrail {
    root_profile_name: Option<String>,
    apps: BTreeMap<String, CrateConfig>,
    packages: Option<CrateConfig>,
    parse_error: Option<String>,
}

#[derive(Debug, Clone)]
struct WorkspaceFacts {
    root_rel_dir: String,
    cargo_rel_path: String,
    workspace_dependencies: toml::map::Map<String, toml::Value>,
    workspace_package_dirs: BTreeSet<String>,
    member_dirs: Vec<String>,
}

#[derive(Debug, Clone)]
struct MemberFacts {
    crate_name: String,
    rel_dir: String,
    cargo_rel_path: String,
    workspace_root_rel_dir: Option<String>,
    profile_name: Option<String>,
    allowed_deps: Option<BTreeSet<String>>,
}

fn parse_guardrail(tree: &ProjectTree) -> Option<ParsedGuardrail> {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        return tree
            .file_exists("guardrail3.toml")
            .then(|| ParsedGuardrail {
                root_profile_name: None,
                apps: BTreeMap::new(),
                packages: None,
                parse_error: Some(
                    "Failed to read guardrail3.toml for dependency policy resolution.".to_owned(),
                ),
            });
    };
    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => match validate_deps_guardrail_shape(&parsed) {
            Ok(()) => match parsed.clone().try_into::<GuardrailConfig>() {
                Ok(config) => Some(ParsedGuardrail {
                    root_profile_name: config.profile.map(|profile| profile.name),
                    apps: config
                        .rust
                        .as_ref()
                        .and_then(|rust| rust.apps.clone())
                        .unwrap_or_default(),
                    packages: config.rust.and_then(|rust| rust.packages),
                    parse_error: None,
                }),
                Err(parse_error) => Some(ParsedGuardrail {
                    root_profile_name: None,
                    apps: BTreeMap::new(),
                    packages: None,
                    parse_error: Some(format!(
                        "Failed to parse guardrail3.toml for dependency policy resolution: {parse_error}"
                    )),
                }),
            },
            Err(parse_error) => Some(ParsedGuardrail {
                root_profile_name: None,
                apps: BTreeMap::new(),
                packages: None,
                parse_error: Some(format!(
                    "Failed to parse guardrail3.toml for dependency policy resolution: {parse_error}"
                )),
            }),
        },
        Err(parse_error) => Some(ParsedGuardrail {
            root_profile_name: None,
            apps: BTreeMap::new(),
            packages: None,
            parse_error: Some(format!(
                "Failed to parse guardrail3.toml for dependency policy resolution: {parse_error}"
            )),
        }),
    }
}

fn validate_deps_guardrail_shape(parsed: &toml::Value) -> Result<(), String> {
    let Some(root) = parsed.as_table() else {
        return Err("guardrail3.toml root must be a table.".to_owned());
    };

    if let Some(profile) = root.get("profile") {
        let Some(profile) = profile.as_table() else {
            return Err("`profile` must be a table.".to_owned());
        };
        if let Some(name) = profile.get("name") {
            let Some(name) = name.as_str() else {
                return Err("`profile.name` must be a string.".to_owned());
            };
            if name.is_empty() {
                return Err("`profile.name` must be non-empty.".to_owned());
            }
        }
    }

    let Some(rust) = root.get("rust") else {
        return Ok(());
    };
    let Some(rust) = rust.as_table() else {
        return Err("`rust` must be a table.".to_owned());
    };

    for key in rust.keys() {
        if !matches!(
            key.as_str(),
            "workspace_root" | "workspaces" | "apps" | "packages" | "checks"
        ) {
            return Err(format!("`rust` contains unsupported key `{key}`."));
        }
    }

    if let Some(apps) = rust.get("apps") {
        let Some(apps) = apps.as_table() else {
            return Err("`rust.apps` must be a table.".to_owned());
        };
        for (app_name, config) in apps {
            validate_crate_policy_table(config, &format!("rust.apps.{app_name}"))?;
        }
    }

    if let Some(packages) = rust.get("packages") {
        validate_crate_policy_table(packages, "rust.packages")?;
    }

    Ok(())
}

fn validate_crate_policy_table(value: &toml::Value, context: &str) -> Result<(), String> {
    let Some(table) = value.as_table() else {
        return Err(format!("`{context}` must be a table."));
    };

    for key in table.keys() {
        if !matches!(
            key.as_str(),
            "layer" | "profile" | "type" | "allowed_deps" | "checks"
        ) {
            return Err(format!("`{context}` contains unsupported key `{key}`."));
        }
    }

    for key in ["layer", "profile", "type"] {
        if let Some(value) = table.get(key) {
            let Some(value) = value.as_str() else {
                return Err(format!("`{context}.{key}` must be a string."));
            };
            if value.is_empty() {
                return Err(format!("`{context}.{key}` must be non-empty."));
            }
        }
    }

    if let Some(allowed_deps) = table.get("allowed_deps") {
        let Some(allowed_deps) = allowed_deps.as_array() else {
            return Err(format!(
                "`{context}.allowed_deps` must be an array of strings."
            ));
        };
        for dep in allowed_deps {
            let Some(dep) = dep.as_str() else {
                return Err(format!(
                    "`{context}.allowed_deps` must contain only strings."
                ));
            };
            if dep.is_empty() {
                return Err(format!(
                    "`{context}.allowed_deps` must not contain empty dependency names."
                ));
            }
        }
    }

    Ok(())
}

fn validate_workspace_manifest_shape(parsed: &toml::Value) -> Result<(), String> {
    let Some(workspace) = parsed.get("workspace") else {
        return Ok(());
    };
    let Some(workspace) = workspace.as_table() else {
        return Err("`[workspace]` must be a table.".to_owned());
    };

    if let Some(members) = workspace.get("members") {
        let Some(members) = members.as_array() else {
            return Err("`[workspace].members` must be an array of strings.".to_owned());
        };
        for member in members {
            let Some(member) = member.as_str() else {
                return Err("`[workspace].members` must contain only strings.".to_owned());
            };
            if member.is_empty() {
                return Err("`[workspace].members` must not contain empty patterns.".to_owned());
            }
        }
    }

    if let Some(dependencies) = workspace.get("dependencies") {
        let Some(dependencies) = dependencies.as_table() else {
            return Err("`[workspace.dependencies]` must be a table.".to_owned());
        };
        for (alias, value) in dependencies {
            validate_workspace_dependency_shape(alias, value)?;
        }
    }

    Ok(())
}

fn validate_workspace_dependency_shape(alias: &str, value: &toml::Value) -> Result<(), String> {
    if value.is_str() {
        return Ok(());
    }

    let Some(table) = value.as_table() else {
        return Err(format!(
            "`[workspace.dependencies].{alias}` must be a string or table."
        ));
    };

    if let Some(package) = table.get("package") {
        let Some(package) = package.as_str() else {
            return Err(format!(
                "`[workspace.dependencies].{alias}.package` must be a string."
            ));
        };
        if package.is_empty() {
            return Err(format!(
                "`[workspace.dependencies].{alias}.package` must be non-empty."
            ));
        }
    }

    if let Some(path) = table.get("path") {
        let Some(path) = path.as_str() else {
            return Err(format!(
                "`[workspace.dependencies].{alias}.path` must be a string."
            ));
        };
        if path.is_empty() {
            return Err(format!(
                "`[workspace.dependencies].{alias}.path` must be non-empty."
            ));
        }
    }

    Ok(())
}

fn validate_dependency_manifest_shape(parsed: &toml::Value) -> Result<(), String> {
    for section_key in ["dependencies", "build-dependencies", "dev-dependencies"] {
        let Some(section) = parsed.get(section_key) else {
            continue;
        };
        let Some(section) = section.as_table() else {
            return Err(format!("`[{section_key}]` must be a table."));
        };
        for (alias, value) in section {
            validate_dependency_spec_shape(section_key, alias, value)?;
        }
    }

    Ok(())
}

fn validate_dependency_spec_shape(
    section_key: &str,
    alias: &str,
    value: &toml::Value,
) -> Result<(), String> {
    if value.is_str() {
        return Ok(());
    }

    let Some(table) = value.as_table() else {
        return Err(format!(
            "`[{section_key}].{alias}` must be a string or table."
        ));
    };

    if let Some(package) = table.get("package") {
        let Some(package) = package.as_str() else {
            return Err(format!(
                "`[{section_key}].{alias}.package` must be a string."
            ));
        };
        if package.is_empty() {
            return Err(format!(
                "`[{section_key}].{alias}.package` must be non-empty."
            ));
        }
    }

    if let Some(path) = table.get("path") {
        let Some(path) = path.as_str() else {
            return Err(format!("`[{section_key}].{alias}.path` must be a string."));
        };
        if path.is_empty() {
            return Err(format!("`[{section_key}].{alias}.path` must be non-empty."));
        }
    }

    if let Some(workspace) = table.get("workspace") {
        if workspace.as_bool().is_none() {
            return Err(format!(
                "`[{section_key}].{alias}.workspace` must be a boolean."
            ));
        }
    }

    Ok(())
}

fn discover_workspaces(
    tree: &ProjectTree,
    route: &RsDepsRoute,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<WorkspaceFacts> {
    let mut roots = route
        .roots
        .iter()
        .map(|root| root.rel_dir.clone())
        .collect::<Vec<_>>();
    roots.sort();
    roots.dedup();

    let mut workspaces = Vec::new();
    for root_rel_dir in roots {
        let cargo_rel_path = if root_rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{root_rel_dir}/Cargo.toml")
        };
        let Some(content) = tree.file_content(&cargo_rel_path) else {
            continue;
        };
        let parsed = match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => parsed,
            Err(parse_error) => {
                input_failures.push(InputFailureFacts {
                    rel_path: cargo_rel_path.clone(),
                    message: format!("Failed to parse workspace Cargo.toml: {parse_error}"),
                });
                continue;
            }
        };
        if let Err(parse_error) = validate_workspace_manifest_shape(&parsed) {
            input_failures.push(InputFailureFacts {
                rel_path: cargo_rel_path.clone(),
                message: format!("Failed to parse workspace Cargo.toml: {parse_error}"),
            });
            continue;
        }
        let Some(workspace) = parsed.get("workspace") else {
            continue;
        };
        let raw_members = workspace
            .get("members")
            .and_then(toml::Value::as_array)
            .map(|members| {
                members
                    .iter()
                    .filter_map(toml::Value::as_str)
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let mut member_dirs = BTreeSet::new();
        for member in &raw_members {
            for member_dir in resolve_member_pattern(tree, &root_rel_dir, member) {
                if tree.file_exists(&format!("{member_dir}/Cargo.toml")) {
                    let _ = member_dirs.insert(member_dir);
                } else {
                    input_failures.push(InputFailureFacts {
                        rel_path: member_dir.clone(),
                        message: format!(
                            "Workspace member `{member_dir}` matched `{member}` but has no Cargo.toml."
                        ),
                    });
                }
            }
        }
        let workspace_dependencies = workspace
            .get("dependencies")
            .and_then(toml::Value::as_table)
            .cloned()
            .unwrap_or_default();
        let mut workspace_package_dirs = member_dirs.clone();
        if parsed.get("package").is_some() {
            let _ = workspace_package_dirs.insert(root_rel_dir.clone());
        }
        workspaces.push(WorkspaceFacts {
            root_rel_dir,
            cargo_rel_path,
            workspace_dependencies,
            workspace_package_dirs,
            member_dirs: member_dirs.into_iter().collect(),
        });
    }
    workspaces
}

fn resolve_member_pattern(
    tree: &ProjectTree,
    workspace_root_rel_dir: &str,
    member: &str,
) -> Vec<String> {
    let pattern = if workspace_root_rel_dir.is_empty() {
        member.to_owned()
    } else {
        format!("{workspace_root_rel_dir}/{member}")
    };

    let mut matches = tree.matching_dir_rels(&pattern);
    if matches.is_empty() && tree.dir_exists(&pattern) {
        matches.push(pattern);
    }
    matches.sort();
    matches.dedup();
    matches
}

fn workspace_by_member(workspaces: &[WorkspaceFacts]) -> BTreeMap<String, String> {
    let mut mapping = BTreeMap::new();
    let mut ordered = workspaces.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|workspace| std::cmp::Reverse(workspace.root_rel_dir.len()));
    for workspace in ordered {
        for member in &workspace.member_dirs {
            let _ = mapping
                .entry(member.clone())
                .or_insert_with(|| workspace.root_rel_dir.clone());
        }
    }
    mapping
}

fn discover_members(
    tree: &ProjectTree,
    routed_root_rels: &BTreeSet<String>,
    workspaces: &[WorkspaceFacts],
    workspace_by_member: &BTreeMap<String, String>,
    parsed_guardrail: &Option<ParsedGuardrail>,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<MemberFacts> {
    let mut member_dirs = workspaces
        .iter()
        .flat_map(|workspace| workspace.member_dirs.iter().cloned())
        .collect::<BTreeSet<_>>();

    for root_rel_dir in routed_root_rels.iter().cloned() {
        if workspace_by_member.contains_key(&root_rel_dir) {
            continue;
        }
        let cargo_rel_path = if root_rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{root_rel_dir}/Cargo.toml")
        };
        let Some(content) = tree.file_content(&cargo_rel_path) else {
            if tree.file_exists(&cargo_rel_path) {
                input_failures.push(InputFailureFacts {
                    rel_path: cargo_rel_path.clone(),
                    message: "Failed to read Cargo.toml for dependency root discovery.".to_owned(),
                });
            }
            continue;
        };
        let parsed = match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => parsed,
            Err(parse_error) => {
                input_failures.push(InputFailureFacts {
                    rel_path: cargo_rel_path.clone(),
                    message: format!(
                        "Failed to parse Cargo.toml for dependency root discovery: {parse_error}"
                    ),
                });
                continue;
            }
        };
        if parsed.get("package").is_some() {
            let _ = member_dirs.insert(root_rel_dir);
        }
    }

    let mut members = Vec::new();
    for rel_dir in member_dirs {
        let cargo_rel_path = if rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{rel_dir}/Cargo.toml")
        };
        let crate_name = tree
            .file_content(&cargo_rel_path)
            .and_then(|content| toml::from_str::<toml::Value>(content).ok())
            .and_then(|parsed| {
                parsed
                    .get("package")
                    .and_then(|value| value.get("name"))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| {
                if rel_dir.is_empty() {
                    "root".to_owned()
                } else {
                    rel_dir.rsplit('/').next().unwrap_or(&rel_dir).to_owned()
                }
            });
        let (profile_name, allowed_deps) = policy_for_member(&rel_dir, parsed_guardrail.as_ref());

        members.push(MemberFacts {
            crate_name,
            rel_dir: rel_dir.clone(),
            cargo_rel_path,
            workspace_root_rel_dir: {
                if let Some(workspace_root) = workspace_by_member.get(&rel_dir) {
                    Some(workspace_root.clone())
                } else if workspaces
                    .iter()
                    .any(|workspace| workspace.root_rel_dir == rel_dir)
                {
                    Some(rel_dir.clone())
                } else {
                    None
                }
            },
            profile_name,
            allowed_deps,
        });
    }
    members.sort_by(|left, right| left.rel_dir.cmp(&right.rel_dir));
    members
}

fn policy_for_member(
    rel_dir: &str,
    parsed_guardrail: Option<&ParsedGuardrail>,
) -> (Option<String>, Option<BTreeSet<String>>) {
    let Some(parsed_guardrail) = parsed_guardrail else {
        return (None, None);
    };

    if rel_dir.starts_with("apps/") {
        if let Some(app_name) = rel_dir.split('/').nth(1) {
            if let Some(config) = parsed_guardrail.apps.get(app_name) {
                return (
                    config.profile.clone().or(config.type_.clone()),
                    config
                        .allowed_deps
                        .clone()
                        .map(|deps| deps.into_iter().collect::<BTreeSet<_>>()),
                );
            }
        }
    }

    if rel_dir.starts_with("packages/") {
        if let Some(config) = &parsed_guardrail.packages {
            return (
                config.profile.clone().or(config.type_.clone()),
                config
                    .allowed_deps
                    .clone()
                    .map(|deps| deps.into_iter().collect::<BTreeSet<_>>()),
            );
        }
    }

    (parsed_guardrail.root_profile_name.clone(), None)
}

fn collect_dependency_entries(
    tree: &ProjectTree,
    members: &[MemberFacts],
    workspaces: &[WorkspaceFacts],
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<DependencyEntryFacts> {
    let workspaces_by_root = workspaces
        .iter()
        .map(|workspace| (workspace.root_rel_dir.clone(), workspace))
        .collect::<BTreeMap<_, _>>();
    let mut entries = Vec::new();

    for member in members {
        let Some(content) = tree.file_content(&member.cargo_rel_path) else {
            input_failures.push(InputFailureFacts {
                rel_path: member.cargo_rel_path.clone(),
                message: "Missing Cargo.toml content for dependency policy check.".to_owned(),
            });
            continue;
        };
        let parsed = match toml::from_str::<toml::Value>(content) {
            Ok(parsed) => parsed,
            Err(parse_error) => {
                input_failures.push(InputFailureFacts {
                    rel_path: member.cargo_rel_path.clone(),
                    message: format!(
                        "Failed to parse Cargo.toml for dependency policy check: {parse_error}"
                    ),
                });
                continue;
            }
        };
        if let Err(parse_error) = validate_dependency_manifest_shape(&parsed) {
            input_failures.push(InputFailureFacts {
                rel_path: member.cargo_rel_path.clone(),
                message: format!(
                    "Failed to parse Cargo.toml for dependency policy check: {parse_error}"
                ),
            });
            continue;
        }

        for (section_key, section_kind) in [
            ("dependencies", DependencySectionKind::Dependencies),
            (
                "build-dependencies",
                DependencySectionKind::BuildDependencies,
            ),
            ("dev-dependencies", DependencySectionKind::DevDependencies),
        ] {
            let Some(table) = parsed.get(section_key).and_then(toml::Value::as_table) else {
                continue;
            };
            for (alias, value) in table {
                match external_dep_name(member, alias, value, &workspaces_by_root) {
                    Ok(Some(dep_package_name)) => entries.push(DependencyEntryFacts {
                        crate_name: member.crate_name.clone(),
                        cargo_rel_path: member.cargo_rel_path.clone(),
                        section_kind,
                        dep_alias: alias.clone(),
                        allowlisted: member
                            .allowed_deps
                            .as_ref()
                            .is_some_and(|allowed| allowed.contains(&dep_package_name)),
                        allowlist_present: member.allowed_deps.is_some(),
                        dep_package_name,
                    }),
                    Ok(None) => {}
                    Err(message) => input_failures.push(InputFailureFacts {
                        rel_path: member.cargo_rel_path.clone(),
                        message,
                    }),
                }
            }
        }
    }

    entries
}

fn collect_lockfiles(
    tree: &ProjectTree,
    workspaces: &[WorkspaceFacts],
    members: &[MemberFacts],
    parsed_guardrail: Option<&ParsedGuardrail>,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<LockfileFacts> {
    let mut root_profiles = BTreeMap::new();
    let mut reported_gitignore_failures = BTreeSet::new();
    for member in members {
        let _ = root_profiles
            .entry(member.rel_dir.clone())
            .or_insert_with(|| member.profile_name.clone());
    }

    let mut root_rels = BTreeSet::new();
    for workspace in workspaces {
        let _ = root_rels.insert(workspace.root_rel_dir.clone());
        let _ = root_profiles
            .entry(workspace.root_rel_dir.clone())
            .or_insert_with(|| policy_for_member(&workspace.root_rel_dir, parsed_guardrail).0);
    }
    for member in members {
        if member.workspace_root_rel_dir.is_none() {
            let _ = root_rels.insert(member.rel_dir.clone());
        }
    }

    root_rels
        .into_iter()
        .map(|root_rel_dir| {
            let cargo_lock_rel_path = if root_rel_dir.is_empty() {
                "Cargo.lock".to_owned()
            } else {
                format!("{root_rel_dir}/Cargo.lock")
            };
            let (cargo_lock_ignored, gitignore_rel_path) = lockfile_ignore_status(
                tree,
                &root_rel_dir,
                &cargo_lock_rel_path,
                input_failures,
                &mut reported_gitignore_failures,
            );
            LockfileFacts {
                root_rel_dir: root_rel_dir.clone(),
                cargo_lock_rel_path: cargo_lock_rel_path.clone(),
                cargo_lock_exists: tree.file_exists(&cargo_lock_rel_path),
                cargo_lock_ignored,
                gitignore_rel_path,
                profile_name: root_profiles.get(&root_rel_dir).cloned().flatten(),
            }
        })
        .collect()
}

fn external_dep_name(
    member: &MemberFacts,
    alias: &str,
    value: &toml::Value,
    workspaces_by_root: &BTreeMap<String, &WorkspaceFacts>,
) -> Result<Option<String>, String> {
    let dep_table = value.as_table();
    let package_name = dep_table
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_str)
        .unwrap_or(alias)
        .to_owned();

    let uses_workspace = dep_table
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        == Some(true);

    if uses_workspace {
        let workspace_root = member
            .workspace_root_rel_dir
            .as_ref()
            .ok_or_else(|| format!("`{alias}` uses `workspace = true` but `{}` is not associated with a workspace root.", member.rel_dir))?;
        let workspace = workspaces_by_root.get(workspace_root).ok_or_else(|| {
            format!(
                "Workspace dependency resolution failed for `{alias}` in `{}`: missing workspace facts.",
                member.rel_dir
            )
        })?;
        let workspace_value = workspace.workspace_dependencies.get(alias).ok_or_else(|| {
            format!(
                "`{alias}` uses `workspace = true` in `{}`, but `[workspace.dependencies].{alias}` is missing in `{}`.",
                member.cargo_rel_path, workspace.cargo_rel_path
            )
        })?;
        let workspace_package = workspace_value
            .as_table()
            .and_then(|table| table.get("package"))
            .and_then(toml::Value::as_str)
            .unwrap_or(alias)
            .to_owned();
        if let Some(dep_path) = workspace_value
            .as_table()
            .and_then(|table| table.get("path"))
            .and_then(toml::Value::as_str)
        {
            if is_workspace_package_path(&workspace.root_rel_dir, dep_path, workspace) {
                return Ok(None);
            }
        }
        return Ok(Some(workspace_package));
    }

    if let Some(dep_path) = dep_table
        .and_then(|table| table.get("path"))
        .and_then(toml::Value::as_str)
    {
        if let Some(workspace_root) = &member.workspace_root_rel_dir {
            if let Some(workspace) = workspaces_by_root.get(workspace_root) {
                if is_workspace_package_path(&member.rel_dir, dep_path, workspace) {
                    return Ok(None);
                }
            }
        }
    }

    Ok(Some(package_name))
}

fn is_workspace_package_path(
    base_rel_dir: &str,
    dep_path: &str,
    workspace: &WorkspaceFacts,
) -> bool {
    let resolved = normalize_rel_path(base_rel_dir, dep_path);
    workspace.workspace_package_dirs.contains(&resolved)
}

fn normalize_rel_path(base_rel_dir: &str, dep_path: &str) -> String {
    let joined = if base_rel_dir.is_empty() {
        Path::new(dep_path).to_path_buf()
    } else {
        Path::new(base_rel_dir).join(dep_path)
    };
    let mut parts = Vec::new();

    for component in joined.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::ParentDir => {
                if parts.last().is_some_and(|last| last != "..") {
                    let _ = parts.pop();
                } else {
                    parts.push("..".to_owned());
                }
            }
            Component::RootDir | Component::Prefix(_) => {}
        }
    }

    parts.join("/")
}

fn lockfile_ignore_status(
    tree: &ProjectTree,
    root_rel_dir: &str,
    cargo_lock_rel_path: &str,
    input_failures: &mut Vec<InputFailureFacts>,
    reported_gitignore_failures: &mut BTreeSet<String>,
) -> (bool, Option<String>) {
    let mut ignored = false;
    let mut source = None;

    for gitignore_rel_path in ancestor_gitignore_rels(root_rel_dir) {
        let Some(content) = tree.file_content(&gitignore_rel_path) else {
            if tree.file_exists(&gitignore_rel_path)
                && reported_gitignore_failures.insert(gitignore_rel_path.clone())
            {
                input_failures.push(InputFailureFacts {
                    rel_path: gitignore_rel_path.clone(),
                    message: "Failed to read `.gitignore` for Cargo.lock masking checks."
                        .to_owned(),
                });
            }
            continue;
        };
        for line in content.lines() {
            if let Some(next_ignored) =
                cargo_lock_ignore_match(line, &gitignore_rel_path, cargo_lock_rel_path)
            {
                ignored = next_ignored;
                source = if ignored {
                    Some(gitignore_rel_path.clone())
                } else {
                    None
                };
            }
        }
    }

    (ignored, source)
}

fn ancestor_gitignore_rels(root_rel_dir: &str) -> Vec<String> {
    let mut rels = vec![".gitignore".to_owned()];
    if root_rel_dir.is_empty() {
        return rels;
    }

    let mut current = String::new();
    for segment in root_rel_dir.split('/') {
        current = if current.is_empty() {
            segment.to_owned()
        } else {
            format!("{current}/{segment}")
        };
        rels.push(format!("{current}/.gitignore"));
    }
    rels
}

fn cargo_lock_ignore_match(
    line: &str,
    gitignore_rel_path: &str,
    cargo_lock_rel_path: &str,
) -> Option<bool> {
    let gitignore_dir_rel = gitignore_rel_path
        .strip_suffix("/.gitignore")
        .unwrap_or_default();
    let candidate_rel = if gitignore_dir_rel.is_empty() {
        cargo_lock_rel_path.to_owned()
    } else if let Some(rest) = cargo_lock_rel_path.strip_prefix(&format!("{gitignore_dir_rel}/")) {
        rest.to_owned()
    } else {
        return None;
    };

    let basename = "Cargo.lock";
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let (ignored, pattern_text) = if let Some(pattern) = trimmed.strip_prefix('!') {
        (false, pattern)
    } else {
        (true, trimmed)
    };
    let anchored = pattern_text.starts_with('/');
    let normalized = pattern_text.trim_start_matches('/');
    if normalized.is_empty() {
        return None;
    }

    let matched = if normalized == "Cargo.lock" {
        if anchored {
            candidate_rel == basename
        } else {
            true
        }
    } else if !normalized.contains('/') {
        glob::Pattern::new(normalized).ok().is_some_and(|pattern| {
            if anchored {
                pattern.matches(&candidate_rel)
            } else {
                pattern.matches(basename)
            }
        })
    } else {
        glob::Pattern::new(normalized)
            .ok()
            .is_some_and(|pattern| pattern.matches(&candidate_rel))
    };

    matched.then_some(ignored)
}
