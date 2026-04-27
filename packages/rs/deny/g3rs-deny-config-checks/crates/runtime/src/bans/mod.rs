mod allow_wildcard_paths;
mod duplicate_entries;
mod extra_feature_bans_inventory;
mod highlight_inventory;
mod multiple_versions_floor;
mod run;
mod tokio_full_ban;
mod wildcards_inventory;

pub(crate) use run::check;
