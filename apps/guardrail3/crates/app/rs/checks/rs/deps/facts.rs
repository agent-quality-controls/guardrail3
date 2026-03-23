use std::collections::{BTreeMap, BTreeSet};

use crate::domain::config::types::{CrateConfig, GuardrailConfig};
use crate::domain::project_tree::ProjectTree;
use crate::ports::outbound::ToolChecker;

#[derive(Debug, Clone)]
pub struct ToolFacts {
    pub tool_name: String,
    pub installed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LockfileFacts {
    pub cargo_lock_exists: bool,
    pub cargo_lock_ignored: bool,
    pub root_profile_name: Option<String>,
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
    pub lockfile: LockfileFacts,
    pub dependency_entries: Vec<DependencyEntryFacts>,
    pub allowlist_coverage: Vec<AllowlistCoverageFacts>,
    pub input_failures: Vec<InputFailureFacts>,
}

pub fn collect(tree: &ProjectTree, tc: &dyn ToolChecker) -> DepsFacts {
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
    let workspaces = discover_workspaces(tree, &mut input_failures);
    let workspace_by_member = workspace_by_member(&workspaces);
    let members = discover_members(
        tree,
        &workspaces,
        &workspace_by_member,
        &parsed_guardrail,
        &mut input_failures,
    );

    let dependency_entries =
        collect_dependency_entries(tree, &members, &workspaces, &mut input_failures);
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
        lockfile: LockfileFacts {
            cargo_lock_exists: tree.file_exists("Cargo.lock"),
            cargo_lock_ignored: cargo_lock_is_ignored(tree.file_content(".gitignore")),
            root_profile_name: parsed_guardrail.and_then(|guardrail| guardrail.root_profile_name),
        },
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
    let content = tree.file_content("guardrail3.toml")?;
    match toml::from_str::<GuardrailConfig>(content) {
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
    }
}

fn discover_workspaces(
    tree: &ProjectTree,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<WorkspaceFacts> {
    let mut roots = Vec::new();
    if tree.file_exists("Cargo.toml") {
        roots.push(String::new());
    }
    roots.extend(tree.dirs_with_file("Cargo.toml"));
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
        workspaces.push(WorkspaceFacts {
            root_rel_dir,
            cargo_rel_path,
            workspace_dependencies,
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
    workspaces: &[WorkspaceFacts],
    workspace_by_member: &BTreeMap<String, String>,
    parsed_guardrail: &Option<ParsedGuardrail>,
    input_failures: &mut Vec<InputFailureFacts>,
) -> Vec<MemberFacts> {
    let mut member_dirs = workspaces
        .iter()
        .flat_map(|workspace| workspace.member_dirs.iter().cloned())
        .collect::<BTreeSet<_>>();

    for root_rel_dir in std::iter::once(String::new()).chain(tree.dirs_with_file("Cargo.toml")) {
        if workspace_by_member.contains_key(&root_rel_dir) {
            continue;
        }
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
            workspace_root_rel_dir: workspace_by_member.get(&rel_dir).cloned(),
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

fn external_dep_name(
    member: &MemberFacts,
    alias: &str,
    value: &toml::Value,
    workspaces_by_root: &BTreeMap<String, &WorkspaceFacts>,
) -> Result<Option<String>, String> {
    let dep_table = value.as_table();
    if dep_table.is_some_and(|table| table.contains_key("path")) {
        return Ok(None);
    }

    let package_name = dep_table
        .and_then(|table| table.get("package"))
        .and_then(toml::Value::as_str)
        .unwrap_or(alias)
        .to_owned();

    if dep_table
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        == Some(true)
    {
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
        if workspace_value
            .as_table()
            .is_some_and(|table| table.contains_key("path"))
        {
            return Ok(None);
        }
        let workspace_package = workspace_value
            .as_table()
            .and_then(|table| table.get("package"))
            .and_then(toml::Value::as_str)
            .unwrap_or(alias)
            .to_owned();
        return Ok(Some(workspace_package));
    }

    Ok(Some(package_name))
}

fn cargo_lock_is_ignored(gitignore: Option<&str>) -> bool {
    gitignore
        .map(|gitignore| {
            gitignore.lines().any(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('!') {
                    return false;
                }

                let normalized = trimmed.trim_start_matches('/');
                if normalized == "Cargo.lock" {
                    return true;
                }

                glob::Pattern::new(normalized)
                    .ok()
                    .is_some_and(|pattern| pattern.matches("Cargo.lock"))
            })
        })
        .unwrap_or(false)
}
