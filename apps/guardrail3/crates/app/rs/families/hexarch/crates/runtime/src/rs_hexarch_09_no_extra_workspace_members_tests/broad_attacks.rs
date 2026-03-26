use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_09_no_extra_workspace_members as assertions;
use super::{copy_fixture, write_file};

#[test]
fn phantom_workspace_members_hit_every_mutated_app() {
    let tmp = copy_fixture();
    for (app, body) in [
        (
            "devctl",
            "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/app/core\",\n    \"crates/ports/outbound/traits\",\n    \"crates/adapters/inbound/cli\",\n    \"crates/adapters/outbound/fs\",\n    \"crates/domain/phantom\",\n]\nresolver = \"2\"\n",
        ),
        (
            "backend",
            "[workspace]\nmembers = [\n    \"crates/domain/engine\",\n    \"crates/domain/types\",\n    \"crates/app/commands\",\n    \"crates/app/queries\",\n    \"crates/ports/inbound/api\",\n    \"crates/ports/outbound/events\",\n    \"crates/ports/outbound/repo\",\n    \"crates/adapters/inbound/rest\",\n    \"crates/adapters/outbound/postgres\",\n    \"crates/adapters/outbound/queue\",\n    \"crates/domain/phantom\",\n]\nresolver = \"2\"\n",
        ),
        (
            "worker",
            "[workspace]\nmembers = [\n    \"crates/domain/jobs\",\n    \"crates/app/processor\",\n    \"crates/ports/outbound/queue\",\n    \"crates/adapters/inbound/poller\",\n    \"crates/adapters/outbound/db\",\n    \"crates/adapters/outbound/sqs\",\n    \"crates/domain/phantom\",\n]\nresolver = \"2\"\n",
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
                title_contains: Some(&["crates/domain/phantom"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/backend"),
                file_contains: None,
                title_contains: Some(&["crates/domain/phantom"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/worker"),
                file_contains: None,
                title_contains: Some(&["crates/domain/phantom"]),
                message_contains: None,
            },
        ],
    );
}
