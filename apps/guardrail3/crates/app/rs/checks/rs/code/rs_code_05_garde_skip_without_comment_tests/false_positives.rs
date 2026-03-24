use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_documented_primitive_unvalidatable_and_cross_rule_garde_skip_surfaces() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let documented_field_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let plain_comment_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";
    let primitive_rel = "apps/devctl/crates/ports/outbound/traits/src/lib.rs";
    let unvalidatable_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let subcommand_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let documented_type_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";
    let primitive_type_rel = "apps/devctl/crates/app/core/src/lib.rs";

    let documented_field_content =
        std::fs::read_to_string(root.join(documented_field_rel)).expect("read documented field");
    let plain_comment_content =
        std::fs::read_to_string(root.join(plain_comment_rel)).expect("read plain comment");
    let primitive_content =
        std::fs::read_to_string(root.join(primitive_rel)).expect("read primitive");
    let unvalidatable_content =
        std::fs::read_to_string(root.join(unvalidatable_rel)).expect("read unvalidatable");
    let subcommand_content =
        std::fs::read_to_string(root.join(subcommand_rel)).expect("read subcommand");
    let documented_type_content =
        std::fs::read_to_string(root.join(documented_type_rel)).expect("read documented type");
    let primitive_type_content =
        std::fs::read_to_string(root.join(primitive_type_rel)).expect("read primitive type");

    write_file(
        root,
        documented_field_rel,
        &format!(
            "{documented_field_content}\nstruct DocumentedSkipProbe {{\n    #[garde(skip)] // reason: validated in outer workflow\n    field: String,\n}}\n"
        ),
    );
    let plain_comment_new = format!(
        "{plain_comment_content}\nstruct PlainCommentSkipProbe {{\n    #[garde(skip)] // validated elsewhere\n    field: String,\n}}\n"
    );
    write_file(root, plain_comment_rel, &plain_comment_new);
    write_file(
        root,
        primitive_rel,
        &format!(
            "{primitive_content}\nstruct PrimitiveSkipProbe {{\n    #[garde(skip)]\n    count: usize,\n}}\n"
        ),
    );
    write_file(
        root,
        unvalidatable_rel,
        &format!(
            "{unvalidatable_content}\nstruct UnvalidatableSkipProbe {{\n    #[garde(skip)]\n    tags: std::collections::HashMap<String, String>,\n}}\n"
        ),
    );
    write_file(
        root,
        subcommand_rel,
        &format!(
            "{subcommand_content}\nenum CommandMode {{\n    Sync,\n}}\nstruct SubcommandSkipProbe {{\n    #[command(subcommand)]\n    #[garde(skip)]\n    mode: CommandMode,\n}}\n"
        ),
    );
    let documented_type_new = format!(
        "{documented_type_content}\n#[garde(skip)] // reason: external validation envelope\nstruct DocumentedWholeTypeSkipProbe {{\n    payload: String,\n}}\n#[garde(skip)] // validated elsewhere\nstruct PlainCommentWholeTypeSkipProbe {{\n    payload: String,\n}}\n"
    );
    write_file(root, documented_type_rel, &documented_type_new);
    write_file(
        root,
        primitive_type_rel,
        &format!(
            "{primitive_type_content}\n#[garde(skip)]\nstruct PrimitiveWholeTypeSkipProbe {{\n    count: usize,\n    enabled: bool,\n}}\n"
        ),
    );

    let plain_comment_line = plain_comment_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // validated elsewhere"))
        .expect("plain comment line")
        + 1;
    let plain_comment_type_line = documented_type_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // validated elsewhere"))
        .expect("plain comment type line")
        + 1;
    let results = run_family(root);
    let rs_code_05_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-05")
        .collect::<Vec<_>>();
    let mut rs_code_06_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-06")
        .map(|result| {
            (
                result.file.clone().expect("file"),
                result.line,
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
            )
        })
        .collect::<Vec<_>>();
    rs_code_06_results.sort();

    assert_eq!(files_for_rule(&results, "RS-CODE-05"), BTreeSet::new());
    assert!(rs_code_05_results.is_empty());
    assert_eq!(
        files_for_rule(&results, "RS-CODE-06"),
        BTreeSet::from([plain_comment_rel.to_owned(), documented_type_rel.to_owned()])
    );
    assert_eq!(
        rs_code_06_results,
        vec![
            (
                documented_type_rel.to_owned(),
                Some(plain_comment_type_line),
                format!("{:?}", Severity::Error),
                "garde(skip) comment missing reason".to_owned(),
                "`#[garde(skip)]` on non-primitive type `PlainCommentWholeTypeSkipProbe` needs `// reason:`."
                    .to_owned(),
            ),
            (
                plain_comment_rel.to_owned(),
                Some(plain_comment_line),
                format!("{:?}", Severity::Error),
                "garde(skip) comment missing reason".to_owned(),
                "`#[garde(skip)]` on non-primitive field `field: String` needs `// reason:`."
                    .to_owned(),
            ),
        ]
    );
}
