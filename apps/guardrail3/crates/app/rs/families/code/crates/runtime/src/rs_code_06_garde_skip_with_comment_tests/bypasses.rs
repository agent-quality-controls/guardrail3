use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn detects_non_primitive_garde_skips_with_plain_comments_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let type_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let field_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let empty_reason_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let wrong_key_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";
    let vec_rel = "apps/devctl/crates/app/core/src/lib.rs";

    let type_content = std::fs::read_to_string(root.join(type_rel)).expect("read type file");
    let field_content = std::fs::read_to_string(root.join(field_rel)).expect("read field file");
    let empty_reason_content =
        std::fs::read_to_string(root.join(empty_reason_rel)).expect("read empty reason file");
    let wrong_key_content =
        std::fs::read_to_string(root.join(wrong_key_rel)).expect("read wrong key file");
    let vec_content = std::fs::read_to_string(root.join(vec_rel)).expect("read vec file");

    let type_new = format!(
        "{type_content}\n#[garde(skip)] // validated elsewhere\nstruct WholeTypeCommentProbe {{\n    plan: String,\n}}\n"
    );
    let field_new = format!(
        "{field_content}\nstruct FieldCommentProbe {{\n    #[garde(skip)] // validated elsewhere\n    field: String,\n}}\n"
    );
    let vec_new = format!(
        "{vec_content}\nstruct VecCommentProbe {{\n    #[garde(skip)] // temporary bypass\n    items: Vec<String>,\n}}\n"
    );
    let empty_reason_new = format!(
        "{empty_reason_content}\nstruct EmptyReasonCommentProbe {{\n    #[garde(skip)] // reason:\n    field: String,\n}}\n"
    );
    let wrong_key_new = format!(
        "{wrong_key_content}\n#[garde(skip)] // because: external validation envelope\nstruct WrongKeyWholeTypeCommentProbe {{\n    payload: String,\n}}\n"
    );

    write_file(root, type_rel, &type_new);
    write_file(root, field_rel, &field_new);
    write_file(root, empty_reason_rel, &empty_reason_new);
    write_file(root, wrong_key_rel, &wrong_key_new);
    write_file(root, vec_rel, &vec_new);

    let type_line = type_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // validated elsewhere"))
        .expect("type line")
        + 1;
    let field_line = field_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // validated elsewhere"))
        .expect("field line")
        + 1;
    let empty_reason_line = empty_reason_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // reason:"))
        .expect("empty reason line")
        + 1;
    let wrong_key_line = wrong_key_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // because: external validation envelope"))
        .expect("wrong key line")
        + 1;
    let vec_line = vec_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // temporary bypass"))
        .expect("vec line")
        + 1;

    let results = run_family(root);
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

    assert_eq!(
        files_for_rule(&results, "RS-CODE-06"),
        BTreeSet::from([
            type_rel.to_owned(),
            field_rel.to_owned(),
            empty_reason_rel.to_owned(),
            wrong_key_rel.to_owned(),
            vec_rel.to_owned()
        ])
    );
    assert_eq!(
        rs_code_06_results,
        vec![
            (
                type_rel.to_owned(),
                Some(type_line),
                format!("{:?}", Severity::Error),
                "garde(skip) comment missing reason".to_owned(),
                "`#[garde(skip)]` on non-primitive type `WholeTypeCommentProbe` needs `// reason:`."
                    .to_owned(),
            ),
            (
                field_rel.to_owned(),
                Some(field_line),
                format!("{:?}", Severity::Error),
                "garde(skip) comment missing reason".to_owned(),
                "`#[garde(skip)]` on non-primitive field `field: String` needs `// reason:`."
                    .to_owned(),
            ),
            (
                wrong_key_rel.to_owned(),
                Some(wrong_key_line),
                format!("{:?}", Severity::Error),
                "garde(skip) comment missing reason".to_owned(),
                "`#[garde(skip)]` on non-primitive type `WrongKeyWholeTypeCommentProbe` needs `// reason:`."
                    .to_owned(),
            ),
            (
                empty_reason_rel.to_owned(),
                Some(empty_reason_line),
                format!("{:?}", Severity::Error),
                "garde(skip) comment missing reason".to_owned(),
                "`#[garde(skip)]` on non-primitive field `field: String` needs `// reason:`."
                    .to_owned(),
            ),
            (
                vec_rel.to_owned(),
                Some(vec_line),
                format!("{:?}", Severity::Error),
                "garde(skip) comment missing reason".to_owned(),
                "`#[garde(skip)]` on non-primitive field `items: Vec<String>` needs `// reason:`."
                    .to_owned(),
            ),
        ]
    );
}
