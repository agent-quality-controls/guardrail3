use guardrail3_app_rs_legality::{
    RustIllegalFamilyFileFact, RustLegalFamilyFileFact, RustLegalityFacts,
};
use guardrail3_app_rs_ownership::RustFamilyFileAttachment;
use guardrail3_app_rs_placement::RustRootPlacementRootFacts;
use guardrail3_domain_config::types::{GuardrailConfig, RustChecksConfig};
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

use crate::scoped_files::filter_for_roots;
use crate::views;

pub struct FamilyMapper<'a> {
    legality: &'a RustLegalityFacts,
    config: Option<&'a GuardrailConfig>,
    selected_families: &'a RustFamilySelection,
    scoped_files: Option<&'a std::collections::BTreeSet<String>>,
    validation_scope: Option<&'a str>,
}

impl std::fmt::Debug for FamilyMapper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FamilyMapper")
            .field("legality", &self.legality)
            .field("config", &self.config)
            .field("selected_families", &self.selected_families)
            .field("scoped_files", &self.scoped_files)
            .field("validation_scope", &self.validation_scope)
            .finish()
    }
}

impl<'a> FamilyMapper<'a> {
    #[must_use]
    pub fn from_legality(
        legality: &'a RustLegalityFacts,
        config: Option<&'a GuardrailConfig>,
        selected_families: &'a RustFamilySelection,
        scoped_files: Option<&'a std::collections::BTreeSet<String>>,
    ) -> Self {
        Self {
            legality,
            config,
            selected_families,
            scoped_files,
            validation_scope: None,
        }
    }

    #[must_use]
    pub const fn with_validation_scope(mut self, validation_scope: Option<&'a str>) -> Self {
        self.validation_scope = validation_scope;
        self
    }

    #[must_use]
    pub fn map_rs_topology(&self) -> views::RsTopologyRoute {
        views::RsTopologyRoute::new(
            self.legality.structure()
                .roots()
                .iter()
                .map(|root| {
                    views::RsTopologyRootView::new(
                        root_view(root),
                        root.classification(),
                        root.topology_role(),
                        root.app_zone_candidates().to_vec(),
                        root.package_zone_candidates().to_vec(),
                    )
                })
                .collect(),
            self.legality.structure()
                .overlaps()
                .iter()
                .map(|overlap| {
                    views::RsTopologyOverlapView::new(
                        overlap.app_root_rel().to_owned(),
                        overlap.app_cargo_rel_path().to_owned(),
                        overlap.package_root_rel().to_owned(),
                        overlap.package_cargo_rel_path().to_owned(),
                    )
                })
                .collect(),
            self.legality.structure()
                .input_failures()
                .iter()
                .map(|failure| {
                    views::RsRootInputFailureView::new(
                        failure.rel_path().to_owned(),
                        failure.message().to_owned(),
                    )
                })
                .collect(),
            self.legality
                .topology_issues()
                .iter()
                .map(views::RsTopologyIssueView::from_fact)
                .collect(),
            self.map_topology_family_files(),
        )
    }

