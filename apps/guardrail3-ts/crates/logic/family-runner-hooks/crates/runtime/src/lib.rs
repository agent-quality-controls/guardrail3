/// Runs the TS hook family.
mod run;

#[cfg(feature = "api")]
pub use run::run;
