#[path = "multi_root_local_override.rs"] // reason: test matrix sidecar split by scenario
mod multi_root_local_override;
#[path = "nested_entries.rs"] // reason: test matrix sidecar split by scenario
mod nested_entries;
#[path = "sources_and_bans.rs"] // reason: test matrix sidecar split by scenario
mod sources_and_bans;
#[path = "top_level_and_sections.rs"] // reason: test matrix sidecar split by scenario
mod top_level_and_sections;
#[path = "unsupported_schema.rs"] // reason: test matrix sidecar split by scenario
mod unsupported_schema;
