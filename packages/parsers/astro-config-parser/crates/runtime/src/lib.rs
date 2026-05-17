//! Runtime crate for the Astro config parser.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::wildcard_enum_match_arm,
    clippy::arithmetic_side_effects,
    clippy::excessive_nesting,
    clippy::type_complexity,
    clippy::too_many_lines,
    clippy::module_name_repetitions,
    clippy::map_unwrap_or,
    clippy::shadow_unrelated,
    clippy::multiple_crate_versions,
    reason = "SWC AST traversal: wildcard_enum_match_arm fires on every match against swc_ecma_ast enums which evolve upstream and demand a default branch; arithmetic_side_effects fires on AST cursor offsets bounded by program size (usize overflow unreachable); excessive_nesting matches the inherent depth of ECMAScript ImportDecl/CallExpr/MemberExpr trees; type_complexity reflects the typed Astro snapshot tuples; too_many_lines reflects the single-pass SWC reducer; missing_errors_doc would duplicate the crate Error enum docs; module_name_repetitions follows the parser-domain (Astro) + document role naming convention; map_unwrap_or rewrites would obscure the SWC fallback chains; shadow_unrelated fires on SWC visitor bindings that share short names (expr/stmt) across nested visits; multiple_crate_versions arises because swc_common pins siphasher 0.3.11 while a transitive (criterion via dev-tree) pulls siphasher 1.0.2"
)]

/// Typed-document accessor helpers exposed via the public API.
mod document;
/// Error variants surfaced by the parser entry points.
mod error;
/// Filesystem boundary helpers.
mod fs;
/// SWC-based parser that reduces Astro config into a typed snapshot.
mod parser;
/// Re-exports for the typed document model (feature `api`).
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use document::{has_integration, integration, parse_error_reason, typed};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, module_has_runtime_source_import, parse, parse_document};
