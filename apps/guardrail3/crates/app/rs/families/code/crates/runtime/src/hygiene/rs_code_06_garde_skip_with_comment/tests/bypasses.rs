use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_06_garde_skip_with_comment::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn detects_non_exempt_garde_skips_with_plain_comments_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let type_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let field_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let empty_reason_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let wrong_key_rel = "apps/backend/crates/ports/outbound/events/src/lib.rs";
    let vec_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let alias_rel = "apps/worker/crates/adapters/outbound/slack/src/lib.rs";
    let option_alias_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let weak_reason_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";

    let type_content = test_support::read_file(root, type_rel);
    let field_content = test_support::read_file(root, field_rel);
    let empty_reason_content = test_support::read_file(root, empty_reason_rel);
    let wrong_key_content = test_support::read_file(root, wrong_key_rel);
    let vec_content = test_support::read_file(root, vec_rel);
    let alias_content = test_support::read_file(root, alias_rel);
    let option_alias_content = test_support::read_file(root, option_alias_rel);
    let weak_reason_content = test_support::read_file(root, weak_reason_rel);

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
    let alias_new = format!(
        "{alias_content}\nstruct UserMap;\nstruct AliasCommentProbe {{\n    #[garde(skip)] // validated elsewhere\n    field: UserMap,\n}}\n"
    );
    let option_alias_new = format!(
        "{option_alias_content}\nstruct UserMap;\nstruct OptionAliasCommentProbe {{\n    #[garde(skip)] // temporary bypass\n    field: Option<UserMap>,\n}}\n"
    );
    let weak_reason_new = format!(
        "{weak_reason_content}\nstruct WeakReasonSkipProbe {{\n    #[garde(skip)] // reason: temp\n    field: String,\n}}\n"
    );

    write_file(root, type_rel, &type_new);
    write_file(root, field_rel, &field_new);
    write_file(root, empty_reason_rel, &empty_reason_new);
    write_file(root, wrong_key_rel, &wrong_key_new);
    write_file(root, vec_rel, &vec_new);
    write_file(root, alias_rel, &alias_new);
    write_file(root, option_alias_rel, &option_alias_new);
    write_file(root, weak_reason_rel, &weak_reason_new);

    let type_line = type_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // validated elsewhere"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let field_line = field_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // validated elsewhere"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let empty_reason_line = empty_reason_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // reason:"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let wrong_key_line = wrong_key_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // because: external validation envelope"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let vec_line = vec_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // temporary bypass"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let alias_line = alias_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // validated elsewhere"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let option_alias_line = option_alias_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // temporary bypass"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let weak_reason_line = weak_reason_new
        .lines()
        .position(|line| line.contains("#[garde(skip)] // reason: temp"))
        .map(|index| index + 1)
        .unwrap_or_default();

    let results = run_family(root);

    assert_files(
        &results,
        BTreeSet::from([
            type_rel.to_owned(),
            field_rel.to_owned(),
            empty_reason_rel.to_owned(),
            wrong_key_rel.to_owned(),
            vec_rel.to_owned(),
            alias_rel.to_owned(),
            option_alias_rel.to_owned(),
            weak_reason_rel.to_owned(),
        ]),
    );
    assert_findings(
        &results,
        &[
            RuleFinding::new(
                Severity::Error,
                "garde(skip) comment missing reason",
                "`#[garde(skip)]` on non-exempt type `WholeTypeCommentProbe` needs `// reason:`.",
                Some(type_rel),
                Some(type_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "garde(skip) comment missing reason",
                "`#[garde(skip)]` on non-exempt field `field: String` needs `// reason:`.",
                Some(field_rel),
                Some(field_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "garde(skip) comment missing reason",
                "`#[garde(skip)]` on non-exempt type `WrongKeyWholeTypeCommentProbe` needs `// reason:`.",
                Some(wrong_key_rel),
                Some(wrong_key_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "garde(skip) comment missing reason",
                "`#[garde(skip)]` on non-exempt field `field: String` needs `// reason:`.",
                Some(empty_reason_rel),
                Some(empty_reason_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "garde(skip) comment missing reason",
                "`#[garde(skip)]` on non-exempt field `items: Vec<String>` needs `// reason:`.",
                Some(vec_rel),
                Some(vec_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "garde(skip) comment missing reason",
                "`#[garde(skip)]` on non-exempt field `field: UserMap` needs `// reason:`.",
                Some(alias_rel),
                Some(alias_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "garde(skip) comment missing reason",
                "`#[garde(skip)]` on non-exempt field `field: Option<UserMap>` needs `// reason:`.",
                Some(option_alias_rel),
                Some(option_alias_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "garde(skip) reason too weak",
                "`#[garde(skip)]` on non-exempt field `field: String` reason must be specific and at least two words. Weak reason `temp` found.",
                Some(weak_reason_rel),
                Some(weak_reason_line),
                false,
            ),
        ],
    );
}

#[test]
fn detects_non_exempt_garde_skip_with_block_comment() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let new_content = format!(
        "{content}\nstruct BlockCommentSkipProbe {{\n    #[garde(skip)] /* validated elsewhere */\n    field: String,\n}}\n"
    );
    write_file(root, rel, &new_content);

    let line = new_content
        .lines()
        .position(|entry| entry.contains("#[garde(skip)] /* validated elsewhere */"))
        .map(|index| index + 1)
        .unwrap_or_default();

    let results = run_family(root);

    assert_files(&results, BTreeSet::from([rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding::new(
            Severity::Error,
            "garde(skip) comment missing reason",
            "`#[garde(skip)]` on non-exempt field `field: String` needs `// reason:`.",
            Some(rel),
            Some(line),
            false,
        )],
    );
}
