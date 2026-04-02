use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_06_garde_skip_with_comment::{
    RuleFinding, Severity, assert_findings,
};
use test_support::write_file;

#[test]
fn reports_reasoned_non_exempt_and_skips_exempt_garde_skip_surfaces() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let reasoned_field_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let uppercase_reason_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";
    let tight_reason_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let missing_comment_rel = "apps/devctl/crates/ports/outbound/traits/src/lib.rs";
    let primitive_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let exempt_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let option_map_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let reference_rel = "apps/worker/crates/adapters/outbound/slack/src/lib.rs";
    let trait_object_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let subcommand_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";
    let reasoned_type_rel = "apps/devctl/crates/app/core/src/lib.rs";

    let reasoned_field_content = test_support::read_file(root, reasoned_field_rel);
    let uppercase_reason_content = test_support::read_file(root, uppercase_reason_rel);
    let tight_reason_content = test_support::read_file(root, tight_reason_rel);
    let missing_comment_content = test_support::read_file(root, missing_comment_rel);
    let primitive_content = test_support::read_file(root, primitive_rel);
    let exempt_content = test_support::read_file(root, exempt_rel);
    let option_map_content = test_support::read_file(root, option_map_rel);
    let reference_content = test_support::read_file(root, reference_rel);
    let trait_object_content = test_support::read_file(root, trait_object_rel);
    let subcommand_content = test_support::read_file(root, subcommand_rel);
    let reasoned_type_content = test_support::read_file(root, reasoned_type_rel);

    let reasoned_field_new = format!(
        "{reasoned_field_content}\nstruct ReasonedSkipProbe {{\n    #[garde(skip)] // reason: validated in outer workflow\n    field: String,\n}}\n"
    );
    let uppercase_reason_new = format!(
        "{uppercase_reason_content}\nstruct UppercaseReasonSkipProbe {{\n    #[garde(skip)] // reason: compatibility boundary\n    field: String,\n}}\n"
    );
    let tight_reason_new = format!(
        "{tight_reason_content}\nstruct TightReasonSkipProbe {{\n    #[garde(skip)] // reason: compatibility boundary\n    field: String,\n}}\n"
    );
    write_file(root, reasoned_field_rel, &reasoned_field_new);
    write_file(root, uppercase_reason_rel, &uppercase_reason_new);
    write_file(root, tight_reason_rel, &tight_reason_new);
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
        exempt_rel,
        &format!(
            "{exempt_content}\nstruct ExplicitMapCommentSkipProbe {{\n    #[garde(skip)] // temporary bypass\n    tags: std::collections::HashMap<String, String>,\n}}\n"
        ),
    );
    write_file(
        root,
        option_map_rel,
        &format!(
            "{option_map_content}\nstruct OptionMapCommentSkipProbe {{\n    #[garde(skip)] // temporary bypass\n    tags: Option<std::collections::HashMap<String, String>>,\n}}\n"
        ),
    );
    write_file(
        root,
        reference_rel,
        &format!(
            "{reference_content}\nstruct ReferenceCommentSkipProbe {{\n    #[garde(skip)] // temporary bypass\n    label: &'static str,\n}}\n"
        ),
    );
    write_file(
        root,
        trait_object_rel,
        &format!(
            "{trait_object_content}\ntrait Handler {{}}\nstruct TraitObjectCommentSkipProbe {{\n    #[garde(skip)] // temporary bypass\n    handler: Box<dyn Handler>,\n}}\n"
        ),
    );
    write_file(
        root,
        subcommand_rel,
        &format!(
            "{subcommand_content}\nenum CommandMode {{\n    Sync,\n}}\nstruct SubcommandCommentSkipProbe {{\n    #[command(subcommand)]\n    #[garde(skip)] // temporary bypass\n    mode: CommandMode,\n}}\n"
        ),
    );
    let reasoned_type_new = format!(
        "{reasoned_type_content}\n#[garde(skip)] // reason: external validation envelope\nstruct ReasonedWholeTypeSkipProbe {{\n    payload: String,\n}}\n#[garde(skip)]\nstruct PrimitiveWholeTypeSkipProbe {{\n    count: usize,\n    enabled: bool,\n}}\n#[garde(skip)] // temporary bypass\nstruct PrimitiveWholeTypeCommentProbe {{\n    count: usize,\n    enabled: bool,\n}}\n#[garde(skip)] // temporary bypass\nstruct UnvalidatableWholeTypeCommentProbe {{\n    tags: std::collections::HashMap<String, String>,\n}}\n"
    );
    write_file(root, reasoned_type_rel, &reasoned_type_new);

    let reasoned_field_line = reasoned_field_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // reason: validated in outer workflow"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let uppercase_reason_line = uppercase_reason_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // reason: compatibility boundary"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let tight_reason_line = tight_reason_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // reason: compatibility boundary"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let reasoned_type_line = reasoned_type_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // reason: external validation envelope"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_findings(
        &run_family(root),
        &[
            RuleFinding::new(
                Severity::Warn,
                "garde(skip) with reason",
                "`#[garde(skip)]` on non-exempt field `field: String` reason: validated in outer workflow",
                Some(reasoned_field_rel),
                Some(reasoned_field_line),
                false,
            ),
            RuleFinding::new(
                Severity::Warn,
                "garde(skip) with reason",
                "`#[garde(skip)]` on non-exempt field `field: String` reason: compatibility boundary",
                Some(uppercase_reason_rel),
                Some(uppercase_reason_line),
                false,
            ),
            RuleFinding::new(
                Severity::Warn,
                "garde(skip) with reason",
                "`#[garde(skip)]` on non-exempt field `field: String` reason: compatibility boundary",
                Some(tight_reason_rel),
                Some(tight_reason_line),
                false,
            ),
            RuleFinding::new(
                Severity::Warn,
                "garde(skip) with reason",
                "`#[garde(skip)]` on non-exempt type `ReasonedWholeTypeSkipProbe` reason: external validation envelope",
                Some(reasoned_type_rel),
                Some(reasoned_type_line),
                false,
            ),
        ],
    );
}
