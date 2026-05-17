//! Runtime rules for the `g3rs-deps` family of file-tree checks.

/// Rule that asserts `Cargo.lock` is committed at the workspace root.
mod cargo_lock_present;
/// Rule that asserts `.gitignore` files do not mask `Cargo.lock`.
mod gitignore_not_ignoring_cargo_lock;
/// Family entry point that runs all `g3rs-deps` file-tree rules.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
