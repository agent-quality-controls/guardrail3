/// Channel and components rule.
mod channel_and_components;
/// MSRV consistency rule.
mod msrv_consistency;
/// Top-level runtime entry point.
mod run;

#[cfg(feature = "checks")]
pub use run::check;
