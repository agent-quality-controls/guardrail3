use std::collections::BTreeSet;

use guardrail3_app_rs_placement::{RustArchRole, RustRootClassification};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsRootView {
    pub rel_dir: String,
    pub cargo_rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsArchOverlapView {
    pub app_root_rel: String,
    pub app_cargo_rel_path: String,
    pub package_root_rel: String,
    pub package_cargo_rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsRootInputFailureView {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsArchRootView {
    pub root: RsRootView,
    pub classification: RustRootClassification,
    pub arch_role: Option<RustArchRole>,
    pub app_zone_candidates: Vec<String>,
    pub package_zone_candidates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsArchRoute {
    pub roots: Vec<RsArchRootView>,
    pub overlaps: Vec<RsArchOverlapView>,
    pub input_failures: Vec<RsRootInputFailureView>,
    pub reporting_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsHexarchRoute {
    pub roots: Vec<RsRootView>,
    pub scoped_files: Option<BTreeSet<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsTestRoute {
    pub roots: Vec<RsRootView>,
    pub scoped_files: Option<BTreeSet<String>>,
}
