//! Runtime for g3ts `tsconfig.json` config-check rules.

/// Rule: tsconfig file must exist and be readable.
mod exists;
/// Rule: tsconfig `extends` chain must resolve to a valid base config.
mod extends_chain_resolves;
/// Rule: tsconfig must either extend a base or define inline strict flags.
mod extends_or_inline;
/// Rule: tsconfig must be parseable.
mod parseable;
/// Top-level check dispatch.
mod run;
/// Rule: tsconfig must enable the strict baseline (per-flag table).
mod strict_baseline;
/// Shared support utilities (strict flag descriptors, finding builders).
mod support;

#[cfg(feature = "checks")]
pub use run::check;
