/// Rule implementation for `allow git inventory`.
mod allow_git_inventory;
/// Rule implementation for `allow registry baseline`.
mod allow_registry_baseline;
/// Rule implementation for `ignore accumulation`.
mod ignore_accumulation;
/// Rule implementation for `ignore hygiene`.
mod ignore_hygiene;
/// Family entry point that runs all rules.
mod run;
/// Rule implementation for `skip hygiene`.
mod skip_hygiene;
/// Rule implementation for `unknown keys`.
mod unknown_keys;
/// Rule implementation for `unknown sources policy`.
mod unknown_sources_policy;

pub(crate) use run::check;
