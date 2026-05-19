use g3_guardrail_toml_types::WaiverConfig;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool encodes an independent observable attribute of an arch crate node (Cargo `[package]` table presence, `[workspace]` table presence, `src/lib.rs` presence, `src/main.rs` presence, shared-marker); downstream rule packages (g3rs-arch-source-checks, g3rs-arch-ingestion) pattern-match these flags independently, so collapsing them into an enum would break out-of-scope callers"
)]
pub struct G3RsArchCrateNode {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub package_name: Option<String>,
    pub has_package: bool,
    pub has_workspace: bool,
    pub has_lib_rs: bool,
    pub has_main_rs: bool,
    pub lib_rs_rel: Option<String>,
    pub parent_rel_dir: Option<String>,
    pub shared: bool,
    pub feature_contract: G3RsArchFeatureContract,
    pub dependency_counts: G3RsArchDependencyCounts,
    pub cargo_parse_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchFeatureContract {
    pub has_default_feature: bool,
    pub has_all_feature: bool,
    pub all_feature_deps: Vec<String>,
    pub default_feature_deps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchDependencyCounts {
    pub production: usize,
    pub dev: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchFacadeItem {
    pub line: usize,
    pub kind: &'static str,
    pub name: String,
    pub is_broad_reexport: bool,
    pub feature_gate: Option<String>,
    pub gated_on_all: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchFeatureExport {
    pub line: usize,
    pub name: String,
    pub feature_gate: Option<String>,
    pub gated_on_all: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchFacadeSurface {
    pub rel_path: String,
    pub is_lib_rs: bool,
    pub is_mod_rs: bool,
    pub body_items: Vec<G3RsArchFacadeItem>,
    pub broad_reexports: Vec<G3RsArchFacadeItem>,
    pub pub_exports: Vec<G3RsArchFeatureExport>,
    pub pub_export_count: usize,
    pub ungated_export_count: usize,
    pub gated_on_all_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchModuleDir {
    pub dir_rel: String,
    pub mod_decl_file: String,
    pub mod_decl_line: usize,
    pub is_pub: bool,
    pub has_mod_rs: bool,
    pub has_sibling_file: bool,
    pub rs_file_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum G3RsArchBoundaryRef {
    RootWorkspace,
    Crate(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchDependencyEdge {
    pub source_rel_dir: String,
    pub source_cargo_rel: String,
    pub dep_alias: String,
    pub raw_path: String,
    pub resolved_target_rel: Option<String>,
    pub target_is_crate: bool,
    pub section: String,
    pub crossed_boundary: Option<G3RsArchBoundaryRef>,
    pub is_direct_child: bool,
    pub target_shared: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchSourceCrate {
    pub rel_dir: String,
    pub lib_rs_rel: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchLibFacadeChecksInput {
    pub krate: G3RsArchSourceCrate,
    pub lib_surface: Option<G3RsArchFacadeSurface>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchPathAttrSite {
    pub rel_path: String,
    pub line: usize,
    pub module_name: String,
    pub path_value: Option<String>,
    pub cfg_test_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchSourceChecksInput {
    pub lib_facade_checks: Vec<G3RsArchLibFacadeChecksInput>,
    pub mod_facade_surfaces: Vec<G3RsArchFacadeSurface>,
    pub path_attr_sites: Vec<G3RsArchPathAttrSite>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool encodes an independent observable attribute of an arch config crate (shared-marker, feature-contract requirement, presence of `default` feature, presence of `all` feature); downstream rule packages pattern-match these flags independently, so collapsing them would break out-of-scope callers"
)]
pub struct G3RsArchConfigCrate {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub shared: bool,
    pub production_dependency_count: usize,
    pub dev_dependency_count: usize,
    pub requires_feature_contract: bool,
    pub has_default_feature: bool,
    pub has_all_feature: bool,
    pub all_feature_deps: Vec<String>,
    pub default_feature_deps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct G3RsArchConfigChecksInput {
    pub crates: Vec<G3RsArchConfigCrate>,
    pub dependency_edges: Vec<G3RsArchDependencyEdge>,
    pub rust_policy: G3RsArchRustPolicyState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsArchFileTreeCrate {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub has_package: bool,
    pub has_lib_rs: bool,
    pub has_main_rs: bool,
    pub cargo_parse_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum G3RsArchRustPolicyState {
    Missing,
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        rel_path: String,
        waivers: Vec<WaiverConfig>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct G3RsArchFileTreeChecksInput {
    pub crates: Vec<G3RsArchFileTreeCrate>,
    pub module_dirs: Vec<G3RsArchModuleDir>,
    pub rust_policy: G3RsArchRustPolicyState,
}
