/// Rule: additional clippy disallowed-method bans required by garde policy.
mod additional_method_bans;
/// Rule: core clippy disallowed-method bans required by garde policy.
mod core_method_bans;
/// Rule: garde dependency must be present in the manifest.
mod dependency_present;
/// Rule: clippy disallowed-types bans on extractor types.
mod extractor_type_bans;
/// Rule: ban `reqwest::Response::json` and friends.
mod reqwest_json_ban;
/// Family runner that dispatches to per-rule check modules.
mod run;
/// Shared helpers for matching clippy ban configurations.
mod support;

#[cfg(feature = "checks")]
pub use run::check;
