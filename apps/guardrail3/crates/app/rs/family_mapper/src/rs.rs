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
        }
    }

    #[must_use]
    pub fn map_rs_arch(&self) -> views::RsArchRoute {
        if !self.selected_families.contains(RustValidateFamily::Arch) {
            return views::RsArchRoute {
                roots: Vec::new(),
                overlaps: Vec::new(),
                input_failures: Vec::new(),
            };
        }

        views::RsArchRoute {
            roots: self
                .scope
                .roots
                .iter()
                .map(|root| views::RsArchRootView {
                    root: root_view(root),
                    classification: root.classification,
                    arch_role: root.arch_role,
                    app_zone_candidates: root.app_zone_candidates.clone(),
                    package_zone_candidates: root.package_zone_candidates.clone(),
                })
                .collect(),
            overlaps: self
                .scope
                .overlaps
                .iter()
                .map(|overlap| views::RsArchOverlapView {
                    app_root_rel: overlap.app_root_rel.clone(),
                    app_cargo_rel_path: overlap.app_cargo_rel_path.clone(),
                    package_root_rel: overlap.package_root_rel.clone(),
                    package_cargo_rel_path: overlap.package_cargo_rel_path.clone(),
                })
                .collect(),
            input_failures: self
                .scope
                .input_failures
                .iter()
                .map(|failure| views::RsRootInputFailureView {
                    rel_path: failure.rel_path.clone(),
                    message: failure.message.clone(),
                })
                .collect(),
        }
    }

    #[must_use]
    pub fn map_rs_hexarch(&self) -> views::RsHexarchRoute {
        if !self.selected_families.contains(RustValidateFamily::Hexarch) {
            return views::RsHexarchRoute {
                roots: Vec::new(),
                scoped_files: None,
                repo_root_cargo_rel_path: None,
                guardrail_config_rel_path: None,
            };
        }

        let roots = self.map_roots_for_family(RustValidateFamily::Hexarch, |root| {
            root.classification == RustRootClassification::App
                && self.root_is_live_for_hexarch(root)
        });
        let root_rels = roots
            .iter()
            .map(|root| root.rel_dir.clone())
            .collect::<Vec<_>>();

        views::RsHexarchRoute {
            scoped_files: filter_for_roots(self.tree, self.scoped_files, &root_rels),
            roots,
            repo_root_cargo_rel_path: self
                .tree
                .file_exists("Cargo.toml")
                .then(|| "Cargo.toml".to_owned()),
            guardrail_config_rel_path: self
                .tree
                .file_exists("guardrail3.toml")
                .then(|| "guardrail3.toml".to_owned()),
        }
    }

    #[must_use]
    pub fn map_rs_code(&self) -> views::RsCodeRoute {
        self.map_scoped_source_route(RustValidateFamily::Code)
    }

    #[must_use]
    pub fn map_rs_clippy(&self) -> views::RsClippyRoute {
        views::RsClippyRoute {
            roots: self.map_roots_for_family(RustValidateFamily::Clippy, |_| true),
        }
    }

    #[must_use]
    pub fn map_rs_cargo(&self) -> views::RsCargoRoute {
        views::RsCargoRoute {
            roots: self.map_roots_for_family(RustValidateFamily::Cargo, |_| true),
        }
    }

    #[must_use]
    pub fn map_rs_deny(&self) -> views::RsDenyRoute {
        views::RsDenyRoute {
            roots: self.map_roots_for_family(RustValidateFamily::Deny, |_| true),
        }
    }

    #[must_use]
    pub fn map_rs_deps(&self) -> views::RsDepsRoute {
        views::RsDepsRoute {
            roots: self.map_roots_for_family(RustValidateFamily::Deps, |_| true),
        }
    }

    #[must_use]
    pub fn map_rs_release(&self) -> views::RsReleaseRoute {
        views::RsReleaseRoute {
            roots: self.map_roots_for_family(RustValidateFamily::Release, |_| true),
        }
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
            .map(|root| root.rel_dir.clone())
            .collect::<Vec<_>>();

        views::RsTestRoute {
            scoped_files: filter_for_roots(self.tree, self.scoped_files, &root_rels),
            roots,
        }
    }

    fn map_scoped_source_route(&self, family: RustValidateFamily) -> views::RsCodeRoute {
        let roots = self.map_scoped_roots_for_family(family, |_| true);
        let root_rels = roots
            .iter()
            .map(|root| root.root.rel_dir.clone())
            .collect::<Vec<_>>();

        views::RsCodeRoute {
            scoped_files: filter_for_roots(self.tree, self.scoped_files, &root_rels),
            roots,
        }
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
            .roots
            .iter()
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
            .roots
            .iter()
            .filter(|root| predicate(root))
            .filter(|root| root_enabled_for_family(root, family, self.config))
            .map(|root| views::RsScopedRootView {
                root: root_view(root),
                classification: root.classification,
            })
            .collect()
    }

    fn root_is_live_for_hexarch(&self, root: &RustRootPlacementRootFacts) -> bool {
        self.tree.file_content(&root.cargo_rel_path).is_some()
    }
}

fn root_view(root: &RustRootPlacementRootFacts) -> views::RsRootView {
    views::RsRootView {
        rel_dir: root.rel_dir.clone(),
        cargo_rel_path: root.cargo_rel_path.clone(),
    }
}

fn root_enabled_for_family(
    root: &RustRootPlacementRootFacts,
    family: RustValidateFamily,
    config: Option<&GuardrailConfig>,
) -> bool {
    let Some(rust) = config.and_then(|value| value.rust.as_ref()) else {
        return true;
    };

    let global = rust
        .checks
        .as_ref()
        .and_then(|checks| checks.family_enabled(family))
        .unwrap_or(true);

    match root_scope(root.rel_dir.as_str()) {
        RootScope::App(app_path) => rust
            .apps
            .as_ref()
            .and_then(|apps| {
                app_path
                    .strip_prefix("apps/")
                    .and_then(|name| apps.get(name))
                    .map(|cfg| effective_family_flag(cfg.checks.as_ref(), family, global))
            })
            .unwrap_or(global),
        RootScope::Packages => rust.packages.as_ref().map_or(global, |cfg| {
            effective_family_flag(cfg.checks.as_ref(), family, global)
        }),
        RootScope::Other => global,
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
