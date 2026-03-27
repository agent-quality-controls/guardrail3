use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn inventories_large_traits_in_real_library_profile_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");

    let mut warn_methods = String::new();
    for index in 0..9 {
        warn_methods.push_str(&format!("    fn warn_{index}(&self);\n"));
    }
    let mut error_methods = String::new();
    for index in 0..13 {
        error_methods.push_str(&format!("    fn error_{index}(&self);\n"));
    }

    let mutated = format!(
        "{package_content}\n\npub trait SharedSurface {{\n{warn_methods}}}\n\npub trait OversizedSurface {{\n{error_methods}}}\n"
    );
    write_file(root, package_rel, &mutated);

    let results = run_family(root);
    let warn_line = mutated
        .lines()
        .position(|line| line.contains("pub trait SharedSurface"))
        .expect("warn trait line")
        + 1;
    let error_line = mutated
        .lines()
        .position(|line| line.contains("pub trait OversizedSurface"))
        .expect("error trait line")
        + 1;
    let mut rs_code_29_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-29")
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
    rs_code_29_results.sort_by_key(|(file, line, severity, _, _, _)| {
        (
            file.clone().unwrap_or_default(),
            *line,
            format!("{severity:?}"),
        )
    });

    assert_eq!(
        files_for_rule(&results, "RS-CODE-29"),
        BTreeSet::from([package_rel.to_owned()])
    );
    assert_eq!(
        rs_code_29_results,
        vec![
            (
                Some(package_rel.to_owned()),
                Some(warn_line),
                Severity::Warn,
                "large trait surface".to_owned(),
                "Trait `SharedSurface` has 9 methods (warn above 8, error above 12).".to_owned(),
                false,
            ),
            (
                Some(package_rel.to_owned()),
                Some(error_line),
                Severity::Error,
                "large trait surface".to_owned(),
                "Trait `OversizedSurface` has 13 methods (warn above 8, error above 12)."
                    .to_owned(),
                false,
            ),
        ]
    );
}
