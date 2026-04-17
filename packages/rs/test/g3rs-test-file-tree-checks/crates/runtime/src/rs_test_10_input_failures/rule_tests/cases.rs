use std::collections::BTreeSet;

use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_file_tree_checks_assertions::rs_test_10_input_failures::rule as assertions;

#[test]
fn reports_parse_failure_as_error_result() {
    let mut results = Vec::new();

    super::super::check(
        "demo",
        "tests/broken.rs",
        "expected one of: `fn`, `struct`, `enum`",
        &mut results,
    );

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-10",
        G3Severity::Error,
        "failed to read test input",
        "tests/broken.rs",
        None,
    );
    assertions::assert_message(
        &results,
        "RS-TEST-FILETREE-10",
        "failed to read test input",
        "tests/broken.rs",
        "expected one of: `fn`, `struct`, `enum`",
    );
}

#[test]
fn inactive_root_with_only_test_support_stays_quiet() {
    let mut input = input(
        vec![file(
            "test_support/src/lib.rs",
            G3RsTestFileKind::TestSupport,
            None,
            Some("lib"),
            None,
            "pub fn helper(name: &str) -> String { name.to_owned() }\n",
        )],
        vec![component(
            "",
            "crates/runtime",
            Some("demo_runtime"),
            true,
            Some("demo_assertions"),
        )],
    );
    input
        .input_failures
        .push(g3rs_test_types::G3RsTestFileTreeInputFailure {
            rel_path: "test_support/src/broken.rs".to_owned(),
            message: "parse failed".to_owned(),
        });

    let results = assertions::check(&input);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn inactive_root_with_only_assertions_module_stays_quiet() {
    let results = assertions::check(&input(
        vec![file(
            "crates/assertions/src/lib.rs",
            G3RsTestFileKind::AssertionsModule,
            Some(""),
            Some("lib"),
            Some("demo_assertions"),
            "pub fn assert_runtime() { assert!(true); }\n",
        )],
        vec![component(
            "",
            "crates/runtime",
            Some("demo_runtime"),
            true,
            Some("demo_assertions"),
        )],
    ));

    assertions::assert_no_rule(&results, "RS-TEST-FILETREE-02");
    assertions::assert_no_rule(&results, "RS-TEST-FILETREE-03");
    assertions::assert_no_rule(&results, "RS-TEST-FILETREE-10");
    assertions::assert_no_rule(&results, "RS-TEST-FILETREE-18");
}

fn file(
    rel_path: &str,
    kind: G3RsTestFileKind,
    component_rel_dir: Option<&str>,
    owner_module_name: Option<&str>,
    assertions_package_name: Option<&str>,
    content: &str,
) -> g3rs_test_types::G3RsTestSourceFile {
    g3rs_test_types::G3RsTestSourceFile {
        rel_path: rel_path.to_owned(),
        kind,
        owner_module_name: owner_module_name.map(str::to_owned),
        component_rel_dir: component_rel_dir.map(str::to_owned),
        assertions_package_name: assertions_package_name.map(str::to_owned),
        content: content.to_owned(),
    }
}

fn component(
    rel_dir: &str,
    runtime_rel_dir: &str,
    runtime_package_name: Option<&str>,
    assertions_exists: bool,
    assertions_package_name: Option<&str>,
) -> g3rs_test_types::G3RsTestComponentFileTreeFacts {
    g3rs_test_types::G3RsTestComponentFileTreeFacts {
        rel_dir: rel_dir.to_owned(),
        runtime_rel_dir: runtime_rel_dir.to_owned(),
        runtime_cargo_rel_path: if runtime_rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{runtime_rel_dir}/Cargo.toml")
        },
        runtime_package_name: runtime_package_name.map(str::to_owned),
        runtime_normal_dependencies: BTreeSet::new(),
        runtime_dev_dependencies: BTreeSet::new(),
        assertions_rel_dir: if runtime_rel_dir.is_empty() {
            "assertions".to_owned()
        } else {
            format!("{}/assertions", parent_dir(runtime_rel_dir))
        },
        assertions_cargo_rel_path: if runtime_rel_dir.is_empty() {
            "assertions/Cargo.toml".to_owned()
        } else {
            format!("{}/assertions/Cargo.toml", parent_dir(runtime_rel_dir))
        },
        assertions_exists,
        nested_assertions_cargo_rel_path: None,
        assertions_package_name: assertions_package_name.map(str::to_owned),
        assertions_dependencies: BTreeSet::new(),
        sidecars: Vec::new(),
        external_harnesses: Vec::new(),
    }
}

fn input(
    files: Vec<g3rs_test_types::G3RsTestSourceFile>,
    components: Vec<g3rs_test_types::G3RsTestComponentFileTreeFacts>,
) -> g3rs_test_types::G3RsTestFileTreeChecksInput {
    let local_package_names = components
        .iter()
        .filter_map(|component| component.runtime_package_name.clone())
        .chain(
            components
                .iter()
                .filter_map(|component| component.assertions_package_name.clone()),
        )
        .collect();
    g3rs_test_types::G3RsTestFileTreeChecksInput {
        root_rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        files,
        components,
        local_package_names,
        input_failures: Vec::new(),
    }
}

fn parent_dir(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}
