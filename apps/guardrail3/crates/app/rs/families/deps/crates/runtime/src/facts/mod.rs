mod dependency_entries;
mod guardrail;
mod lockfiles;
mod workspaces;

use std::collections::{BTreeMap, BTreeSet};

use cargo_toml_parser::CargoToml;
use guardrail3_app_rs_family_mapper::RsDepsRoute;
use guardrail3_app_rs_ownership::RustFamilyFileKind;
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_outbound_traits::ToolChecker;

use self::dependency_entries::{collect_content_check_facts, collect_dependency_facts, discover_members};
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct AllowlistCoverageFacts {
    pub(crate) crate_name: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) has_allowlist: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DirectDependencyCapFacts {
    pub(crate) crate_name: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) unique_direct_dependency_count: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LocalPathCargoManifestFacts {
    pub(crate) cargo_rel_path: String,
    pub(crate) cargo: CargoToml,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PolicyContentCheckFacts {
    pub(crate) workspace_cargo_rel_path: String,
    pub(crate) workspace_cargo: CargoToml,
    pub(crate) crate_cargo_rel_path: String,
    pub(crate) crate_cargo: CargoToml,
    pub(crate) guardrail_rel_path: String,
    pub(crate) guardrail_content: String,
    pub(crate) local_path_cargo_rel_paths: Vec<String>,
    pub(crate) local_path_cargo_manifests: Vec<LocalPathCargoManifestFacts>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DirectDependencyCapContentFacts {
    pub(crate) workspace_cargo_rel_path: String,
    pub(crate) workspace_cargo: CargoToml,
    pub(crate) crate_cargo_rel_path: String,
    pub(crate) crate_cargo: CargoToml,
    pub(crate) local_path_cargo_rel_paths: Vec<String>,
    pub(crate) local_path_cargo_manifests: Vec<LocalPathCargoManifestFacts>,
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
    #[allow(dead_code)]
    pub(crate) dependency_entries: Vec<DependencyEntryFacts>,
    #[allow(dead_code)]
    pub(crate) allowlist_coverage: Vec<AllowlistCoverageFacts>,
    #[allow(dead_code)]
    pub(crate) direct_dependency_caps: Vec<DirectDependencyCapFacts>,
    pub(crate) policy_content_checks: Vec<PolicyContentCheckFacts>,
    #[allow(dead_code)]
    pub(crate) direct_dependency_cap_content_checks: Vec<DirectDependencyCapContentFacts>,
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
        .filter(|file| file.kind() == RustFamilyFileKind::CargoToml && file.exact_rust_root_owner())
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
                    || file.ancestor_rust_root_rels().is_some_and(|roots| {
                        roots
                            .iter()
                            .any(|root| routed_workspace_roots.contains(root))
                    }))
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
    let workspace_by_member = workspace_by_member(&workspaces);
    let members = discover_members(tree, &workspaces, &workspace_by_member, &parsed_guardrail)
        .into_iter()
        .filter(|member| {
            route
                .validation_scope()
                .is_none_or(|scope| rel_intersects_validation_scope(&member.rel_dir, scope))
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
    let (policy_content_checks, direct_dependency_cap_content_checks) = collect_content_check_facts(
        tree,
        &members,
        &workspaces,
        guardrail_rel_path.as_deref(),
        parsed_guardrail.as_ref(),
        &mut input_failures,
    );
    let allowlist_coverage = members
        .into_iter()
        .map(|member| AllowlistCoverageFacts {
            crate_name: member.crate_name,
            cargo_rel_path: member.cargo_rel_path,
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
        policy_content_checks,
        direct_dependency_cap_content_checks,
        input_failures,
    }
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
