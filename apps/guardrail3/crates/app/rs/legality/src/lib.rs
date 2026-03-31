use std::collections::{BTreeMap, BTreeSet};

use guardrail3_app_rs_ownership::{
    RustFamilyFileAttachment, RustFamilyFileFact, RustFamilyFileKind, RustOwnedSurfaceFacts,
};
use guardrail3_app_rs_placement::{RustRootClassification, RustRootPlacementFacts};
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_validation_model::RustValidateFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustLegalWorkspaceRoot {
    rel_dir: String,
    cargo_rel_path: String,
    classification: RustRootClassification,
}

impl RustLegalWorkspaceRoot {
    #[must_use]
    pub fn new(rel_dir: String, cargo_rel_path: String, classification: RustRootClassification) -> Self {
        Self {
            rel_dir,
            cargo_rel_path,
            classification,
        }
    }

    #[must_use]
    pub fn rel_dir(&self) -> &str {
        &self.rel_dir
    }

    #[must_use]
    pub fn cargo_rel_path(&self) -> &str {
        &self.cargo_rel_path
    }

    #[must_use]
    pub const fn classification(&self) -> RustRootClassification {
        self.classification
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustTopologyIssueKind {
    TopLevelRootMustBeWorkspace,
    LooseTopLevelPackage,
    NestedWorkspace {
        parent_workspace_rel: String,
    },
    UndeclaredWorkspaceMember {
        workspace_root_rel: String,
    },
    WorkspaceMemberPathEscapesRoot {
        workspace_root_rel: String,
        member_pattern: String,
    },
    AuxiliaryTopLevelRootMustBeWorkspace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustTopologyIssueFact {
    rel_dir: String,
    cargo_rel_path: String,
    classification: RustRootClassification,
    kind: RustTopologyIssueKind,
}

impl RustTopologyIssueFact {
    #[must_use]
    pub fn new(
        rel_dir: String,
        cargo_rel_path: String,
        classification: RustRootClassification,
        kind: RustTopologyIssueKind,
    ) -> Self {
        Self {
            rel_dir,
            cargo_rel_path,
            classification,
            kind,
        }
    }

    #[must_use]
    pub fn rel_dir(&self) -> &str {
        &self.rel_dir
    }

    #[must_use]
    pub fn cargo_rel_path(&self) -> &str {
        &self.cargo_rel_path
    }

    #[must_use]
    pub const fn classification(&self) -> RustRootClassification {
        self.classification
    }

    #[must_use]
    pub fn kind(&self) -> &RustTopologyIssueKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustLegalFamilyFileFact {
    family: RustValidateFamily,
    rel_path: String,
    kind: RustFamilyFileKind,
    attachment: RustFamilyFileAttachment,
    workspace_root_rel: String,
}

impl RustLegalFamilyFileFact {
    #[must_use]
    pub fn new(
        family: RustValidateFamily,
        rel_path: String,
        kind: RustFamilyFileKind,
        attachment: RustFamilyFileAttachment,
        workspace_root_rel: String,
    ) -> Self {
        Self {
            family,
            rel_path,
            kind,
            attachment,
            workspace_root_rel,
        }
    }

    #[must_use]
    pub const fn family(&self) -> RustValidateFamily {
        self.family
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub const fn kind(&self) -> RustFamilyFileKind {
        self.kind
    }

    #[must_use]
    pub fn attachment(&self) -> &RustFamilyFileAttachment {
        &self.attachment
    }

    #[must_use]
    pub fn workspace_root_rel(&self) -> &str {
        &self.workspace_root_rel
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustIllegalFamilyFileReason {
    OutsideEveryLegalWorkspace,
    AboveLegalWorkspaceRoots {
        workspace_root_rels: Vec<String>,
    },
    NestedBeneathLegalWorkspace {
        workspace_root_rel: String,
        owner_rel: String,
    },
    AttachedToIllegalRoot {
        root_rel: String,
    },
    AttachedToLegalMemberRoot {
        workspace_root_rel: String,
        member_rel: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustIllegalFamilyFileFact {
    family: RustValidateFamily,
    rel_path: String,
    kind: RustFamilyFileKind,
    attachment: RustFamilyFileAttachment,
    reason: RustIllegalFamilyFileReason,
}

impl RustIllegalFamilyFileFact {
    #[must_use]
    pub fn new(
        family: RustValidateFamily,
        rel_path: String,
        kind: RustFamilyFileKind,
        attachment: RustFamilyFileAttachment,
        reason: RustIllegalFamilyFileReason,
    ) -> Self {
        Self {
            family,
            rel_path,
            kind,
            attachment,
            reason,
        }
    }

    #[must_use]
    pub const fn family(&self) -> RustValidateFamily {
        self.family
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub const fn kind(&self) -> RustFamilyFileKind {
        self.kind
    }

    #[must_use]
    pub fn attachment(&self) -> &RustFamilyFileAttachment {
        &self.attachment
    }

    #[must_use]
    pub fn reason(&self) -> &RustIllegalFamilyFileReason {
        &self.reason
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RustLegalityFacts {
    legal_workspace_roots: Vec<RustLegalWorkspaceRoot>,
    topology_issues: Vec<RustTopologyIssueFact>,
    legal_family_files: Vec<RustLegalFamilyFileFact>,
    illegal_family_files: Vec<RustIllegalFamilyFileFact>,
}

impl RustLegalityFacts {
    #[must_use]
    pub fn new(
        legal_workspace_roots: Vec<RustLegalWorkspaceRoot>,
        topology_issues: Vec<RustTopologyIssueFact>,
        legal_family_files: Vec<RustLegalFamilyFileFact>,
        illegal_family_files: Vec<RustIllegalFamilyFileFact>,
    ) -> Self {
        Self {
            legal_workspace_roots,
            topology_issues,
            legal_family_files,
            illegal_family_files,
        }
    }

    #[must_use]
    pub fn legal_workspace_roots(&self) -> &[RustLegalWorkspaceRoot] {
        &self.legal_workspace_roots
    }

    #[must_use]
    pub fn topology_issues(&self) -> &[RustTopologyIssueFact] {
        &self.topology_issues
    }

    #[must_use]
    pub fn legal_family_files(&self) -> &[RustLegalFamilyFileFact] {
        &self.legal_family_files
    }

    #[must_use]
    pub fn illegal_family_files(&self) -> &[RustIllegalFamilyFileFact] {
        &self.illegal_family_files
    }
}

#[derive(Debug, Clone)]
struct CargoRootSnapshot {
    rel_dir: String,
    cargo_rel_path: String,
    classification: RustRootClassification,
    parse_error: Option<String>,
    has_workspace: bool,
    has_package: bool,
    expanded_members: Vec<String>,
    escaping_member_patterns: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RootLegality {
    LegalWorkspace,
    LegalMember,
    Illegal,
}

#[must_use]
pub fn collect(
    tree: &ProjectTree,
    placement: &RustRootPlacementFacts,
    owned_surface: &RustOwnedSurfaceFacts,
) -> RustLegalityFacts {
    let snapshots = collect_snapshots(tree, placement);
    let top_level_workspaces = top_level_workspace_candidates(&snapshots);
    let mut legal_workspace_roots = Vec::new();
    let mut topology_issues = Vec::new();
    let mut root_legality = BTreeMap::<String, RootLegality>::new();
    let mut workspace_members = BTreeMap::<String, BTreeSet<String>>::new();

    for snapshot in snapshots.values() {
        if snapshot.has_workspace {
            if let Some(parent_workspace_rel) = nearest_ancestor_workspace(&snapshot.rel_dir, &top_level_workspaces) {
                topology_issues.push(RustTopologyIssueFact::new(
                    snapshot.rel_dir.clone(),
                    snapshot.cargo_rel_path.clone(),
                    snapshot.classification,
                    RustTopologyIssueKind::NestedWorkspace {
                        parent_workspace_rel,
                    },
                ));
                let _ = root_legality.insert(snapshot.rel_dir.clone(), RootLegality::Illegal);
                continue;
            }

            for member_pattern in &snapshot.escaping_member_patterns {
                topology_issues.push(RustTopologyIssueFact::new(
                    snapshot.rel_dir.clone(),
                    snapshot.cargo_rel_path.clone(),
                    snapshot.classification,
                    RustTopologyIssueKind::WorkspaceMemberPathEscapesRoot {
                        workspace_root_rel: snapshot.rel_dir.clone(),
                        member_pattern: member_pattern.clone(),
                    },
                ));
            }

            if snapshot.classification == RustRootClassification::Auxiliary && !snapshot.has_workspace {
                topology_issues.push(RustTopologyIssueFact::new(
                    snapshot.rel_dir.clone(),
                    snapshot.cargo_rel_path.clone(),
                    snapshot.classification,
                    RustTopologyIssueKind::AuxiliaryTopLevelRootMustBeWorkspace,
                ));
                let _ = root_legality.insert(snapshot.rel_dir.clone(), RootLegality::Illegal);
                continue;
            }

            if root_is_workspace_eligible(snapshot) {
                legal_workspace_roots.push(RustLegalWorkspaceRoot::new(
                    snapshot.rel_dir.clone(),
                    snapshot.cargo_rel_path.clone(),
                    snapshot.classification,
                ));
                let _ = workspace_members.insert(
                    snapshot.rel_dir.clone(),
                    snapshot.expanded_members.iter().cloned().collect(),
                );
                let _ = root_legality.insert(snapshot.rel_dir.clone(), RootLegality::LegalWorkspace);
            } else {
                let _ = root_legality.insert(snapshot.rel_dir.clone(), RootLegality::Illegal);
            }
            continue;
        }

        if let Some(workspace_root_rel) = nearest_ancestor_workspace(&snapshot.rel_dir, &top_level_workspaces) {
            let declared = workspace_members
                .get(&workspace_root_rel)
                .is_some_and(|members| members.contains(&snapshot.rel_dir));
            if declared && root_is_member_candidate(snapshot) {
                let _ = root_legality.insert(snapshot.rel_dir.clone(), RootLegality::LegalMember);
            } else {
                topology_issues.push(RustTopologyIssueFact::new(
                    snapshot.rel_dir.clone(),
                    snapshot.cargo_rel_path.clone(),
                    snapshot.classification,
                    RustTopologyIssueKind::UndeclaredWorkspaceMember {
                        workspace_root_rel,
                    },
                ));
                let _ = root_legality.insert(snapshot.rel_dir.clone(), RootLegality::Illegal);
            }
            continue;
        }

        if snapshot.classification == RustRootClassification::Auxiliary {
            topology_issues.push(RustTopologyIssueFact::new(
                snapshot.rel_dir.clone(),
                snapshot.cargo_rel_path.clone(),
                snapshot.classification,
                RustTopologyIssueKind::AuxiliaryTopLevelRootMustBeWorkspace,
            ));
        } else {
            topology_issues.push(RustTopologyIssueFact::new(
                snapshot.rel_dir.clone(),
                snapshot.cargo_rel_path.clone(),
                snapshot.classification,
                RustTopologyIssueKind::TopLevelRootMustBeWorkspace,
            ));
            if snapshot.has_package {
                topology_issues.push(RustTopologyIssueFact::new(
                    snapshot.rel_dir.clone(),
                    snapshot.cargo_rel_path.clone(),
                    snapshot.classification,
                    RustTopologyIssueKind::LooseTopLevelPackage,
                ));
            }
        }
        let _ = root_legality.insert(snapshot.rel_dir.clone(), RootLegality::Illegal);
    }

    legal_workspace_roots.sort_by(|left, right| left.cargo_rel_path.cmp(&right.cargo_rel_path));
    topology_issues.sort_by(|left, right| {
        left.cargo_rel_path
            .cmp(&right.cargo_rel_path)
            .then(issue_sort_key(left.kind()).cmp(&issue_sort_key(right.kind())))
    });
    topology_issues.dedup();

    let legal_workspace_rels = legal_workspace_roots
        .iter()
        .map(|root| root.rel_dir.clone())
        .collect::<BTreeSet<_>>();
    let legal_member_rels = root_legality
        .iter()
        .filter_map(|(rel, legality)| (*legality == RootLegality::LegalMember).then_some(rel.clone()))
        .collect::<BTreeSet<_>>();

    let mut legal_family_files = Vec::new();
    let mut illegal_family_files = Vec::new();
    for fact in owned_surface.family_files() {
        if fact.kind() == RustFamilyFileKind::CargoToml {
            match cargo_file_workspace_owner(fact, &legal_workspace_rels, &legal_member_rels) {
                Some(workspace_root_rel) => {
                    legal_family_files.push(RustLegalFamilyFileFact::new(
                        fact.family(),
                        fact.rel_path().to_owned(),
                        fact.kind(),
                        fact.attachment().clone(),
                        workspace_root_rel,
                    ));
                }
                None => {}
            }
            continue;
        }

        match legal_policy_file_workspace_owner(fact, &legal_workspace_rels, &legal_member_rels) {
            Ok(workspace_root_rel) => {
                legal_family_files.push(RustLegalFamilyFileFact::new(
                    fact.family(),
                    fact.rel_path().to_owned(),
                    fact.kind(),
                    fact.attachment().clone(),
                    workspace_root_rel,
                ));
            }
            Err(reason) => {
                illegal_family_files.push(RustIllegalFamilyFileFact::new(
                    fact.family(),
                    fact.rel_path().to_owned(),
                    fact.kind(),
                    fact.attachment().clone(),
                    reason,
                ));
            }
        }
    }

    legal_family_files.sort_by(|left, right| {
        left.family()
            .cmp(&right.family())
            .then(left.workspace_root_rel().cmp(right.workspace_root_rel()))
            .then(left.rel_path().cmp(right.rel_path()))
            .then(left.kind().cmp(&right.kind()))
    });
    illegal_family_files.sort_by(|left, right| {
        left.family()
            .cmp(&right.family())
            .then(left.rel_path().cmp(right.rel_path()))
            .then(left.kind().cmp(&right.kind()))
    });

    RustLegalityFacts::new(
        legal_workspace_roots,
        topology_issues,
        legal_family_files,
        illegal_family_files,
    )
}

fn collect_snapshots(
    tree: &ProjectTree,
    placement: &RustRootPlacementFacts,
) -> BTreeMap<String, CargoRootSnapshot> {
    placement
        .roots()
        .iter()
        .map(|root| {
            let snapshot = cargo_root_snapshot(tree, root.rel_dir(), root.cargo_rel_path(), root.classification());
            (snapshot.rel_dir.clone(), snapshot)
        })
        .collect()
}

fn cargo_root_snapshot(
    tree: &ProjectTree,
    rel_dir: &str,
    cargo_rel_path: &str,
    classification: RustRootClassification,
) -> CargoRootSnapshot {
    let Some(content) = tree.file_content(cargo_rel_path) else {
        return CargoRootSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            classification,
            parse_error: Some("Cargo.toml content missing from ProjectTree".to_owned()),
            has_workspace: false,
            has_package: false,
            expanded_members: Vec::new(),
            escaping_member_patterns: Vec::new(),
        };
    };

    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => {
            let has_workspace = parsed.get("workspace").is_some();
            let has_package = parsed.get("package").is_some();
            let (expanded_members, escaping_member_patterns) =
                parse_workspace_members(tree, rel_dir, &parsed);
            CargoRootSnapshot {
                rel_dir: rel_dir.to_owned(),
                cargo_rel_path: cargo_rel_path.to_owned(),
                classification,
                parse_error: None,
                has_workspace,
                has_package,
                expanded_members,
                escaping_member_patterns,
            }
        }
        Err(parse_error) => CargoRootSnapshot {
            rel_dir: rel_dir.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            classification,
            parse_error: Some(parse_error.to_string()),
            has_workspace: false,
            has_package: false,
            expanded_members: Vec::new(),
            escaping_member_patterns: Vec::new(),
        },
    }
}

fn parse_workspace_members(
    tree: &ProjectTree,
    workspace_rel: &str,
    parsed: &toml::Value,
) -> (Vec<String>, Vec<String>) {
    let Some(raw_members) = parsed
        .get("workspace")
        .and_then(|value| value.get("members"))
        .and_then(toml::Value::as_array)
    else {
        return (Vec::new(), Vec::new());
    };

    let mut expanded = BTreeSet::new();
    let mut escaping = BTreeSet::new();
    for member in raw_members.iter().filter_map(toml::Value::as_str) {
        if member_pattern_escapes_root(member) {
            let _ = escaping.insert(member.to_owned());
            continue;
        }
        for member_rel in expand_member_pattern(tree, workspace_rel, member) {
            let _ = expanded.insert(member_rel);
        }
    }

    (
        expanded.into_iter().collect(),
        escaping.into_iter().collect(),
    )
}

fn expand_member_pattern(tree: &ProjectTree, workspace_rel: &str, member: &str) -> Vec<String> {
    let trimmed = member.trim_matches('/');
    let pattern = if workspace_rel.is_empty() {
        trimmed.to_owned()
    } else {
        ProjectTree::join_rel(workspace_rel, trimmed)
    };
    if trimmed.contains('*') || trimmed.contains('?') || trimmed.contains('[') {
        tree.matching_dir_rels(&pattern)
    } else {
        vec![pattern]
    }
}

fn member_pattern_escapes_root(member: &str) -> bool {
    member
        .split('/')
        .any(|segment| segment == "..")
}

fn top_level_workspace_candidates(
    snapshots: &BTreeMap<String, CargoRootSnapshot>,
) -> BTreeSet<String> {
    snapshots
        .values()
        .filter(|snapshot| snapshot.has_workspace)
        .map(|snapshot| snapshot.rel_dir.clone())
        .collect()
}

fn nearest_ancestor_workspace(
    rel_dir: &str,
    workspace_rels: &BTreeSet<String>,
) -> Option<String> {
    workspace_rels
        .iter()
        .filter(|workspace_rel| *workspace_rel != rel_dir && path_is_under(rel_dir, workspace_rel))
        .max_by_key(|workspace_rel| workspace_rel.len())
        .cloned()
}

fn root_is_workspace_eligible(snapshot: &CargoRootSnapshot) -> bool {
    snapshot.has_workspace
        && snapshot.parse_error.is_none()
        && (snapshot.rel_dir.is_empty()
            || matches!(
                snapshot.classification,
                RustRootClassification::App
                    | RustRootClassification::Package
                    | RustRootClassification::Auxiliary
            ))
}

fn root_is_member_candidate(snapshot: &CargoRootSnapshot) -> bool {
    snapshot.parse_error.is_none() && snapshot.has_package && !snapshot.has_workspace
}

fn cargo_file_workspace_owner(
    fact: &RustFamilyFileFact,
    legal_workspace_rels: &BTreeSet<String>,
    legal_member_rels: &BTreeSet<String>,
) -> Option<String> {
    match fact.attachment() {
        RustFamilyFileAttachment::ExactRoot { root_rel } => {
            if legal_workspace_rels.contains(root_rel) {
                return Some(root_rel.clone());
            }
            if legal_member_rels.contains(root_rel) {
                return legal_workspace_rels
                    .iter()
                    .filter(|workspace_rel| path_is_under(root_rel, workspace_rel))
                    .max_by_key(|workspace_rel| workspace_rel.len())
                    .cloned();
            }
            None
        }
        RustFamilyFileAttachment::NestedUnderRoot { .. }
        | RustFamilyFileAttachment::AncestorOfRoots { .. }
        | RustFamilyFileAttachment::OutsideRoots { .. } => None,
    }
}

fn legal_policy_file_workspace_owner(
    fact: &RustFamilyFileFact,
    legal_workspace_rels: &BTreeSet<String>,
    legal_member_rels: &BTreeSet<String>,
) -> Result<String, RustIllegalFamilyFileReason> {
    match fact.attachment() {
        RustFamilyFileAttachment::ExactRoot { root_rel } => {
            if legal_workspace_rels.contains(root_rel) {
                Ok(root_rel.clone())
            } else if legal_member_rels.contains(root_rel) {
                let workspace_root_rel = legal_workspace_rels
                    .iter()
                    .filter(|workspace_rel| path_is_under(root_rel, workspace_rel))
                    .max_by_key(|workspace_rel| workspace_rel.len())
                    .cloned()
                    .unwrap_or_default();
                Err(RustIllegalFamilyFileReason::AttachedToLegalMemberRoot {
                    workspace_root_rel,
                    member_rel: root_rel.clone(),
                })
            } else {
                Err(RustIllegalFamilyFileReason::AttachedToIllegalRoot {
                    root_rel: root_rel.clone(),
                })
            }
        }
        RustFamilyFileAttachment::NestedUnderRoot { root_rel, owner_rel } => {
            let workspace_root_rel = if legal_workspace_rels.contains(root_rel) {
                root_rel.clone()
            } else {
                legal_workspace_rels
                    .iter()
                    .filter(|workspace_rel| path_is_under(owner_rel, workspace_rel))
                    .max_by_key(|workspace_rel| workspace_rel.len())
                    .cloned()
                    .unwrap_or_else(|| root_rel.clone())
            };
            Err(RustIllegalFamilyFileReason::NestedBeneathLegalWorkspace {
                workspace_root_rel,
                owner_rel: owner_rel.clone(),
            })
        }
        RustFamilyFileAttachment::AncestorOfRoots {
            root_rels,
            ..
        } => {
            if fact.kind() == RustFamilyFileKind::GuardrailToml {
                let legal_roots = root_rels
                    .iter()
                    .filter(|root_rel| legal_workspace_rels.contains(*root_rel))
                    .cloned()
                    .collect::<Vec<_>>();
                if let Some(workspace_root_rel) =
                    legal_roots.iter().max_by_key(|root_rel| root_rel.len())
                {
                    return Ok(workspace_root_rel.clone());
                }
            }

            Err(RustIllegalFamilyFileReason::AboveLegalWorkspaceRoots {
                workspace_root_rels: root_rels
                    .iter()
                    .filter(|root_rel| legal_workspace_rels.contains(*root_rel))
                    .cloned()
                    .collect(),
            })
        }
        RustFamilyFileAttachment::OutsideRoots { .. } => {
            Err(RustIllegalFamilyFileReason::OutsideEveryLegalWorkspace)
        }
    }
}

fn issue_sort_key(kind: &RustTopologyIssueKind) -> (&'static str, String) {
    match kind {
        RustTopologyIssueKind::TopLevelRootMustBeWorkspace => ("top-level-workspace", String::new()),
        RustTopologyIssueKind::LooseTopLevelPackage => ("loose-top-level-package", String::new()),
        RustTopologyIssueKind::NestedWorkspace {
            parent_workspace_rel,
        } => ("nested-workspace", parent_workspace_rel.clone()),
        RustTopologyIssueKind::UndeclaredWorkspaceMember {
            workspace_root_rel,
        } => ("undeclared-member", workspace_root_rel.clone()),
        RustTopologyIssueKind::WorkspaceMemberPathEscapesRoot {
            workspace_root_rel,
            member_pattern,
        } => ("member-path-escape", format!("{workspace_root_rel}:{member_pattern}")),
        RustTopologyIssueKind::AuxiliaryTopLevelRootMustBeWorkspace => {
            ("auxiliary-top-level-workspace", String::new())
        }
    }
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

#[cfg(test)]
#[path = "lib_tests/mod.rs"]
mod lib_tests;
