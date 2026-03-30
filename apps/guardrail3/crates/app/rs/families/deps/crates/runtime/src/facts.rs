mod dependency_entries;
mod guardrail;
mod lockfiles;
mod workspaces;

use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_family_mapper::RsDepsRoute;
use guardrail3_domain_config::types::CrateConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_outbound_traits::ToolChecker;

use self::dependency_entries::{collect_dependency_entries, discover_members};
use self::guardrail::parse_guardrail;
use self::lockfiles::collect_lockfiles;
use self::workspaces::{discover_workspaces, workspace_by_member};

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
