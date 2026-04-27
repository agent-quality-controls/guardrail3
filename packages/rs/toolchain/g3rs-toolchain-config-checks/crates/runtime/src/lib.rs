mod channel_and_components;
mod msrv_consistency;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
