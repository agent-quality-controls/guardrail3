#[path = "golden.rs"] // reason: test matrix sidecar split by scenario
mod golden;
#[path = "ignored_unrelated_nested.rs"] // reason: test matrix sidecar split by scenario
mod ignored_unrelated_nested;
#[path = "legacy_config.rs"] // reason: test matrix sidecar split by scenario
mod legacy_config;
#[path = "malformed.rs"] // reason: test matrix sidecar split by scenario
mod malformed;
#[path = "member_root.rs"] // reason: test matrix sidecar split by scenario
mod member_root;
#[path = "missing_content.rs"] // reason: test matrix sidecar split by scenario
mod missing_content;
#[path = "nested_root.rs"] // reason: test matrix sidecar split by scenario
mod nested_root;
