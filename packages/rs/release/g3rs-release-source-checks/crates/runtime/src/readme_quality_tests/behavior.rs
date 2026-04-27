use g3rs_release_source_checks_assertions::readme_quality as assertions;

use super::helpers;

#[test]
fn errors_when_readme_is_too_short() {
    let results = helpers::check("# Demo\nshort\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "demo: README is a stub",
            "README at `crates/demo/README.md` is only 13 bytes. Add meaningful content to the README.",
            "crates/demo/README.md",
            false,
        )],
    );
}

#[test]
fn errors_when_readme_has_no_heading() {
    let results = helpers::check(&"x".repeat(260));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "demo: README has no heading",
            "README at `crates/demo/README.md` has no markdown heading. Add a markdown heading (for example `# Crate Name`).",
            "crates/demo/README.md",
            false,
        )],
    );
}

#[test]
fn inventories_when_readme_quality_is_good() {
    let results = helpers::check(&format!("# Demo\n\n{}", "x".repeat(260)));

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "demo: README quality looks good",
            "README at `crates/demo/README.md` has content and headings.",
            "crates/demo/README.md",
            true,
        )],
    );
}
