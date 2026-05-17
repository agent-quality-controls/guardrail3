/// Rule implementation for `allow wildcard paths`.
mod allow_wildcard_paths;
/// Rule implementation for `duplicate entries`.
mod duplicate_entries;
/// Rule implementation for `extra feature bans inventory`.
mod extra_feature_bans_inventory;
/// Rule implementation for `highlight inventory`.
mod highlight_inventory;
/// Rule implementation for `multiple versions floor`.
mod multiple_versions_floor;
/// Family entry point that runs all rules.
mod run;
/// Rule implementation for `tokio full ban`.
mod tokio_full_ban;
/// Rule implementation for `wildcards inventory`.
mod wildcards_inventory;

pub(crate) use run::check;
