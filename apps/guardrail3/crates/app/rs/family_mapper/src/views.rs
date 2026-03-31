use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use guardrail3_app_rs_legality::{RustTopologyIssueFact, RustTopologyIssueKind};
use guardrail3_app_rs_ownership::{RustFamilyFileAttachment, RustFamilyFileKind};
use guardrail3_app_rs_placement::{RustArchRole, RustRootClassification};
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
use guardrail3_validation_model::RustValidateFamily;

#[derive(Debug, Clone)]
pub struct RsProjectSurface {
    tree: ProjectTree,
}

impl RsProjectSurface {
    #[must_use]
    pub fn from_tree(tree: &ProjectTree) -> Self {
        Self { tree: tree.clone() }
    }

    #[must_use]
    pub fn from_route_scope(
        tree: &ProjectTree,
        root_rels: &[String],
        extra_file_rels: &[String],
        scoped_files: Option<&BTreeSet<String>>,
    ) -> Self {
        let mut allowed_files = BTreeSet::new();
        let mut allowed_dirs = BTreeSet::new();

        for (dir_rel, entry) in tree.structure() {
            if root_rels
                .iter()
                .any(|root_rel| path_is_under(dir_rel, root_rel))
            {
                let _ = allowed_dirs.insert(dir_rel.clone());
                for file_name in entry.files() {
                    let rel_path = ProjectTree::join_rel(dir_rel, file_name);
                    let _ = allowed_files.insert(rel_path);
                }
            }
        }

        for rel_path in extra_file_rels {
            let _ = allowed_files.insert(rel_path.clone());
        }

        if let Some(scoped) = scoped_files {
            allowed_files.retain(|rel_path| scoped.contains(rel_path));
            for rel_path in extra_file_rels {
                let _ = allowed_files.insert(rel_path.clone());
            }
        }

        for rel_path in &allowed_files {
            let mut cursor = split_parent(rel_path);
            loop {
                let _ = allowed_dirs.insert(cursor.to_owned());
                if cursor.is_empty() {
                    break;
                }
                cursor = split_parent(cursor);
            }
        }

        let mut structure = BTreeMap::new();
        for dir_rel in &allowed_dirs {
            if let Some(entry) = tree.dir_contents(dir_rel) {
                let filtered_dirs = entry
                    .dirs()
                    .iter()
                    .filter_map(|child| {
                        let rel = ProjectTree::join_rel(dir_rel, child);
                        allowed_dirs.contains(&rel).then_some(child.clone())
                    })
                    .collect::<Vec<_>>();
                let filtered_files = entry
                    .files()
                    .iter()
                    .filter_map(|child| {
                        let rel = ProjectTree::join_rel(dir_rel, child);
                        allowed_files.contains(&rel).then_some(child.clone())
                    })
                    .collect::<Vec<_>>();
                let filtered_symlink_dirs = entry
                    .symlink_dirs()
                    .iter()
                    .filter_map(|child| {
                        let rel = ProjectTree::join_rel(dir_rel, child);
                        allowed_dirs.contains(&rel).then_some(child.clone())
                    })
                    .collect::<Vec<_>>();
                let filtered_symlink_files = entry
                    .symlink_files()
                    .iter()
                    .filter_map(|child| {
                        let rel = ProjectTree::join_rel(dir_rel, child);
                        allowed_files.contains(&rel).then_some(child.clone())
                    })
                    .collect::<Vec<_>>();
                let _ = structure.insert(
                    dir_rel.clone(),
                    DirEntry::new(
                        filtered_dirs,
                        filtered_files,
                        filtered_symlink_dirs,
                        filtered_symlink_files,
                    ),
                );
            }
        }

        let content = tree
            .content()
            .iter()
            .filter_map(|(rel, value)| {
                allowed_files
                    .contains(rel)
                    .then_some((rel.clone(), value.clone()))
            })
            .collect::<BTreeMap<_, _>>();

        let root = PathBuf::from(tree.root());
        Self {
            tree: ProjectTree::new(root, structure, content),
        }
    }

    #[must_use]
    pub fn tree(&self) -> &ProjectTree {
        &self.tree
    }
}

