mod dependency_entries;
mod guardrail;
mod lockfiles;
mod workspaces;

use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsDepsRoute;
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
        .roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
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
