use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_06_garde_skip_with_comment::{assert_files, assert_findings, RuleFinding};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

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

    assert_files(&results, BTreeSet::from([
            type_rel.to_owned(),
            field_rel.to_owned(),
            empty_reason_rel.to_owned(),
            wrong_key_rel.to_owned(),
            vec_rel.to_owned()
        ]));
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) comment missing reason",
                message: "`#[garde(skip)]` on non-primitive type `WholeTypeCommentProbe` needs `// reason:`.",
                file: Some(type_rel),
                line: Some(type_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) comment missing reason",
                message: "`#[garde(skip)]` on non-primitive field `field: String` needs `// reason:`.",
                file: Some(field_rel),
                line: Some(field_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) comment missing reason",
                message: "`#[garde(skip)]` on non-primitive type `WrongKeyWholeTypeCommentProbe` needs `// reason:`.",
                file: Some(wrong_key_rel),
                line: Some(wrong_key_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) comment missing reason",
                message: "`#[garde(skip)]` on non-primitive field `field: String` needs `// reason:`.",
                file: Some(empty_reason_rel),
                line: Some(empty_reason_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) comment missing reason",
                message: "`#[garde(skip)]` on non-primitive field `items: Vec<String>` needs `// reason:`.",
                file: Some(vec_rel),
                line: Some(vec_line),
                inventory: false,
            },
        ],
    );
}
