mod package_rules;
mod policy_rules;
mod rule_wiring;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"]
mod lib_tests;
