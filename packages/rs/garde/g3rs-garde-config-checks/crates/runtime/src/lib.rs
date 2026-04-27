mod additional_method_bans;
mod core_method_bans;
mod dependency_present;
mod extractor_type_bans;
mod reqwest_json_ban;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
