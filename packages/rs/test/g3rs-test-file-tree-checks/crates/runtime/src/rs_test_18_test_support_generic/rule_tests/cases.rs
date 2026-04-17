use std::collections::BTreeSet;

use g3rs_test_types::G3RsTestFileKind;
use guardrail3_check_types::G3Severity;

use g3rs_test_file_tree_checks_assertions::rs_test_18_test_support_generic::rule as assertions;

fn active_component() -> g3rs_test_types::G3RsTestComponentFileTreeFacts {
    with_external_harness(
        {
            let mut component = component(
                "",
                "crates/runtime",
                Some("demo_runtime"),
                true,
                Some("demo_assertions"),
            );
            let _ = component
                .runtime_dev_dependencies
                .insert("demo_assertions".to_owned());
            let _ = component
                .assertions_dependencies
                .insert("demo_runtime".to_owned());
            component
        },
        "crates/runtime/tests/public_surface.rs",
    )
}

fn active_files() -> Vec<g3rs_test_types::G3RsTestSourceFile> {
    vec![
        file(
            "crates/runtime/tests/public_surface.rs",
            G3RsTestFileKind::ExternalHarness,
            Some(""),
            None,
            Some("demo_assertions"),
            "use demo_assertions::assert_runtime;\n#[test]\nfn public_surface() { assert_runtime(); }\n",
        ),
        file(
            "crates/assertions/src/lib.rs",
            G3RsTestFileKind::AssertionsModule,
            Some(""),
            Some("lib"),
            Some("demo_assertions"),
            "pub fn assert_runtime() { assert_eq!(demo_runtime::value(), 1); }\n",
        ),
    ]
}

#[test]
fn reports_inventory_for_generic_test_support() {
    let mut files = active_files();
    files.push(file(
        "test_support/src/lib.rs",
        G3RsTestFileKind::TestSupport,
        None,
        Some("lib"),
        None,
        "pub fn fixture_name(name: &str) -> String { name.to_owned() }\n",
    ));
    let results = assertions::check(&input(files, vec![active_component()]));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-FILETREE-18",
        "test_support stays generic",
        "test_support/src/lib.rs",
    );
}

#[test]
fn reports_inventory_for_crates_test_support_layout() {
    let mut files = active_files();
    files.push(file(
        "crates/test_support/src/lib.rs",
        G3RsTestFileKind::TestSupport,
        None,
        Some("lib"),
        None,
        "pub fn fixture_name(name: &str) -> String { name.to_owned() }\n",
    ));
    let results = assertions::check(&input(files, vec![active_component()]));

    assertions::assert_has_inventory(
        &results,
        "RS-TEST-FILETREE-18",
        "test_support stays generic",
        "crates/test_support/src/lib.rs",
    );
}

#[test]
fn reports_importing_runtime() {
    let mut files = active_files();
    files.push(file(
        "test_support/src/lib.rs",
        G3RsTestFileKind::TestSupport,
        None,
        Some("lib"),
        None,
        "use demo_runtime::value;\npub fn fixture_value() -> u8 { value() }\n",
    ));
    let results = assertions::check(&input(files, vec![active_component()]));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-18",
        G3Severity::Error,
        "test_support imports local component crate",
        "test_support/src/lib.rs",
        Some(1),
    );
}

#[test]
fn reports_direct_runtime_call() {
    let mut files = active_files();
    files.push(file(
        "test_support/src/lib.rs",
        G3RsTestFileKind::TestSupport,
        None,
        Some("lib"),
        None,
        "pub fn fixture_value() -> u8 { demo_runtime::value() }\n",
    ));
    let results = assertions::check(&input(files, vec![active_component()]));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-18",
        G3Severity::Error,
        "test_support calls local component crate",
        "test_support/src/lib.rs",
        None,
    );
}

#[test]
fn reports_route_construction_import() {
    let mut files = active_files();
    files.push(file(
        "test_support/src/lib.rs",
        G3RsTestFileKind::TestSupport,
        None,
        Some("lib"),
        None,
        "use FamilyMapper;\npub fn route() {}\n",
    ));
    let results = assertions::check(&input(files, vec![active_component()]));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-18",
        G3Severity::Error,
        "test_support imports route construction infrastructure",
        "test_support/src/lib.rs",
        Some(1),
    );
}

#[test]
fn reports_public_semantic_constant() {
    let mut files = active_files();
    files.push(file(
        "test_support/src/lib.rs",
        G3RsTestFileKind::TestSupport,
        None,
        Some("lib"),
        None,
        "pub const EXPECTED_ID: &str = \"RS-TEST-FILETREE-03\";\n",
    ));
    let results = assertions::check(&input(files, vec![active_component()]));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-18",
        G3Severity::Error,
        "test_support exports public semantic constant",
        "test_support/src/lib.rs",
        Some(1),
    );
}

#[test]
fn reports_canned_path_or_string_helper() {
    let mut files = active_files();
    files.push(file(
        "test_support/src/lib.rs",
        G3RsTestFileKind::TestSupport,
        None,
        Some("lib"),
        None,
        "const FIXTURE: &str = \"fixtures/demo.json\";\npub fn fixture_path() -> &str { FIXTURE }\n",
    ));
    let results = assertions::check(&input(files, vec![active_component()]));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-18",
        G3Severity::Error,
        "test_support exports canned path or string helper",
        "test_support/src/lib.rs",
        Some(2),
    );
}

#[test]
fn reports_canned_fixture_helper() {
    let mut files = active_files();
    files.push(file(
        "test_support/src/lib.rs",
        G3RsTestFileKind::TestSupport,
        None,
        Some("lib"),
        None,
        "fn fixture_path() -> &'static str { \"fixtures/demo.json\" }\npub fn demo_fixture() -> Vec<&'static str> { vec![fixture_path()] }\n",
    ));
    let results = assertions::check(&input(files, vec![active_component()]));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-18",
        G3Severity::Error,
        "test_support exports canned fixture helper",
        "test_support/src/lib.rs",
        Some(2),
    );
}

#[test]
fn reports_semantic_finding_helper() {
    let mut files = active_files();
    files.push(file(
        "test_support/src/lib.rs",
        G3RsTestFileKind::TestSupport,
        None,
        Some("lib"),
        None,
        "use guardrail3_domain_report::CheckResult;\npub fn has_rule(results: &[CheckResult], rule_id: &str) -> bool { results.iter().any(|result| result.id() == rule_id) }\n",
    ));
    let results = assertions::check(&input(files, vec![active_component()]));

    assertions::assert_has_result(
        &results,
        "RS-TEST-FILETREE-18",
        G3Severity::Error,
        "test_support exports semantic finding helper",
        "test_support/src/lib.rs",
        Some(2),
    );
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

fn with_external_harness(
    mut component: g3rs_test_types::G3RsTestComponentFileTreeFacts,
    rel_path: &str,
) -> g3rs_test_types::G3RsTestComponentFileTreeFacts {
    component.external_harnesses.push(rel_path.to_owned());
    component
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
