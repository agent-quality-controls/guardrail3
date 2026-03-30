#[path = "allowed_roots.rs"] // reason: test matrix sidecar split by scenario
mod allowed_roots;
#[path = "allowed_roots_dotfile.rs"] // reason: test matrix sidecar split by scenario
mod allowed_roots_dotfile;
#[path = "forbidden_locations.rs"] // reason: test matrix sidecar split by scenario
mod forbidden_locations;
#[path = "same_root_precedence.rs"] // reason: test matrix sidecar split by scenario
mod same_root_precedence;
#[path = "same_root_precedence_nested.rs"] // reason: test matrix sidecar split by scenario
mod same_root_precedence_nested;
#[path = "unparseable_routed_root.rs"] // reason: test matrix sidecar split by scenario
mod unparseable_routed_root;
