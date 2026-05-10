//! Shell-script parsing runtime for hook-script analysis.
//!
//! Each submodule below carries narrow responsibility for one stage of the
//! shell-script analysis pipeline. The crate is intentionally `pub mod`-flat
//! so that the rule rooting in `command_query` can drive lexing, AST shape
//! analysis, and fail-open detection without exposing internal items.

/// Public command-query DSL the hook rules use to ask questions about scripts.
#[cfg(feature = "api")]
pub mod command_query;
/// Detects `|| true`, `; exit 0`, and similar fail-open suffixes on shell commands.
mod fail_open;
/// Top-level shell-script parser producing the typed `ParsedShellScript`.
mod parser;
/// Cheap AST scaffold for the bash-like subset the parser understands.
mod shell_ast;
/// Cross-cutting helpers (line walking, scope tracking, dead-branch elimination).
mod support;
/// Public type re-exports from `hook-shell-parser-types`.
#[cfg(feature = "api")]
pub mod types;

#[cfg(feature = "api")]
pub use parser::parse_script;
