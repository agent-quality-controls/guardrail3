mod dependency_entries;
mod guardrail;
mod lockfiles;
mod workspaces;

use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsDepsRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::ToolChecker;

use self::dependency_entries::{collect_dependency_facts, discover_members};
use self::guardrail::parse_guardrail;
use self::lockfiles::collect_lockfiles;
use self::workspaces::{discover_workspaces, workspace_by_member};

#[derive(Debug, Clone)]
pub struct ToolFacts {
    pub(crate) tool_name: String,
    pub(crate) installed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LockfileFacts {
    pub(crate) root_rel_dir: String,
    pub(crate) cargo_lock_rel_path: String,
    pub(crate) cargo_lock_exists: bool,
    pub(crate) cargo_lock_ignored: bool,
    pub(crate) gitignore_rel_path: Option<String>,
    pub(crate) profile_name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencySectionKind {
    Dependencies,
    BuildDependencies,
    DevDependencies,
}

#[derive(Debug, Clone)]
pub struct DependencyEntryFacts {
    pub(crate) crate_name: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) section_kind: DependencySectionKind,
    pub(crate) table_label: String,
    pub(crate) dep_package_name: String,
    pub(crate) allowlist_present: bool,
    pub(crate) allowlisted: bool,
}

#[derive(Debug, Clone)]
pub struct AllowlistCoverageFacts {
    pub(crate) crate_name: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) profile_name: Option<String>,
    pub(crate) has_allowlist: bool,
}

#[derive(Debug, Clone)]
pub struct DirectDependencyCapFacts {
    pub(crate) crate_name: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) unique_direct_dependency_count: usize,
}

#[derive(Debug, Clone)]
pub struct InputFailureFacts {
    pub(crate) rel_path: String,
    pub(crate) message: String,
}

#[derive(Debug, Clone, Default)]
pub struct DepsFacts {
    pub(crate) tools: Vec<ToolFacts>,
    pub(crate) lockfiles: Vec<LockfileFacts>,
    pub(crate) dependency_entries: Vec<DependencyEntryFacts>,
    pub(crate) allowlist_coverage: Vec<AllowlistCoverageFacts>,
    pub(crate) direct_dependency_caps: Vec<DirectDependencyCapFacts>,
    pub(crate) input_failures: Vec<InputFailureFacts>,
}

#[derive(Debug, Clone)]
struct DepsCratePolicy {
    profile_name: Option<String>,
    type_name: Option<String>,
    allowed_deps: Option<BTreeSet<String>>,
}

pub fn collect(tree: &ProjectTree, route: &RsDepsRoute, tc: &dyn ToolChecker) -> DepsFacts {
    let exact_root_cargo_dirs = route
        .family_files()
        .iter()
        .filter(|file| {
            file.kind() == RustFamilyFileKind::CargoToml && file.exact_rust_root_owner()
        })
        .map(|file| file.logical_owner_rel().to_owned())
        .collect::<BTreeSet<_>>();
    let routed_workspace_roots = route
        .roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
        .collect::<BTreeSet<_>>();
    let guardrail_rel_path = route
        .family_files()
        .iter()
        .find(|file| {
            file.kind() == RustFamilyFileKind::GuardrailToml
                && (routed_workspace_roots.is_empty()
                    || routed_workspace_roots.contains(file.logical_owner_rel())
                    || file
                        .ancestor_rust_root_rels()
                        .is_some_and(|roots| roots.iter().any(|root| routed_workspace_roots.contains(root))))
        })
        .map(|file| file.rel_path().to_owned());
    let parsed_guardrail = parse_guardrail(tree, guardrail_rel_path.as_deref());
    let mut input_failures = parsed_guardrail
        .as_ref()
        .and_then(|guardrail| guardrail.parse_error.clone())
        .map(|message| {
            vec![InputFailureFacts {
                rel_path: guardrail_rel_path
                    .clone()
                    .unwrap_or_else(|| "guardrail3.toml".to_owned()),
                message,
            }]
        })
        .unwrap_or_default();
    let workspaces = discover_workspaces(tree, route, &exact_root_cargo_dirs, &mut input_failures);
    if !routed_workspace_roots.is_empty() {
        collect_guardrail_placement_failures(route, &routed_workspace_roots, &mut input_failures);
        collect_cargo_placement_failures(&exact_root_cargo_dirs, &workspaces, &mut input_failures);
    }
    let workspace_by_member = workspace_by_member(&workspaces);
    let members = discover_members(
        tree,
        &workspaces,
        &workspace_by_member,
        &parsed_guardrail,
    )
    .into_iter()
    .filter(|member| {
        route.validation_scope().is_none_or(|scope| {
            rel_intersects_validation_scope(&member.rel_dir, scope)
        })
    })
    .collect::<Vec<_>>();

    let (dependency_entries, direct_dependency_caps) =
        collect_dependency_facts(tree, &members, &workspaces, &mut input_failures);
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
    let mut seen_input_failures = BTreeSet::new();
    input_failures.retain(|failure| {
        seen_input_failures.insert((failure.rel_path.clone(), failure.message.clone()))
    });

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
        direct_dependency_caps,
        input_failures,
    }
}

