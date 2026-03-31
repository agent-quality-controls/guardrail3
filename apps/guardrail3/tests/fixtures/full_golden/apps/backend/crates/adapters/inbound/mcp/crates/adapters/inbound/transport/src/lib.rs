use backend_mcp_app_handlers::handle_request;

pub fn serve_tool(tool_name: &str, household_slug: &str) -> String {
    let response = handle_request(backend_mcp_domain_protocol::ToolRequest {
        tool_name: tool_name.to_owned(),
        household_slug: household_slug.to_owned(),
    });
    response.summary
}
