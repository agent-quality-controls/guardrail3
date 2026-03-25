use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_weak_public_result_error_types_in_library_profile_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");

    let mutated = format!(
        "{package_content}\n\npub fn parse_shared_slug() -> Result<TenantSlug, String> {{\n    Err(\"missing tenant\".to_owned())\n}}\n"
    );
    write_file(root, package_rel, &mutated);

    let results = run_family(root);
    let weak_line = mutated
        .lines()
        .position(|line| line.contains("pub fn parse_shared_slug()"))
        .expect("weak public result line")
        + 1;
    let rs_code_25_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-25")
        .map(|result| {
            (
                result.file.clone(),
                result.line,
                result.severity,
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-25"),
        BTreeSet::from([package_rel.to_owned()])
    );
    assert_eq!(
        rs_code_25_results,
        vec![(
            Some(package_rel.to_owned()),
            Some(weak_line),
            Severity::Warn,
            "weak public error type".to_owned(),
            "Public function `parse_shared_slug` returns `Result<_, String>`. Use a typed error instead."
                .to_owned(),
            false,
        )]
    );
}