fn collect_guardrail_placement_failures(
    route: &RsDepsRoute,
    routed_workspace_roots: &BTreeSet<String>,
    input_failures: &mut Vec<InputFailureFacts>,
) {
    for file in route
        .family_files()
        .iter()
        .filter(|file| file.kind() == RustFamilyFileKind::GuardrailToml)
    {
        let allowed_exact_root =
            file.exact_rust_root_owner() && routed_workspace_roots.contains(file.logical_owner_rel());
        let allowed_ancestor = file
            .ancestor_rust_root_rels()
            .is_some_and(|roots| roots.iter().any(|root| routed_workspace_roots.contains(root)));
        if allowed_exact_root || allowed_ancestor {
            continue;
        }

        let message = if routed_workspace_roots
            .iter()
            .any(|root_rel| rel_is_nested_beneath(file.logical_owner_rel(), root_rel))
        {
            "guardrail3.toml is nested under a workspace root but does not belong to the workspace root. Deps policy files are only allowed at workspace roots."
        } else {
            "guardrail3.toml sits outside every legal workspace root. Deps policy files are only allowed at workspace roots."
        };
        input_failures.push(InputFailureFacts {
            rel_path: file.rel_path().to_owned(),
            message: message.to_owned(),
        });
    }
}

fn collect_cargo_placement_failures(
    exact_root_cargo_dirs: &BTreeSet<String>,
    workspaces: &[WorkspaceFacts],
    input_failures: &mut Vec<InputFailureFacts>,
) {
    let legal_workspace_roots = workspaces
        .iter()
        .map(|workspace| workspace.root_rel_dir.as_str())
        .collect::<BTreeSet<_>>();
    let legal_package_dirs = workspaces
        .iter()
        .flat_map(|workspace| workspace.workspace_package_dirs.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();

    for rel_dir in exact_root_cargo_dirs {
        if legal_workspace_roots.contains(rel_dir.as_str()) || legal_package_dirs.contains(rel_dir.as_str()) {
            continue;
        }

        let cargo_rel_path = if rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{rel_dir}/Cargo.toml")
        };
        let message = if workspaces
            .iter()
            .any(|workspace| rel_is_nested_beneath(rel_dir, &workspace.root_rel_dir))
        {
            "Cargo.toml is nested under a workspace root but is not declared as a workspace package. Deps checks do not allow loose crates inside governed workspaces."
        } else {
            "Cargo.toml sits outside every legal workspace root. Deps checks require Rust crates to belong to a legal workspace."
        };
        input_failures.push(InputFailureFacts {
            rel_path: cargo_rel_path,
            message: message.to_owned(),
        });
    }
}

fn rel_is_nested_beneath(rel_dir: &str, parent_rel: &str) -> bool {
    if parent_rel.is_empty() {
        return !rel_dir.is_empty();
    }

    rel_dir
        .strip_prefix(parent_rel)
        .is_some_and(|rest| rest.starts_with('/'))
}

fn rel_intersects_validation_scope(rel_dir: &str, validation_scope: &str) -> bool {
    if validation_scope.is_empty() || rel_dir.is_empty() {
        return true;
    }

    rel_dir == validation_scope
        || rel_dir
            .strip_prefix(validation_scope)
            .is_some_and(|rest| rest.starts_with('/'))
        || validation_scope
            .strip_prefix(rel_dir)
            .is_some_and(|rest| rest.starts_with('/'))
}

#[derive(Debug, Clone)]
struct ParsedGuardrail {
    root_profile_name: Option<String>,
    apps: BTreeMap<String, DepsCratePolicy>,
    packages: Option<DepsCratePolicy>,
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
