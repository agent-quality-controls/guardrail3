use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_06_garde_skip_with_comment::{assert_files, assert_findings, assert_no_hits, RuleFinding};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn skips_reasoned_primitive_unvalidatable_and_missing_comment_garde_skip_surfaces() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let reasoned_field_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let uppercase_reason_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";
    let tight_reason_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let missing_comment_rel = "apps/devctl/crates/ports/outbound/traits/src/lib.rs";
    let primitive_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let unvalidatable_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let subcommand_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";
    let reasoned_type_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let block_comment_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";

    let reasoned_field_content =
        std::fs::read_to_string(root.join(reasoned_field_rel)).expect("read reasoned field");
    let uppercase_reason_content =
        std::fs::read_to_string(root.join(uppercase_reason_rel)).expect("read uppercase reason");
    let tight_reason_content =
        std::fs::read_to_string(root.join(tight_reason_rel)).expect("read tight reason");
    let missing_comment_content =
        std::fs::read_to_string(root.join(missing_comment_rel)).expect("read missing comment");
    let primitive_content =
        std::fs::read_to_string(root.join(primitive_rel)).expect("read primitive");
    let unvalidatable_content =
        std::fs::read_to_string(root.join(unvalidatable_rel)).expect("read unvalidatable");
    let subcommand_content =
        std::fs::read_to_string(root.join(subcommand_rel)).expect("read subcommand");
    let reasoned_type_content =
        std::fs::read_to_string(root.join(reasoned_type_rel)).expect("read reasoned type");
    let block_comment_content =
        std::fs::read_to_string(root.join(block_comment_rel)).expect("read block comment");

    write_file(
        root,
        reasoned_field_rel,
        &format!(
            "{reasoned_field_content}\nstruct ReasonedSkipProbe {{\n    #[garde(skip)] // reason: validated in outer workflow\n    field: String,\n}}\n"
        ),
    );
    write_file(
        root,
        uppercase_reason_rel,
        &format!(
            "{uppercase_reason_content}\nstruct UppercaseReasonSkipProbe {{\n    #[garde(skip)] // REASON: compatibility boundary\n    field: String,\n}}\n"
        ),
    );
    write_file(
        root,
        tight_reason_rel,
        &format!(
            "{tight_reason_content}\nstruct TightReasonSkipProbe {{\n    #[garde(skip)] //reason: compatibility boundary\n    field: String,\n}}\n"
        ),
    );
    let missing_comment_new = format!(
        "{missing_comment_content}\nstruct MissingCommentSkipProbe {{\n    #[garde(skip)]\n    field: String,\n}}\n"
    );
    write_file(root, missing_comment_rel, &missing_comment_new);
    write_file(
        root,
        primitive_rel,
        &format!(
            "{primitive_content}\nstruct PrimitiveCommentSkipProbe {{\n    #[garde(skip)] // temporary bypass\n    count: usize,\n}}\n"
        ),
    );
    write_file(
        root,
        unvalidatable_rel,
        &format!(
            "{unvalidatable_content}\nstruct UnvalidatableCommentSkipProbe {{\n    #[garde(skip)] // temporary bypass\n    tags: std::collections::HashMap<String, String>,\n}}\n"
        ),
    );
    write_file(
        root,
        subcommand_rel,
        &format!(
            "{subcommand_content}\nenum CommandMode {{\n    Sync,\n}}\nstruct SubcommandCommentSkipProbe {{\n    #[command(subcommand)]\n    #[garde(skip)] // temporary bypass\n    mode: CommandMode,\n}}\n"
        ),
    );
    write_file(
        root,
        reasoned_type_rel,
        &format!(
            "{reasoned_type_content}\n#[garde(skip)] // reason: external validation envelope\nstruct ReasonedWholeTypeSkipProbe {{\n    payload: String,\n}}\n#[garde(skip)]\nstruct PrimitiveWholeTypeSkipProbe {{\n    count: usize,\n    enabled: bool,\n}}\n#[garde(skip)] // temporary bypass\nstruct PrimitiveWholeTypeCommentProbe {{\n    count: usize,\n    enabled: bool,\n}}\n#[garde(skip)] // temporary bypass\nstruct UnvalidatableWholeTypeCommentProbe {{\n    tags: std::collections::HashMap<String, String>,\n}}\n"
        ),
    );
    let block_comment_new = format!(
        "{block_comment_content}\nstruct BlockCommentSkipProbe {{\n    #[garde(skip)] /* validated elsewhere */\n    field: String,\n}}\n"
    );
    write_file(root, block_comment_rel, &block_comment_new);

    let missing_comment_line = missing_comment_new
        .lines()
        .position(|line| line.contains("#[garde(skip)]"))
        .expect("missing comment line")
        + 1;
    let block_comment_line = block_comment_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] /* validated elsewhere */"))
        .expect("block comment line")
        + 1;

    let results = run_family(root);

    assert_no_hits(&results);
    assert_files(&results, BTreeSet::from([missing_comment_rel.to_owned(), block_comment_rel.to_owned()]));
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) without comment",
                message: "`#[garde(skip)]` on non-primitive field `field: String` requires documentation.",
                file: Some(missing_comment_rel),
                line: Some(missing_comment_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) without comment",
                message: "`#[garde(skip)]` on non-primitive field `field: String` requires documentation.",
                file: Some(block_comment_rel),
                line: Some(block_comment_line),
                inventory: false,
            },
        ],
    );
}
