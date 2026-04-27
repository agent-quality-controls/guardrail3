mod allow_git_inventory;
mod allow_registry_baseline;
mod ignore_accumulation;
mod ignore_hygiene;
mod run;
mod skip_hygiene;
mod unknown_keys;
mod unknown_sources_policy;

pub(crate) use run::check;
