use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_05_garde_skip_without_comment::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn detects_non_exempt_garde_skips_without_comments_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let type_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";
    let field_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let vec_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let alias_rel = "apps/worker/crates/adapters/outbound/slack/src/lib.rs";
    let option_alias_rel = "apps/backend/crates/app/commands/src/lib.rs";

    let type_content = test_support::read_file(root, type_rel);
    let field_content = test_support::read_file(root, field_rel);
    let vec_content = test_support::read_file(root, vec_rel);
    let alias_content = test_support::read_file(root, alias_rel);
    let option_alias_content = test_support::read_file(root, option_alias_rel);

    let type_new = format!(
        "{type_content}\n#[garde(skip)]\nstruct WholeTypeSkipProbe {{\n    plan: String,\n}}\n"
    );
    let field_new = format!(
        "{field_content}\nstruct FieldSkipProbe {{\n    #[garde(skip)]\n    field: String,\n}}\n"
    );
    let vec_new = format!(
        "{vec_content}\nstruct VecSkipProbe {{\n    #[garde(skip)]\n    items: Vec<String>,\n}}\n"
    );
    let alias_new = format!(
        "{alias_content}\nstruct UserMap;\nstruct AliasSkipProbe {{\n    #[garde(skip)]\n    field: UserMap,\n}}\n"
    );
    let option_alias_new = format!(
        "{option_alias_content}\nstruct UserMap;\nstruct OptionAliasSkipProbe {{\n    #[garde(skip)]\n    field: Option<UserMap>,\n}}\n"
    );

    write_file(root, type_rel, &type_new);
    write_file(root, field_rel, &field_new);
    write_file(root, vec_rel, &vec_new);
    write_file(root, alias_rel, &alias_new);
    write_file(root, option_alias_rel, &option_alias_new);

    let type_line = type_new
        .lines()
        .position(|line| line.contains("#[garde(skip)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let field_line = field_new
        .lines()
        .position(|line| line.contains("#[garde(skip)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let vec_line = vec_new
        .lines()
        .position(|line| line.contains("#[garde(skip)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let alias_line = alias_new
        .lines()
        .position(|line| line.contains("#[garde(skip)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let option_alias_line = option_alias_new
        .lines()
        .position(|line| line.contains("#[garde(skip)]"))
        .map(|index| index + 1)
        .unwrap_or_default();

    let results = run_family(root);

    assert_files(
        &results,
        BTreeSet::from([
            type_rel.to_owned(),
            field_rel.to_owned(),
            vec_rel.to_owned(),
            alias_rel.to_owned(),
            option_alias_rel.to_owned(),
        ]),
    );
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) without comment",
                message: "`#[garde(skip)]` on non-exempt type `WholeTypeSkipProbe` requires documentation.",
                file: Some(type_rel),
                line: Some(type_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) without comment",
                message: "`#[garde(skip)]` on non-exempt field `field: String` requires documentation.",
                file: Some(field_rel),
                line: Some(field_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) without comment",
                message: "`#[garde(skip)]` on non-exempt field `items: Vec<String>` requires documentation.",
                file: Some(vec_rel),
                line: Some(vec_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) without comment",
                message: "`#[garde(skip)]` on non-exempt field `field: UserMap` requires documentation.",
                file: Some(alias_rel),
                line: Some(alias_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "garde(skip) without comment",
                message: "`#[garde(skip)]` on non-exempt field `field: Option<UserMap>` requires documentation.",
                file: Some(option_alias_rel),
                line: Some(option_alias_line),
                inventory: false,
            },
        ],
    );
}
