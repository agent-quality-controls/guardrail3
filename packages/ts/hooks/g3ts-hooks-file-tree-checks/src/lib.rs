/// Top-level file-tree check entry point.
#[cfg(feature = "api")]
mod run;

#[cfg(feature = "api")]
pub use run::check;
