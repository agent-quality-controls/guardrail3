use cargo_toml_parser::types::CargoToml;
use clippy_toml_parser::types::ClippyToml;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum G3RsGardeApplicability {
    Inactive,
    Active,
}

#[derive(Debug, Clone, Serialize)]
#[expect(
    clippy::large_enum_variant,
    reason = "ClippyToml carries the parsed payload; downstream consumers (g3rs-garde-ingestion) construct and pattern-match this variant by field name, so boxing here would force breaking out-of-scope callers"
)]
pub enum G3RsGardeClippyInput {
    Missing,
    Parsed {
        rel_path: String,
        clippy: ClippyToml,
    },
    Invalid {
        rel_path: String,
        message: String,
    },
}

/// Input contract for extracted garde config checks.
///
/// The app owns discovery, placement, and parse-failure routing. This package
/// receives already-selected parsed files and validates their config semantics.
#[derive(Debug, Clone, Serialize)]
pub struct G3RsGardeConfigChecksInput {
    pub applicability: G3RsGardeApplicability,
    /// Repo-relative path to the routed root Cargo manifest.
    pub cargo_rel_path: String,
    /// Parsed Cargo manifest content.
    pub cargo: CargoToml,
    /// Covering clippy config state for garde ban checks.
    pub clippy_input: G3RsGardeClippyInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsSourceFile {
    pub rel_path: String,
    pub abs_path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum G3RsGardeBoundaryKind {
    Struct,
    Enum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsGardeDerivedBoundaryTypeSite {
    pub rel_path: String,
    pub line: usize,
    pub name: String,
    pub boundary_kind: G3RsGardeBoundaryKind,
    pub boundary_macros: Vec<String>,
    pub has_validate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsGardeManualDeserializeImplSite {
    pub rel_path: String,
    pub line: usize,
    pub type_name: String,
    pub needs_validate: bool,
    pub has_validate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsGardeQueryAsMacroSite {
    pub rel_path: String,
    pub line: usize,
    pub macro_name: String,
    pub policy_resolved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool encodes an independent observable attribute of a garde boundary field site (validation requirement, nested validation, presence of `#[garde(skip)]`, `#[garde(dive)]`, meaningful garde rule, context use, and parent boundary context); a state machine cannot collapse independent presence flags without losing information"
)]
pub struct G3RsGardeBoundaryFieldSite {
    pub rel_path: String,
    pub line: usize,
    pub boundary_name: String,
    pub field_name: String,
    pub field_type: String,
    pub requires_field_validation: bool,
    pub nested_validated: bool,
    pub has_garde_skip: bool,
    pub has_garde_dive: bool,
    pub has_meaningful_garde_rule: bool,
    pub uses_context: bool,
    pub boundary_has_context: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3RsGardeInputFailureSite {
    pub rel_path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum G3RsGardeRustPolicyInput {
    Missing,
    Parsed {
        rel_path: String,
        garde_enabled: bool,
    },
    Invalid {
        rel_path: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct G3RsGardeSourceChecksInput {
    pub applicability: G3RsGardeApplicability,
    pub garde_dependency_present: bool,
    pub rust_policy: G3RsGardeRustPolicyInput,
    pub input_failures: Vec<G3RsGardeInputFailureSite>,
    pub struct_targets: Vec<G3RsGardeDerivedBoundaryTypeSite>,
    pub enum_targets: Vec<G3RsGardeDerivedBoundaryTypeSite>,
    pub manual_deserialize_impls: Vec<G3RsGardeManualDeserializeImplSite>,
    pub boundary_fields: Vec<G3RsGardeBoundaryFieldSite>,
    pub query_as_macros: Vec<G3RsGardeQueryAsMacroSite>,
}

/// Placeholder input contract for future garde file-tree checks.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct G3RsGardeFileTreeChecksInput;
