use std::collections::BTreeSet;

use guardrail3_app_rs_placement::{RustArchRole, RustRootClassification};

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
}

impl RsArchRoute {
    #[must_use]
    pub fn new(
        roots: Vec<RsArchRootView>,
        overlaps: Vec<RsArchOverlapView>,
        input_failures: Vec<RsRootInputFailureView>,
    ) -> Self {
        Self {
            roots,
            overlaps,
            input_failures,
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
pub type RsGardeRoute = RsScopedSourceRoute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsCargoRoute {
    roots: Vec<RsRootView>,
    validation_scope: Option<String>,
}

impl RsCargoRoute {
    #[must_use]
    pub fn new(roots: Vec<RsRootView>) -> Self {
        Self {
            roots,
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
    pub fn validation_scope(&self) -> Option<&str> {
        self.validation_scope.as_deref()
    }
}

pub type RsClippyRoute = RsCargoRoute;
pub type RsDepsRoute = RsCargoRoute;
pub type RsLibarchRoute = RsCargoRoute;
pub type RsToolchainRoute = RsCargoRoute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsDenyRoute {
    roots: Vec<RsRootView>,
    validation_scope: Option<String>,
}

impl RsDenyRoute {
    #[must_use]
    pub fn new(roots: Vec<RsRootView>, validation_scope: Option<String>) -> Self {
        Self {
            roots,
            validation_scope,
        }
    }

    #[must_use]
    pub fn roots(&self) -> &[RsRootView] {
        &self.roots
    }

    #[must_use]
    pub fn validation_scope(&self) -> Option<&str> {
        self.validation_scope.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsReleaseRoute {
    roots: Vec<RsRootView>,
}

impl RsReleaseRoute {
    #[must_use]
    pub fn new(roots: Vec<RsRootView>) -> Self {
        Self { roots }
    }

    #[must_use]
    pub fn roots(&self) -> &[RsRootView] {
        &self.roots
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
