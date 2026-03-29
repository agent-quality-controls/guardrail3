#[path = "container_type.rs"] // reason: test matrix sidecar split by scenario
mod container_type;
#[path = "malformed.rs"] // reason: test matrix sidecar split by scenario
mod malformed;
#[path = "multi_root_local_override.rs"] // reason: test matrix sidecar split by scenario
mod multi_root_local_override;
#[path = "valid_inventory.rs"] // reason: test matrix sidecar split by scenario
mod valid_inventory;