    #[must_use]
    pub fn map_rs_arch(&self) -> views::RsArchRoute {
        views::RsArchRoute::new(self.map_global_roots_for_family(RustValidateFamily::Arch), Vec::new())
            .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_hexarch(&self) -> views::RsHexarchRoute {
        if !self.selected_families.contains(RustValidateFamily::Hexarch) {
            return views::RsHexarchRoute::new(Vec::new(), None, None, None);
        }

        let roots = self
            .map_workspace_roots_for_family(RustValidateFamily::Hexarch)
            .into_iter()
            .filter(|root| matches!(root_scope(root.rel_dir()), RootScope::App(_)))
            .filter(|root| self.legality.structure().file_content(root.cargo_rel_path()).is_some())
            .collect::<Vec<_>>();
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();

        views::RsHexarchRoute::new(
            roots,
            filter_for_roots(
                self.legality.structure(),
                self.scoped_files,
                &root_rels,
                self.validation_scope,
            ),
            self.legality.structure()
                .file_content("Cargo.toml")
                .is_some()
                .then(|| "Cargo.toml".to_owned()),
            self.legality.structure()
                .file_content("guardrail3.toml")
                .is_some()
                .then(|| "guardrail3.toml".to_owned()),
        )
    }

    #[must_use]
    pub fn map_rs_code(&self) -> views::RsCodeRoute {
        self.map_global_source_route(RustValidateFamily::Code)
    }

    #[must_use]
    pub fn map_rs_fmt(&self) -> views::RsFmtRoute {
        if !self.selected_families.contains(RustValidateFamily::Fmt) {
            return views::RsFmtRoute::new(Vec::new());
        }
        views::RsFmtRoute::new(
            self.legality
                .legal_family_files()
                .iter()
                .filter(|fact| fact.family() == RustValidateFamily::Fmt)
                .map(legal_file_view)
                .collect(),
        )
    }

    #[must_use]
    pub fn map_rs_clippy(&self) -> views::RsClippyRoute {
        let roots = self.map_workspace_roots_for_family(RustValidateFamily::Clippy);
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();
        views::RsClippyRoute::new(
            roots,
            self.map_local_family_files(RustValidateFamily::Clippy, &root_rels),
        )
        .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_cargo(&self) -> views::RsCargoRoute {
        let roots = self.map_workspace_roots_for_family(RustValidateFamily::Cargo);
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();
        views::RsCargoRoute::new(
            roots,
            self.map_local_family_files(RustValidateFamily::Cargo, &root_rels),
        )
        .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_toolchain(&self) -> views::RsToolchainRoute {
        let roots = self.map_workspace_roots_for_family(RustValidateFamily::Toolchain);
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();
        views::RsToolchainRoute::new(
            roots,
            self.map_local_family_files(RustValidateFamily::Toolchain, &root_rels),
        )
        .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_deny(&self) -> views::RsDenyRoute {
        let roots = self.map_workspace_roots_for_family(RustValidateFamily::Deny);
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();
        views::RsDenyRoute::new(
            roots,
            self.map_local_family_files(RustValidateFamily::Deny, &root_rels),
            self.validation_scope.map(str::to_owned),
        )
    }

    #[must_use]
    pub fn map_rs_libarch(&self) -> views::RsLibarchRoute {
        let roots = self
            .map_manifest_roots_for_family(RustValidateFamily::Libarch)
            .into_iter()
            .filter(|root| matches!(root_scope(root.rel_dir()), RootScope::Packages))
            .collect::<Vec<_>>();
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();
        views::RsLibarchRoute::new(
            roots,
            self.map_local_family_files(RustValidateFamily::Libarch, &root_rels),
        )
        .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_deps(&self) -> views::RsDepsRoute {
        let roots = self.map_workspace_roots_for_family(RustValidateFamily::Deps);
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();

        views::RsDepsRoute::new(
            roots,
            self.map_local_family_files(RustValidateFamily::Deps, &root_rels),
        )
        .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_release(&self) -> views::RsReleaseRoute {
        let roots = self.map_workspace_roots_for_family(RustValidateFamily::Release);
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();
        views::RsReleaseRoute::new(roots)
            .with_family_files(self.map_local_family_files(RustValidateFamily::Release, &root_rels))
            .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_garde(&self) -> views::RsGardeRoute {
        let roots = self.map_scoped_workspace_roots_for_family(RustValidateFamily::Garde);
        let root_rels = roots
            .iter()
            .map(|root| root.root().rel_dir().to_owned())
            .collect::<Vec<_>>();

        views::RsGardeRoute::new(
            roots,
            filter_for_roots(
                self.legality.structure(),
                self.scoped_files,
                &root_rels,
                self.validation_scope,
            ),
            self.map_local_family_files(RustValidateFamily::Garde, &root_rels),
        )
    }

    #[must_use]
    pub fn map_rs_test(&self) -> views::RsTestRoute {
        views::RsTestRoute::new(self.map_global_roots_for_family(RustValidateFamily::Test), None)
    }

    fn map_global_source_route(&self, family: RustValidateFamily) -> views::RsCodeRoute {
        views::RsCodeRoute::new(self.map_global_scoped_roots_for_family(family), None)
    }

    fn map_workspace_roots_for_family(&self, family: RustValidateFamily) -> Vec<views::RsRootView> {
        self.legal_workspace_roots_for_family(family)
            .into_iter()
            .map(|root| {
                views::RsRootView::new(root.rel_dir().to_owned(), root.cargo_rel_path().to_owned())
            })
            .collect()
    }

    fn map_manifest_roots_for_family(&self, family: RustValidateFamily) -> Vec<views::RsRootView> {
        if !self.selected_families.contains(family) {
            return Vec::new();
        }

        let root_rels = self
            .legality
            .legal_family_files()
            .iter()
            .filter(|fact| fact.family() == family)
            .filter(|fact| fact.kind() == RustValidateFamilyFileKind::CargoToml)
            .filter(|fact| self.legal_family_file_matches_scope(fact))
            .filter_map(|fact| match fact.attachment() {
                RustFamilyFileAttachment::ExactRoot { root_rel } => Some(root_rel.clone()),
                RustFamilyFileAttachment::NestedUnderRoot { .. }
                | RustFamilyFileAttachment::AncestorOfRoots { .. }
                | RustFamilyFileAttachment::OutsideRoots { .. } => None,
            })
            .collect::<std::collections::BTreeSet<_>>();

        self.legality.structure()
            .roots()
            .iter()
            .filter(|root| root_rels.contains(root.rel_dir()))
            .filter(|root| root_enabled_for_family(root, family, self.config))
            .map(root_view)
            .collect()
    }

    fn map_scoped_workspace_roots_for_family(
        &self,
        family: RustValidateFamily,
    ) -> Vec<views::RsScopedRootView> {
        self.legal_workspace_roots_for_family(family)
            .into_iter()
            .map(|root| {
                views::RsScopedRootView::new(
                    views::RsRootView::new(
                        root.rel_dir().to_owned(),
                        root.cargo_rel_path().to_owned(),
                    ),
                    root.classification(),
                )
            })
            .collect()
    }

    fn map_global_roots_for_family(&self, family: RustValidateFamily) -> Vec<views::RsRootView> {
        if !self.selected_families.contains(family) {
            return Vec::new();
        }

        self.legality.structure()
            .roots()
            .iter()
            .filter(|root| root_enabled_for_family(root, family, self.config))
            .map(root_view)
            .collect()
    }

    fn map_global_scoped_roots_for_family(
        &self,
        family: RustValidateFamily,
    ) -> Vec<views::RsScopedRootView> {
        if !self.selected_families.contains(family) {
            return Vec::new();
        }

        self.legality.structure()
            .roots()
            .iter()
            .filter(|root| root_enabled_for_family(root, family, self.config))
            .map(|root| views::RsScopedRootView::new(root_view(root), root.classification()))
            .collect()
    }

    fn root_matches_validation_scope(&self, root_rel: &str) -> bool {
        self.validation_scope.is_none_or(|scope_rel| {
            path_is_under(root_rel, scope_rel) || path_is_under(scope_rel, root_rel)
        })
    }

    fn map_local_family_files(
        &self,
        family: RustValidateFamily,
        legal_root_rels: &[String],
    ) -> Vec<views::RsFamilyFileView> {
        self.legality
            .legal_family_files()
            .iter()
            .filter(|fact| fact.family() == family)
            .filter(|fact| {
                legal_root_rels.iter().any(|root_rel| {
                    root_rel == fact.workspace_root_rel()
                        || fact.attachment().ancestor_root_rels().is_some_and(|roots| {
                            roots.iter().any(|candidate| candidate == root_rel)
                        })
                })
            })
            .filter(|fact| self.legal_family_file_matches_scope(fact))
            .map(legal_file_view)
            .collect()
    }

    fn map_topology_family_files(&self) -> Vec<views::RsFamilyFileView> {
        self.legality
            .illegal_family_files()
            .iter()
            .filter(|fact| is_topology_tracked_family_file(fact.family(), fact.kind()))
            .filter(|fact| self.illegal_family_file_matches_scope(fact))
            .map(illegal_file_view)
            .collect()
    }

    fn legal_workspace_roots_for_family(
        &self,
        family: RustValidateFamily,
    ) -> Vec<&guardrail3_app_rs_legality::RustLegalWorkspaceRoot> {
        if !self.selected_families.contains(family) {
            return Vec::new();
        }

        self.legality
            .legal_workspace_roots()
            .iter()
            .filter(|root| self.root_matches_validation_scope(root.rel_dir()))
            .filter(|root| {
                self.legality.structure()
                    .roots()
                    .iter()
                    .find(|candidate| candidate.rel_dir() == root.rel_dir())
                    .is_some_and(|placement_root| {
                        root_enabled_for_family(placement_root, family, self.config)
                    })
            })
            .collect()
    }

    fn legal_family_file_matches_scope(&self, fact: &RustLegalFamilyFileFact) -> bool {
        self.validation_scope.is_none_or(|scope_rel| {
            path_is_under(fact.rel_path(), scope_rel)
                || path_is_under(scope_rel, fact.rel_path())
                || path_is_under(fact.workspace_root_rel(), scope_rel)
                || path_is_under(scope_rel, fact.workspace_root_rel())
                || attachment_matches_scope(fact.attachment(), scope_rel)
        })
    }

    fn illegal_family_file_matches_scope(&self, fact: &RustIllegalFamilyFileFact) -> bool {
        self.validation_scope.is_none_or(|scope_rel| {
            path_is_under(fact.rel_path(), scope_rel)
                || path_is_under(scope_rel, fact.rel_path())
                || attachment_matches_scope(fact.attachment(), scope_rel)
        })
    }
}

fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn attachment_matches_scope(attachment: &RustFamilyFileAttachment, scope_rel: &str) -> bool {
    match attachment {
        RustFamilyFileAttachment::ExactRoot { root_rel } => {
            path_is_under(root_rel, scope_rel) || path_is_under(scope_rel, root_rel)
        }
        RustFamilyFileAttachment::NestedUnderRoot {
            root_rel,
            owner_rel,
        } => {
            path_is_under(owner_rel, scope_rel)
                || path_is_under(scope_rel, owner_rel)
                || path_is_under(root_rel, scope_rel)
                || path_is_under(scope_rel, root_rel)
        }
        RustFamilyFileAttachment::AncestorOfRoots {
            root_rels,
            owner_rel,
        } => {
            path_is_under(owner_rel, scope_rel)
                || path_is_under(scope_rel, owner_rel)
                || root_rels.iter().any(|root_rel| {
                    path_is_under(root_rel, scope_rel) || path_is_under(scope_rel, root_rel)
                })
        }
        RustFamilyFileAttachment::OutsideRoots { owner_rel } => {
            path_is_under(owner_rel, scope_rel) || path_is_under(scope_rel, owner_rel)
        }
    }
}

fn root_view(root: &RustRootPlacementRootFacts) -> views::RsRootView {
    views::RsRootView::new(root.rel_dir().to_owned(), root.cargo_rel_path().to_owned())
}

fn legal_file_view(fact: &RustLegalFamilyFileFact) -> views::RsFamilyFileView {
    views::RsFamilyFileView::new(
        fact.family(),
        fact.rel_path().to_owned(),
        fact.kind(),
        views::RsFamilyFileAttachmentView::from_attachment(fact.attachment()),
        views::RsFamilyFilePlacementView::Legal,
    )
}

fn illegal_file_view(fact: &RustIllegalFamilyFileFact) -> views::RsFamilyFileView {
    views::RsFamilyFileView::new(
        fact.family(),
        fact.rel_path().to_owned(),
        fact.kind(),
        views::RsFamilyFileAttachmentView::from_attachment(fact.attachment()),
        views::RsFamilyFilePlacementView::Illegal {
            reason: illegal_file_reason_text(fact),
        },
    )
}

fn illegal_file_reason_text(fact: &RustIllegalFamilyFileFact) -> String {
    use guardrail3_app_rs_legality::RustIllegalFamilyFileReason;

    match fact.reason() {
        RustIllegalFamilyFileReason::OutsideEveryLegalWorkspace => format!(
            "`{}` is placed outside every legal workspace root for `{}`.",
            fact.rel_path(),
            family_label(fact.family())
        ),
        RustIllegalFamilyFileReason::OutsideValidationRoot => format!(
            "`{}` is placed outside the validation root. `{}` files are only allowed at the validation root.",
            fact.rel_path(),
            family_label(fact.family())
        ),
        RustIllegalFamilyFileReason::AboveLegalWorkspaceRoots {
            workspace_root_rels,
        } => format!(
            "`{}` is placed above legal workspace roots {:?} for `{}`.",
            fact.rel_path(),
            workspace_root_rels,
            family_label(fact.family())
        ),
        RustIllegalFamilyFileReason::NestedBeneathLegalWorkspace {
            workspace_root_rel,
            owner_rel,
        } => format!(
            "`{}` is nested at `{owner_rel}` beneath legal workspace `{workspace_root_rel}`. `{}` files are only allowed at the workspace root.",
            fact.rel_path(),
            family_label(fact.family())
        ),
        RustIllegalFamilyFileReason::AttachedToIllegalRoot { root_rel } => format!(
            "`{}` is attached to illegal Rust root `{root_rel}`. `{}` files are only allowed at legal workspace roots.",
            fact.rel_path(),
            family_label(fact.family())
        ),
        RustIllegalFamilyFileReason::AttachedToLegalMemberRoot {
            workspace_root_rel,
            member_rel,
        } => format!(
            "`{}` is attached to member crate `{member_rel}` under legal workspace `{workspace_root_rel}`. `{}` files are only allowed at the workspace root.",
            fact.rel_path(),
            family_label(fact.family())
        ),
    }
}

fn family_label(family: RustValidateFamily) -> &'static str {
    match family {
        RustValidateFamily::Arch => "arch",
        RustValidateFamily::Toolchain => "toolchain",
        RustValidateFamily::Clippy => "clippy",
        RustValidateFamily::Deny => "deny",
        RustValidateFamily::Cargo => "cargo",
        RustValidateFamily::Deps => "deps",
        RustValidateFamily::Garde => "garde",
        RustValidateFamily::Release => "release",
        RustValidateFamily::Hexarch => "hexarch",
        RustValidateFamily::Libarch => "libarch",
        RustValidateFamily::Topology => "topology",
        RustValidateFamily::Fmt => "fmt",
        RustValidateFamily::Code => "code",
        RustValidateFamily::Test => "test",
        RustValidateFamily::HooksShared => "hooks-shared",
        RustValidateFamily::HooksRs => "hooks-rs",
    }
}

fn is_topology_tracked_family_file(
    family: RustValidateFamily,
    kind: RustValidateFamilyFileKind,
) -> bool {
    matches!(
        family,
        RustValidateFamily::Toolchain
            | RustValidateFamily::Clippy
            | RustValidateFamily::Deny
            | RustValidateFamily::Cargo
            | RustValidateFamily::Deps
            | RustValidateFamily::Garde
            | RustValidateFamily::Release
            | RustValidateFamily::Fmt
    ) && !matches!(kind, RustValidateFamilyFileKind::CargoToml)
}

type RustValidateFamilyFileKind = guardrail3_app_rs_ownership::RustFamilyFileKind;

fn root_enabled_for_family(
    root: &RustRootPlacementRootFacts,
    family: RustValidateFamily,
    config: Option<&GuardrailConfig>,
) -> bool {
    root_enabled_for_family_rel(root.rel_dir(), family, config)
}

fn root_enabled_for_family_rel(
    rel_dir: &str,
    family: RustValidateFamily,
    config: Option<&GuardrailConfig>,
) -> bool {
    if rel_dir.is_empty() {
        return config
            .and_then(GuardrailConfig::rust)
            .and_then(|rust| rust.checks())
            .and_then(|checks| checks.family_enabled(family))
            .unwrap_or(true);
    }

    let Some(rust) = config.and_then(GuardrailConfig::rust) else {
        return true;
    };

    let global = rust
        .checks()
        .and_then(|checks| checks.family_enabled(family))
        .unwrap_or(true);
    let app_count = rust.apps().map_or(0, std::collections::BTreeMap::len);
    let has_packages_scope = rust.packages().is_some();

    match root_scope(rel_dir) {
        RootScope::App(app_path) => rust
            .apps()
            .and_then(|apps| {
                app_path
                    .strip_prefix("apps/")
                    .and_then(|name| apps.get(name))
                    .map(|cfg| effective_family_flag(cfg.checks(), family, global))
            })
            .unwrap_or(global),
        RootScope::Packages => rust.packages().map_or(global, |cfg| {
            effective_family_flag(cfg.checks(), family, global)
        }),
        RootScope::Other => app_count == 0 && !has_packages_scope && global,
    }
}

enum RootScope {
    App(String),
    Packages,
    Other,
}

fn root_scope(rel_dir: &str) -> RootScope {
    let segments = rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let mut app_paths = Vec::new();
    let mut package_hits = 0usize;

    for window in segments.windows(2) {
        match window {
            ["apps", app_name] => app_paths.push(format!("apps/{app_name}")),
            ["packages", _] => package_hits += 1,
            _ => {}
        }
    }

    match (app_paths.len(), package_hits) {
        (1, 0) => RootScope::App(app_paths.remove(0)),
        (0, 1) => RootScope::Packages,
        _ => RootScope::Other,
    }
}

fn effective_family_flag(
    checks: Option<&RustChecksConfig>,
    family: RustValidateFamily,
    global: bool,
) -> bool {
    checks
        .and_then(|value| value.family_enabled(family))
        .unwrap_or(global)
}

#[cfg(test)]
pub(crate) fn root_enabled_for_family_test(
    root: &guardrail3_app_rs_placement::RustRootPlacementRootFacts,
    family: guardrail3_validation_model::RustValidateFamily,
    config: Option<&guardrail3_domain_config::types::GuardrailConfig>,
) -> bool {
    root_enabled_for_family(root, family, config)
}

#[cfg(test)]
pub(crate) fn root_enabled_for_toolchain_test(
    root: &guardrail3_app_rs_placement::RustRootPlacementRootFacts,
    config: Option<&guardrail3_domain_config::types::GuardrailConfig>,
) -> bool {
    root_enabled_for_family_test(
        root,
        guardrail3_validation_model::RustValidateFamily::Toolchain,
        config,
    )
}

#[cfg(test)]
pub(crate) fn app_scoped_config_test() -> guardrail3_domain_config::types::GuardrailConfig {
    use guardrail3_domain_config::types::{
        CrateConfig, GuardrailConfig, RustChecksConfig, RustConfig,
    };

    GuardrailConfig::new(
        None,
        None,
        Some(RustConfig::new(
            Some("apps/guardrail3".to_owned()),
            None,
            Some(std::collections::BTreeMap::from([(
                "guardrail3".to_owned(),
                CrateConfig::new(None, None, Some("service".to_owned()), None, None),
            )])),
            None,
            Some(RustChecksConfig::new(
                None,
                None,
                None,
                Some(true),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )),
        )),
        None,
        None,
    )
}

#[cfg(test)]
pub(crate) fn global_toolchain_enabled_config_test()
-> guardrail3_domain_config::types::GuardrailConfig {
    use guardrail3_domain_config::types::{GuardrailConfig, RustChecksConfig, RustConfig};

    GuardrailConfig::new(
        None,
        None,
        Some(RustConfig::new(
            None,
            None,
            None,
            None,
            Some(RustChecksConfig::new(
                None,
                None,
                None,
                Some(true),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            )),
        )),
        None,
        None,
    )
}

#[cfg(test)]
pub(crate) fn root_test(rel_dir: &str) -> guardrail3_app_rs_placement::RustRootPlacementRootFacts {
    use guardrail3_app_rs_placement::{RustRootClassification, RustRootPlacementRootFacts};

    RustRootPlacementRootFacts::new(
        rel_dir.to_owned(),
        if rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{rel_dir}/Cargo.toml")
        },
        RustRootClassification::App,
        None,
        vec!["apps/guardrail3".to_owned()],
        Vec::new(),
        Vec::new(),
    )
}

#[cfg(test)]
#[path = "rs_tests/mod.rs"]
mod rs_tests;