fn split_parent(rel: &str) -> &str {
    rel.rsplit_once('/').map_or("", |(parent, _)| parent)
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsRootView {
    rel_dir: String,
    cargo_rel_path: String,
}

impl RsRootView {
    #[must_use]
    pub fn new(rel_dir: String, cargo_rel_path: String) -> Self {
        Self {
            rel_dir,
            cargo_rel_path,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsArchOverlapView {
    app_root_rel: String,
    app_cargo_rel_path: String,
    package_root_rel: String,
    package_cargo_rel_path: String,
}

impl RsArchOverlapView {
    #[must_use]
    pub fn new(
        app_root_rel: String,
        app_cargo_rel_path: String,
        package_root_rel: String,
        package_cargo_rel_path: String,
    ) -> Self {
        Self {
            app_root_rel,
            app_cargo_rel_path,
            package_root_rel,
            package_cargo_rel_path,
        }
    }

    #[must_use]
    pub fn app_root_rel(&self) -> &str {
        &self.app_root_rel
    }

    #[must_use]
    pub fn app_cargo_rel_path(&self) -> &str {
        &self.app_cargo_rel_path
    }

    #[must_use]
    pub fn package_root_rel(&self) -> &str {
        &self.package_root_rel
    }

    #[must_use]
    pub fn package_cargo_rel_path(&self) -> &str {
        &self.package_cargo_rel_path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsRootInputFailureView {
    rel_path: String,
    message: String,
}

impl RsRootInputFailureView {
    #[must_use]
    pub fn new(rel_path: String, message: String) -> Self {
        Self { rel_path, message }
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsArchRootView {
    root: RsRootView,
    classification: RustRootClassification,
    arch_role: Option<RustArchRole>,
    app_zone_candidates: Vec<String>,
    package_zone_candidates: Vec<String>,
}

impl RsArchRootView {
    #[must_use]
    pub fn new(
        root: RsRootView,
        classification: RustRootClassification,
        arch_role: Option<RustArchRole>,
        app_zone_candidates: Vec<String>,
        package_zone_candidates: Vec<String>,
    ) -> Self {
        Self {
            root,
            classification,
            arch_role,
            app_zone_candidates,
            package_zone_candidates,
        }
    }

    #[must_use]
    pub fn root(&self) -> &RsRootView {
        &self.root
    }

    #[must_use]
    pub const fn classification(&self) -> RustRootClassification {
        self.classification
    }

    #[must_use]
    pub const fn arch_role(&self) -> Option<RustArchRole> {
        self.arch_role
    }

    #[must_use]
    pub fn app_zone_candidates(&self) -> &[String] {
        &self.app_zone_candidates
    }

    #[must_use]
    pub fn package_zone_candidates(&self) -> &[String] {
        &self.package_zone_candidates
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsArchRoute {
    roots: Vec<RsArchRootView>,
    overlaps: Vec<RsArchOverlapView>,
    input_failures: Vec<RsRootInputFailureView>,
    topology_issues: Vec<RsArchTopologyIssueView>,
    family_files: Vec<RsFamilyFileView>,
}

impl RsArchRoute {
    #[must_use]
    pub fn new(
        roots: Vec<RsArchRootView>,
        overlaps: Vec<RsArchOverlapView>,
        input_failures: Vec<RsRootInputFailureView>,
        topology_issues: Vec<RsArchTopologyIssueView>,
        family_files: Vec<RsFamilyFileView>,
    ) -> Self {
        Self {
            roots,
            overlaps,
            input_failures,
            topology_issues,
            family_files,
        }
    }

    #[must_use]
    pub fn roots(&self) -> &[RsArchRootView] {
        &self.roots
    }

    #[must_use]
    pub fn overlaps(&self) -> &[RsArchOverlapView] {
        &self.overlaps
    }

    #[must_use]
    pub fn input_failures(&self) -> &[RsRootInputFailureView] {
        &self.input_failures
    }

    #[must_use]
    pub fn topology_issues(&self) -> &[RsArchTopologyIssueView] {
        &self.topology_issues
    }

    #[must_use]
    pub fn family_files(&self) -> &[RsFamilyFileView] {
        &self.family_files
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsArchTopologyIssueView {
    rel_dir: String,
    cargo_rel_path: String,
    classification: RustRootClassification,
    kind: RsArchTopologyIssueKindView,
}

impl RsArchTopologyIssueView {
    #[must_use]
    pub fn from_fact(issue: &RustTopologyIssueFact) -> Self {
        Self {
            rel_dir: issue.rel_dir().to_owned(),
            cargo_rel_path: issue.cargo_rel_path().to_owned(),
            classification: issue.classification(),
            kind: RsArchTopologyIssueKindView::from_kind(issue.kind()),
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
    pub fn kind(&self) -> &RsArchTopologyIssueKindView {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RsArchTopologyIssueKindView {
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

impl RsArchTopologyIssueKindView {
    #[must_use]
    pub fn from_kind(kind: &RustTopologyIssueKind) -> Self {
        match kind {
            RustTopologyIssueKind::TopLevelRootMustBeWorkspace => Self::TopLevelRootMustBeWorkspace,
            RustTopologyIssueKind::LooseTopLevelPackage => Self::LooseTopLevelPackage,
            RustTopologyIssueKind::NestedWorkspace {
                parent_workspace_rel,
            } => Self::NestedWorkspace {
                parent_workspace_rel: parent_workspace_rel.clone(),
            },
            RustTopologyIssueKind::UndeclaredWorkspaceMember { workspace_root_rel } => {
                Self::UndeclaredWorkspaceMember {
                    workspace_root_rel: workspace_root_rel.clone(),
                }
            }
            RustTopologyIssueKind::WorkspaceMemberPathEscapesRoot {
                workspace_root_rel,
                member_pattern,
            } => Self::WorkspaceMemberPathEscapesRoot {
                workspace_root_rel: workspace_root_rel.clone(),
                member_pattern: member_pattern.clone(),
            },
            RustTopologyIssueKind::AuxiliaryTopLevelRootMustBeWorkspace => {
                Self::AuxiliaryTopLevelRootMustBeWorkspace
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsScopedRootView {
    root: RsRootView,
    classification: RustRootClassification,
}

impl RsScopedRootView {
    #[must_use]
    pub fn new(root: RsRootView, classification: RustRootClassification) -> Self {
        Self {
            root,
            classification,
        }
    }

    #[must_use]
    pub fn root(&self) -> &RsRootView {
        &self.root
    }

    #[must_use]
    pub const fn classification(&self) -> RustRootClassification {
        self.classification
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsScopedSourceRoute {
    roots: Vec<RsScopedRootView>,
    scoped_files: Option<BTreeSet<String>>,
}

impl RsScopedSourceRoute {
    #[must_use]
    pub fn new(roots: Vec<RsScopedRootView>, scoped_files: Option<BTreeSet<String>>) -> Self {
        Self {
            roots,
            scoped_files,
        }
    }

    #[must_use]
    pub fn roots(&self) -> &[RsScopedRootView] {
        &self.roots
    }

    #[must_use]
    pub fn scoped_files(&self) -> Option<&BTreeSet<String>> {
        self.scoped_files.as_ref()
    }
}

pub type RsCodeRoute = RsScopedSourceRoute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsFmtRoute {
    family_files: Vec<RsFamilyFileView>,
}

impl RsFmtRoute {
    #[must_use]
    pub fn new(family_files: Vec<RsFamilyFileView>) -> Self {
        Self { family_files }
    }

    #[must_use]
    pub fn family_files(&self) -> &[RsFamilyFileView] {
        &self.family_files
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsFamilyFileView {
    family: RustValidateFamily,
    rel_path: String,
    kind: RustFamilyFileKind,
    attachment: RsFamilyFileAttachmentView,
    placement: RsFamilyFilePlacementView,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RsFamilyFileAttachmentView {
    ExactRoot {
        root_rel: String,
    },
    NestedUnderRoot {
        root_rel: String,
        owner_rel: String,
    },
    AncestorOfRoots {
        root_rels: Vec<String>,
        owner_rel: String,
    },
    OutsideRoots {
        owner_rel: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RsFamilyFilePlacementView {
    Legal,
    Illegal { reason: String },
}

impl RsFamilyFileView {
    #[must_use]
    pub fn new(
        family: RustValidateFamily,
        rel_path: String,
        kind: RustFamilyFileKind,
        attachment: RsFamilyFileAttachmentView,
        placement: RsFamilyFilePlacementView,
    ) -> Self {
        Self {
            family,
            rel_path,
            kind,
            attachment,
            placement,
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
    pub fn attachment(&self) -> &RsFamilyFileAttachmentView {
        &self.attachment
    }

    #[must_use]
    pub fn placement(&self) -> &RsFamilyFilePlacementView {
        &self.placement
    }

    #[must_use]
    pub fn logical_owner_rel(&self) -> &str {
        self.attachment.logical_owner_rel()
    }

    #[must_use]
    pub fn nearest_rust_root_rel(&self) -> Option<&str> {
        self.attachment.nearest_rust_root_rel()
    }

    #[must_use]
    pub fn ancestor_rust_root_rels(&self) -> Option<&[String]> {
        self.attachment.ancestor_rust_root_rels()
    }

    #[must_use]
    pub fn exact_rust_root_owner(&self) -> bool {
        self.attachment.exact_rust_root_owner()
    }

    #[must_use]
    pub fn placement_is_legal(&self) -> bool {
        matches!(self.placement, RsFamilyFilePlacementView::Legal)
    }

    #[must_use]
    pub fn placement_reason(&self) -> Option<&str> {
        match &self.placement {
            RsFamilyFilePlacementView::Legal => None,
            RsFamilyFilePlacementView::Illegal { reason } => Some(reason.as_str()),
        }
    }

    #[must_use]
    pub fn included_in_workspace_local_surface(&self, root_rel: &str) -> bool {
        if !self.placement_is_legal() {
            return false;
        }

        match self.attachment() {
            RsFamilyFileAttachmentView::ExactRoot {
                root_rel: file_root_rel,
            } => file_root_rel == root_rel,
            RsFamilyFileAttachmentView::NestedUnderRoot {
                root_rel: file_root_rel,
                ..
            } => file_root_rel == root_rel && supports_nested_local_surface(self.kind),
            RsFamilyFileAttachmentView::AncestorOfRoots { root_rels, .. } => {
                supports_ancestor_local_surface(self.kind)
                    && root_rels.iter().any(|candidate| candidate == root_rel)
            }
            RsFamilyFileAttachmentView::OutsideRoots { .. } => false,
        }
    }
}

impl RsFamilyFileAttachmentView {
    #[must_use]
    pub fn from_attachment(attachment: &RustFamilyFileAttachment) -> Self {
        match attachment {
            RustFamilyFileAttachment::ExactRoot { root_rel } => Self::ExactRoot {
                root_rel: root_rel.clone(),
            },
            RustFamilyFileAttachment::NestedUnderRoot {
                root_rel,
                owner_rel,
            } => Self::NestedUnderRoot {
                root_rel: root_rel.clone(),
                owner_rel: owner_rel.clone(),
            },
            RustFamilyFileAttachment::AncestorOfRoots {
                root_rels,
                owner_rel,
            } => Self::AncestorOfRoots {
                root_rels: root_rels.clone(),
                owner_rel: owner_rel.clone(),
            },
            RustFamilyFileAttachment::OutsideRoots { owner_rel } => Self::OutsideRoots {
                owner_rel: owner_rel.clone(),
            },
        }
    }

    #[must_use]
    pub fn logical_owner_rel(&self) -> &str {
        match self {
            Self::ExactRoot { root_rel } => root_rel,
            Self::NestedUnderRoot { owner_rel, .. }
            | Self::AncestorOfRoots { owner_rel, .. }
            | Self::OutsideRoots { owner_rel } => owner_rel,
        }
    }

    #[must_use]
    pub fn nearest_rust_root_rel(&self) -> Option<&str> {
        match self {
            Self::ExactRoot { root_rel } | Self::NestedUnderRoot { root_rel, .. } => {
                Some(root_rel.as_str())
            }
            Self::AncestorOfRoots { .. } | Self::OutsideRoots { .. } => None,
        }
    }

    #[must_use]
    pub fn ancestor_rust_root_rels(&self) -> Option<&[String]> {
        match self {
            Self::AncestorOfRoots { root_rels, .. } => Some(root_rels.as_slice()),
            Self::ExactRoot { .. } | Self::NestedUnderRoot { .. } | Self::OutsideRoots { .. } => {
                None
            }
        }
    }

    #[must_use]
    pub fn exact_rust_root_owner(&self) -> bool {
        matches!(self, Self::ExactRoot { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsCargoRoute {
    roots: Vec<RsRootView>,
    family_files: Vec<RsFamilyFileView>,
    validation_scope: Option<String>,
}

impl RsCargoRoute {
    #[must_use]
    pub fn new(roots: Vec<RsRootView>, family_files: Vec<RsFamilyFileView>) -> Self {
        Self {
            roots,
            family_files,
            validation_scope: None,
        }
    }

    #[must_use]
    pub fn with_validation_scope(mut self, validation_scope: Option<String>) -> Self {
        self.validation_scope = validation_scope;
        self
    }

    #[must_use]
    pub fn roots(&self) -> &[RsRootView] {
        &self.roots
    }

    #[must_use]
    pub fn family_files(&self) -> &[RsFamilyFileView] {
        &self.family_files
    }

    #[must_use]
    pub fn validation_scope(&self) -> Option<&str> {
        self.validation_scope.as_deref()
    }

    #[must_use]
    pub fn for_workspace(&self, root_rel: &str) -> Self {
        Self {
            roots: self
                .roots
                .iter()
                .filter(|root| root.rel_dir() == root_rel)
                .cloned()
                .collect(),
            family_files: self
                .family_files
                .iter()
                .filter(|file| file.included_in_workspace_local_surface(root_rel))
                .cloned()
                .collect(),
            validation_scope: self.validation_scope.clone(),
        }
    }
}

pub type RsClippyRoute = RsCargoRoute;
pub type RsDepsRoute = RsCargoRoute;
pub type RsLibarchRoute = RsCargoRoute;
pub type RsToolchainRoute = RsCargoRoute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsDenyRoute {
    roots: Vec<RsRootView>,
    family_files: Vec<RsFamilyFileView>,
    validation_scope: Option<String>,
}

impl RsDenyRoute {
    #[must_use]
    pub fn new(
        roots: Vec<RsRootView>,
        family_files: Vec<RsFamilyFileView>,
        validation_scope: Option<String>,
    ) -> Self {
        Self {
            roots,
            family_files,
            validation_scope,
        }
    }

    #[must_use]
    pub fn roots(&self) -> &[RsRootView] {
        &self.roots
    }

    #[must_use]
    pub fn family_files(&self) -> &[RsFamilyFileView] {
        &self.family_files
    }

    #[must_use]
    pub fn validation_scope(&self) -> Option<&str> {
        self.validation_scope.as_deref()
    }

    #[must_use]
    pub fn for_workspace(&self, root_rel: &str) -> Self {
        Self {
            roots: self
                .roots
                .iter()
                .filter(|root| root.rel_dir() == root_rel)
                .cloned()
                .collect(),
            family_files: self
                .family_files
                .iter()
                .filter(|file| file.included_in_workspace_local_surface(root_rel))
                .cloned()
                .collect(),
            validation_scope: self.validation_scope.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsReleaseRoute {
    roots: Vec<RsRootView>,
    family_files: Vec<RsFamilyFileView>,
    validation_scope: Option<String>,
}

impl RsReleaseRoute {
    #[must_use]
    pub fn new(roots: Vec<RsRootView>) -> Self {
        Self {
            roots,
            family_files: Vec::new(),
            validation_scope: None,
        }
    }

    #[must_use]
    pub fn with_family_files(mut self, family_files: Vec<RsFamilyFileView>) -> Self {
        self.family_files = family_files;
        self
    }

    #[must_use]
    pub fn with_validation_scope(mut self, validation_scope: Option<String>) -> Self {
        self.validation_scope = validation_scope;
        self
    }

    #[must_use]
    pub fn roots(&self) -> &[RsRootView] {
        &self.roots
    }

    #[must_use]
    pub fn family_files(&self) -> &[RsFamilyFileView] {
        &self.family_files
    }

    #[must_use]
    pub fn validation_scope(&self) -> Option<&str> {
        self.validation_scope.as_deref()
    }

    #[must_use]
    pub fn for_workspace(&self, root_rel: &str) -> Self {
        Self {
            roots: self
                .roots
                .iter()
                .filter(|root| root.rel_dir() == root_rel)
                .cloned()
                .collect(),
            family_files: self
                .family_files
                .iter()
                .filter(|file| file.included_in_workspace_local_surface(root_rel))
                .cloned()
                .collect(),
            validation_scope: self.validation_scope.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsHexarchRoute {
    roots: Vec<RsRootView>,
    scoped_files: Option<BTreeSet<String>>,
    repo_root_cargo_rel_path: Option<String>,
    guardrail_config_rel_path: Option<String>,
}

impl RsHexarchRoute {
    #[must_use]
    pub fn new(
        roots: Vec<RsRootView>,
        scoped_files: Option<BTreeSet<String>>,
        repo_root_cargo_rel_path: Option<String>,
        guardrail_config_rel_path: Option<String>,
    ) -> Self {
        Self {
            roots,
            scoped_files,
            repo_root_cargo_rel_path,
            guardrail_config_rel_path,
        }
    }

    #[must_use]
    pub fn roots(&self) -> &[RsRootView] {
        &self.roots
    }

    #[must_use]
    pub fn scoped_files(&self) -> Option<&BTreeSet<String>> {
        self.scoped_files.as_ref()
    }

    #[must_use]
    pub fn repo_root_cargo_rel_path(&self) -> Option<&str> {
        self.repo_root_cargo_rel_path.as_deref()
    }

    #[must_use]
    pub fn guardrail_config_rel_path(&self) -> Option<&str> {
        self.guardrail_config_rel_path.as_deref()
    }

    #[must_use]
    pub fn for_workspace(&self, root_rel: &str) -> Self {
        Self {
            roots: self
                .roots
                .iter()
                .filter(|root| root.rel_dir() == root_rel)
                .cloned()
                .collect(),
            scoped_files: self.scoped_files.clone(),
            repo_root_cargo_rel_path: self.repo_root_cargo_rel_path.clone(),
            guardrail_config_rel_path: self.guardrail_config_rel_path.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsGardeRoute {
    roots: Vec<RsScopedRootView>,
    scoped_files: Option<BTreeSet<String>>,
    family_files: Vec<RsFamilyFileView>,
}

impl RsGardeRoute {
    #[must_use]
    pub fn new(
        roots: Vec<RsScopedRootView>,
        scoped_files: Option<BTreeSet<String>>,
        family_files: Vec<RsFamilyFileView>,
    ) -> Self {
        Self {
            roots,
            scoped_files,
            family_files,
        }
    }

    #[must_use]
    pub fn roots(&self) -> &[RsScopedRootView] {
        &self.roots
    }

    #[must_use]
    pub fn scoped_files(&self) -> Option<&BTreeSet<String>> {
        self.scoped_files.as_ref()
    }

    #[must_use]
    pub fn family_files(&self) -> &[RsFamilyFileView] {
        &self.family_files
    }

    #[must_use]
    pub fn for_workspace(&self, root_rel: &str) -> Self {
        Self {
            roots: self
                .roots
                .iter()
                .filter(|root| root.root().rel_dir() == root_rel)
                .cloned()
                .collect(),
            scoped_files: self.scoped_files.clone(),
            family_files: self
                .family_files
                .iter()
                .filter(|file| file.included_in_workspace_local_surface(root_rel))
                .cloned()
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsTestRoute {
    roots: Vec<RsRootView>,
    scoped_files: Option<BTreeSet<String>>,
}

impl RsTestRoute {
    #[must_use]
    pub fn new(roots: Vec<RsRootView>, scoped_files: Option<BTreeSet<String>>) -> Self {
        Self {
            roots,
            scoped_files,
        }
    }

    #[must_use]
    pub fn roots(&self) -> &[RsRootView] {
        &self.roots
    }

    #[must_use]
    pub fn scoped_files(&self) -> Option<&BTreeSet<String>> {
        self.scoped_files.as_ref()
    }
}

fn supports_nested_local_surface(kind: RustFamilyFileKind) -> bool {
    matches!(
        kind,
        RustFamilyFileKind::CargoToml | RustFamilyFileKind::GuardrailToml
    )
}

fn supports_ancestor_local_surface(kind: RustFamilyFileKind) -> bool {
    matches!(kind, RustFamilyFileKind::GuardrailToml)
}
