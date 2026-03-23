use backend_mcp_app_handlers as _;
use backend_mcp_domain_protocol as _;

fn main() {
    println!(
        "{}",
        backend_mcp_adapters_inbound_transport::serve_tool("plan_household", "acme")
    );
}
