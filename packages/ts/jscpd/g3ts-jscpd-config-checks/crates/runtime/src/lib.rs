/// Checks that the `absolute` flag is set to `true` in the root jscpd config.
mod absolute_true;
/// Checks the configured detection format and emits an inventory entry.
mod format_and_inventory;
/// Checks that required ignore globs are declared in the root jscpd config.
mod required_ignores;
/// Checks that a root `.jscpd.json` file is present.
mod root_exists;
/// Checks that the root `.jscpd.json` file is parseable.
mod root_parseable;
/// Orchestrates the jscpd config rule fan-out.
mod run;
/// Shared accessor helpers for the rule implementations.
mod support;
/// Checks that the `threshold` value is set to `0`.
mod threshold_zero;

#[cfg(feature = "checks")]
pub use run::check;
