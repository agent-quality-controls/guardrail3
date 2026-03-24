use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn detects_non_primitive_garde_skips_without_comments_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let type_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let field_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let vec_rel = "apps/devctl/crates/app/core/src/lib.rs";

    let type_content = std::fs::read_to_string(root.join(type_rel)).expect("read type file");
    let field_content = std::fs::read_to_string(root.join(field_rel)).expect("read field file");
    let vec_content = std::fs::read_to_string(root.join(vec_rel)).expect("read vec file");

    let type_new = format!(
        "{type_content}\n#[garde(skip)]\nstruct WholeTypeSkipProbe {{\n    plan: String,\n}}\n"
    );
    let field_new = format!(
        "{field_content}\nstruct FieldSkipProbe {{\n    #[garde(skip)]\n    field: String,\n}}\n"
    );
    let vec_new = format!(
        "{vec_content}\nstruct VecSkipProbe {{\n    #[garde(skip)]\n    items: Vec<String>,\n}}\n"
    );

    write_file(root, type_rel, &type_new);
    write_file(root, field_rel, &field_new);
    write_file(root, vec_rel, &vec_new);

    let type_line = type_new
        .lines()
        .position(|line| line.contains("#[garde(skip)]"))
        .expect("type line")
        + 1;
    let field_line = field_new
        .lines()
        .position(|line| line.contains("#[garde(skip)]"))
        .expect("field line")
        + 1;
    let vec_line = vec_new
        .lines()
        .position(|line| line.contains("#[garde(skip)]"))
        .expect("vec line")
        + 1;

    let results = run_family(root);
    let mut rs_code_05_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-05")
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
    rs_code_05_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-05"),
        BTreeSet::from([
            type_rel.to_owned(),
            field_rel.to_owned(),
            vec_rel.to_owned()
        ])
    );
    assert_eq!(
        rs_code_05_results,
        vec![
            (
                type_rel.to_owned(),
                Some(type_line),
                format!("{:?}", Severity::Error),
                "garde(skip) without comment".to_owned(),
                "`#[garde(skip)]` on non-primitive type `WholeTypeSkipProbe` requires documentation."
                    .to_owned(),
            ),
            (
                field_rel.to_owned(),
                Some(field_line),
                format!("{:?}", Severity::Error),
                "garde(skip) without comment".to_owned(),
                "`#[garde(skip)]` on non-primitive field `field: String` requires documentation."
                    .to_owned(),
            ),
            (
                vec_rel.to_owned(),
                Some(vec_line),
                format!("{:?}", Severity::Error),
                "garde(skip) without comment".to_owned(),
                "`#[garde(skip)]` on non-primitive field `items: Vec<String>` requires documentation."
                    .to_owned(),
            ),
        ]
    );
}
