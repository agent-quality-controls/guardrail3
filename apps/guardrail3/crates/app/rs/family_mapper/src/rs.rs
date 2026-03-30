use guardrail3_app_rs_placement::{
    RustRootClassification, RustRootPlacementFacts, RustRootPlacementRootFacts,
};
use guardrail3_domain_config::types::{GuardrailConfig, RustChecksConfig};
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

use crate::scoped_files::filter_for_roots;
use crate::views;

#[derive(Debug)]
pub struct FamilyMapper<'a> {
    tree: &'a ProjectTree,
    scope: &'a RustRootPlacementFacts,
    config: Option<&'a GuardrailConfig>,
    selected_families: &'a RustFamilySelection,
    scoped_files: Option<&'a std::collections::BTreeSet<String>>,
    validation_scope: Option<&'a str>,
}

impl<'a> FamilyMapper<'a> {
    #[must_use]
    pub const fn new(
        tree: &'a ProjectTree,
        scope: &'a RustRootPlacementFacts,
        config: Option<&'a GuardrailConfig>,
        selected_families: &'a RustFamilySelection,
        scoped_files: Option<&'a std::collections::BTreeSet<String>>,
    ) -> Self {
        Self {
            tree,
            scope,
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
    pub fn map_rs_arch(&self) -> views::RsArchRoute {
        if !self.selected_families.contains(RustValidateFamily::Arch) {
            return views::RsArchRoute::new(Vec::new(), Vec::new(), Vec::new());
        }

        views::RsArchRoute::new(
            self.scope
                .roots()
                .iter()
                .filter(|root| self.root_matches_validation_scope(root.rel_dir()))
                .map(|root| {
                    views::RsArchRootView::new(
                        root_view(root),
                        root.classification(),
                        root.arch_role(),
                        root.app_zone_candidates().to_vec(),
                        root.package_zone_candidates().to_vec(),
                    )
                })
                .collect(),
            self.scope
                .overlaps()
                .iter()
                .filter(|overlap| {
                    self.root_matches_validation_scope(overlap.app_root_rel())
                        || self.root_matches_validation_scope(overlap.package_root_rel())
                })
                .map(|overlap| {
                    views::RsArchOverlapView::new(
                        overlap.app_root_rel().to_owned(),
                        overlap.app_cargo_rel_path().to_owned(),
                        overlap.package_root_rel().to_owned(),
                        overlap.package_cargo_rel_path().to_owned(),
                    )
                })
                .collect(),
            self.scope
                .input_failures()
                .iter()
                .map(|failure| {
                    views::RsRootInputFailureView::new(
                        failure.rel_path().to_owned(),
                        failure.message().to_owned(),
                    )
                })
                .collect(),
        )
    }

    #[must_use]
    pub fn map_rs_hexarch(&self) -> views::RsHexarchRoute {
        if !self.selected_families.contains(RustValidateFamily::Hexarch) {
            return views::RsHexarchRoute::new(Vec::new(), None, None, None);
        }

        let roots = self.map_roots_for_family(RustValidateFamily::Hexarch, |root| {
            root.classification() == RustRootClassification::App
                && self.root_is_live_for_hexarch(root)
        });
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();

        views::RsHexarchRoute::new(
            roots,
            filter_for_roots(
                self.tree,
                self.scoped_files,
                &root_rels,
                self.validation_scope,
            ),
            self.tree
                .file_exists("Cargo.toml")
                .then(|| "Cargo.toml".to_owned()),
            self.tree
                .file_exists("guardrail3.toml")
                .then(|| "guardrail3.toml".to_owned()),
        )
    }

    #[must_use]
    pub fn map_rs_code(&self) -> views::RsCodeRoute {
        self.map_scoped_source_route(RustValidateFamily::Code)
    }

    #[must_use]
    pub fn map_rs_clippy(&self) -> views::RsClippyRoute {
        views::RsClippyRoute::new(self.map_roots_for_family(RustValidateFamily::Clippy, |_| true))
            .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_cargo(&self) -> views::RsCargoRoute {
        views::RsCargoRoute::new(self.map_roots_for_family(RustValidateFamily::Cargo, |_| true))
            .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_toolchain(&self) -> views::RsToolchainRoute {
        views::RsToolchainRoute::new(
            self.map_roots_for_family(RustValidateFamily::Toolchain, |_| true),
        )
        .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_deny(&self) -> views::RsDenyRoute {
        views::RsDenyRoute::new(
            self.map_roots_for_family(RustValidateFamily::Deny, |_| true),
            self.validation_scope.map(str::to_owned),
        )
    }

    #[must_use]
    pub fn map_rs_libarch(&self) -> views::RsLibarchRoute {
        views::RsLibarchRoute::new(
            self.map_roots_for_family(RustValidateFamily::Libarch, |root| {
                root.classification() == RustRootClassification::Package
                    && root
                        .package_zone_candidates()
                        .first()
                        .is_some_and(|candidate| candidate == root.rel_dir())
            }),
        )
        .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_deps(&self) -> views::RsDepsRoute {
        views::RsDepsRoute::new(self.map_roots_for_family(RustValidateFamily::Deps, |_| true))
            .with_validation_scope(self.validation_scope.map(str::to_owned))
    }

    #[must_use]
    pub fn map_rs_release(&self) -> views::RsReleaseRoute {
        views::RsReleaseRoute::new(self.map_roots_for_family(RustValidateFamily::Release, |_| true))
    }

    #[must_use]
    pub fn map_rs_garde(&self) -> views::RsGardeRoute {
        self.map_scoped_source_route(RustValidateFamily::Garde)
    }

    #[must_use]
    pub fn map_rs_test(&self) -> views::RsTestRoute {
        let roots = self.map_roots_for_family(RustValidateFamily::Test, |_| true);
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir().to_owned())
            .collect::<Vec<_>>();

        views::RsTestRoute::new(
            roots,
            filter_for_roots(
                self.tree,
                self.scoped_files,
                &root_rels,
                self.validation_scope,
            ),
        )
    }

    fn map_scoped_source_route(&self, family: RustValidateFamily) -> views::RsCodeRoute {
        let roots = self.map_scoped_roots_for_family(family, |_| true);
        let root_rels = roots
            .iter()
            .map(|root| root.root().rel_dir().to_owned())
            .collect::<Vec<_>>();

        views::RsCodeRoute::new(
            roots,
            filter_for_roots(
                self.tree,
                self.scoped_files,
                &root_rels,
                self.validation_scope,
            ),
        )
    }

    fn map_roots_for_family<F>(
        &self,
        family: RustValidateFamily,
        predicate: F,
    ) -> Vec<views::RsRootView>
    where
        F: Fn(&RustRootPlacementRootFacts) -> bool,
    {
        if !self.selected_families.contains(family) {
            return Vec::new();
        }

        self.scope
            .roots()
            .iter()
            .filter(|root| self.root_matches_validation_scope(root.rel_dir()))
            .filter(|root| predicate(root))
            .filter(|root| root_enabled_for_family(root, family, self.config))
            .map(root_view)
            .collect()
    }

    fn map_scoped_roots_for_family<F>(
        &self,
        family: RustValidateFamily,
        predicate: F,
    ) -> Vec<views::RsScopedRootView>
    where
        F: Fn(&RustRootPlacementRootFacts) -> bool,
    {
        if !self.selected_families.contains(family) {
            return Vec::new();
        }

        self.scope
            .roots()
            .iter()
            .filter(|root| self.root_matches_validation_scope(root.rel_dir()))
            .filter(|root| predicate(root))
            .filter(|root| root_enabled_for_family(root, family, self.config))
            .map(|root| views::RsScopedRootView::new(root_view(root), root.classification()))
            .collect()
    }

    fn root_is_live_for_hexarch(&self, root: &RustRootPlacementRootFacts) -> bool {
        self.tree.file_content(root.cargo_rel_path()).is_some()
    }

    fn root_matches_validation_scope(&self, root_rel: &str) -> bool {
        self.validation_scope.is_none_or(|scope_rel| {
            path_is_under(root_rel, scope_rel) || path_is_under(scope_rel, root_rel)
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

fn root_view(root: &RustRootPlacementRootFacts) -> views::RsRootView {
    views::RsRootView::new(root.rel_dir().to_owned(), root.cargo_rel_path().to_owned())
}

fn root_enabled_for_family(
    root: &RustRootPlacementRootFacts,
    family: RustValidateFamily,
    config: Option<&GuardrailConfig>,
) -> bool {
    let Some(rust) = config.and_then(GuardrailConfig::rust) else {
        return true;
    };

    let global = rust
        .checks()
        .and_then(|checks| checks.family_enabled(family))
        .unwrap_or(true);
    let app_count = rust.apps().map_or(0, std::collections::BTreeMap::len);
    let has_packages_scope = rust.packages().is_some();

    match root_scope(root.rel_dir()) {
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
    let mut segments = rel_dir.split('/').filter(|segment| !segment.is_empty());
    match (segments.next(), segments.next()) {
        (Some("apps"), Some(app_name)) => RootScope::App(format!("apps/{app_name}")),
        (Some("packages"), _) => RootScope::Packages,
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
        match rel_dir.split('/').next() {
            Some("apps") => RustRootClassification::App,
            Some("packages") => RustRootClassification::Package,
            _ => RustRootClassification::Other,
        },
        None,
        if rel_dir.starts_with("apps/") {
            vec!["apps/guardrail3".to_owned()]
        } else {
            Vec::new()
        },
        Vec::new(),
        Vec::new(),
    )
}

#[cfg(test)]
#[path = "rs_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_tests;
