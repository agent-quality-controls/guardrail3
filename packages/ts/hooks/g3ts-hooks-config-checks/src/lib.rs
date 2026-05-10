/// Top-level orchestration for hooks config checks.
#[cfg(feature = "api")]
mod run;

#[cfg(feature = "api")]
pub use run::check;
