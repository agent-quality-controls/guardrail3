use backend_mcp_domain_protocol::{ToolRequest, ToolResponse};

pub fn handle_request(request: ToolRequest) -> ToolResponse {
    ToolResponse {
        tool_name: request.tool_name.clone(),
        summary: format!(
            "household={} tool={} accepted",
            request.household_slug, request.tool_name
        ),
    }
}
