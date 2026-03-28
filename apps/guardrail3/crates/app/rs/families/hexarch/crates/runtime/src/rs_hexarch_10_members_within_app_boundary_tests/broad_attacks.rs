use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_10_members_within_app_boundary as assertions;

#[test]
fn outside_boundary_workspace_members_hit_every_mutated_app() {
    let tmp = copy_fixture();
    for (app, body) in [
        (
            "devctl",
            "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/app/core\",\n    \"crates/ports/outbound/traits\",\n    \"crates/adapters/inbound/cli\",\n    \"crates/adapters/outbound/fs\",\n    \"../../packages/shared-types\",\n]\nresolver = \"2\"\n",
        ),
        (
            "backend",
            "[workspace]\nmembers = [\n    \"crates/domain/engine\",\n    \"crates/domain/types\",\n    \"crates/app/commands\",\n    \"crates/app/queries\",\n    \"crates/ports/inbound/api\",\n    \"crates/ports/outbound/events\",\n    \"crates/ports/outbound/repo\",\n    \"crates/adapters/inbound/rest\",\n    \"crates/adapters/outbound/postgres\",\n    \"crates/adapters/outbound/queue\",\n    \"../../packages/shared-types\",\n]\nresolver = \"2\"\n",
        ),
        (
            "worker",
            "[workspace]\nmembers = [\n    \"crates/domain/jobs\",\n    \"crates/app/processor\",\n    \"crates/ports/outbound/queue\",\n    \"crates/adapters/inbound/poller\",\n    \"crates/adapters/outbound/db\",\n    \"crates/adapters/outbound/sqs\",\n    \"../../packages/shared-types\",\n]\nresolver = \"2\"\n",
        ),
    ] {
        write_file(tmp.path(), &format!("apps/{app}/Cargo.toml"), body);
    }

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl"),
                file_contains: None,
                title_contains: Some(&["../../packages/shared-types"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/backend"),
                file_contains: None,
                title_contains: Some(&["../../packages/shared-types"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/worker"),
                file_contains: None,
                title_contains: Some(&["../../packages/shared-types"]),
                message_contains: None,
            },
        ],
    );
}
