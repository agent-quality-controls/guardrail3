/// Internal module `package_rules`.
mod package_rules;
/// Internal module `policy_rules`.
mod policy_rules;
/// Internal module `rule_wiring`.
mod rule_wiring;
/// Internal module `run`.
mod run;
/// Internal module `support`.
mod support;

#[cfg(feature = "checks")]
pub use run::check;
