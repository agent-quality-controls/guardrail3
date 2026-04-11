use std::collections::BTreeSet;

use g3rs_test_file_tree_checks_types::G3RsTestFileTreeChecksInput;
use g3rs_test_types::{
    G3RsTestComponentFileTreeFacts, G3RsTestFileKind, G3RsTestOwnedSidecarFacts,
    G3RsTestSourceFile,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn run_input(input: G3RsTestFileTreeChecksInput) -> Vec<G3CheckResult> {
    crate::run::check(&input)
}

pub(crate) fn file(
    rel_path: &str,
    kind: G3RsTestFileKind,
    component_rel_dir: Option<&str>,
    owner_module_name: Option<&str>,
    assertions_package_name: Option<&str>,
    content: &str,
) -> G3RsTestSourceFile {
    G3RsTestSourceFile {
        rel_path: rel_path.to_owned(),
        kind,
        owner_module_name: owner_module_name.map(str::to_owned),
        component_rel_dir: component_rel_dir.map(str::to_owned),
        assertions_package_name: assertions_package_name.map(str::to_owned),
        content: content.to_owned(),
    }
}

pub(crate) fn component(
    rel_dir: &str,
    runtime_rel_dir: &str,
    runtime_package_name: Option<&str>,
    assertions_exists: bool,
    assertions_package_name: Option<&str>,
) -> G3RsTestComponentFileTreeFacts {
    G3RsTestComponentFileTreeFacts {
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
        assertions_package_name: assertions_package_name.map(str::to_owned),
        assertions_dependencies: BTreeSet::new(),
        sidecars: Vec::new(),
        external_harnesses: Vec::new(),
    }
}

pub(crate) fn with_sidecar(
    mut component: G3RsTestComponentFileTreeFacts,
    mod_rel_path: &str,
    assertions_module_rel_path: &str,
) -> G3RsTestComponentFileTreeFacts {
    component.sidecars.push(G3RsTestOwnedSidecarFacts {
        mod_rel_path: mod_rel_path.to_owned(),
        assertions_module_rel_path: assertions_module_rel_path.to_owned(),
    });
    component
}

pub(crate) fn with_external_harness(
    mut component: G3RsTestComponentFileTreeFacts,
    rel_path: &str,
) -> G3RsTestComponentFileTreeFacts {
    component.external_harnesses.push(rel_path.to_owned());
    component
}

pub(crate) fn input(
    files: Vec<G3RsTestSourceFile>,
    components: Vec<G3RsTestComponentFileTreeFacts>,
) -> G3RsTestFileTreeChecksInput {
    let local_package_names = components
        .iter()
        .filter_map(|component| component.runtime_package_name.clone())
        .chain(
            components
                .iter()
                .filter_map(|component| component.assertions_package_name.clone()),
        )
        .collect();
    G3RsTestFileTreeChecksInput {
        root_rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        files,
        components,
        local_package_names,
        input_failures: Vec::new(),
    }
}

pub(crate) fn assert_has_result(
    results: &[G3CheckResult],
    rule_id: &str,
    severity: G3Severity,
    title: &str,
    file: &str,
    line: Option<usize>,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == rule_id
                && result.severity() == severity
                && result.title() == title
                && result.file() == Some(file)
                && result.line() == line
        }),
        "missing {rule_id} result: severity={severity:?} title={title:?} file={file:?} line={line:?}\nactual={results:#?}"
    );
}

pub(crate) fn assert_has_inventory(
    results: &[G3CheckResult],
    rule_id: &str,
    title: &str,
    file: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == rule_id
                && result.title() == title
                && result.file() == Some(file)
                && result.inventory()
        }),
        "missing inventory {rule_id} result: title={title:?} file={file:?}\nactual={results:#?}"
    );
}

pub(crate) fn assert_no_rule(results: &[G3CheckResult], rule_id: &str) {
    assert!(
        results.iter().all(|result| result.id() != rule_id),
        "unexpected {rule_id} result present\nactual={results:#?}"
    );
}

fn parent_dir(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}
