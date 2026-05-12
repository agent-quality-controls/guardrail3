/// Rule implementation for `confidence threshold`.
mod confidence_threshold;
/// Rule implementation for `copyleft allowlist`.
mod copyleft_allowlist;
/// Rule implementation for `license allow baseline`.
mod license_allow_baseline;
/// Family entry point that runs all rules.
mod run;

pub(crate) use run::check;
