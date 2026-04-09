use g3rs_test_ast_checks_types::G3RsTestAstChecksInput;
use g3rs_test_types::{
    G3RsTestComponentAstFacts, G3RsTestFileKind, G3RsTestSourceFile,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn run_input(input: G3RsTestAstChecksInput) -> Vec<G3CheckResult> {
    crate::run::check(&input)
}

pub(crate) fn file(
    rel_path: &str,
    kind: G3RsTestFileKind,
    assertions_package_name: Option<&str>,
    content: &str,
) -> G3RsTestSourceFile {
    G3RsTestSourceFile {
        rel_path: rel_path.to_owned(),
        kind,
        owner_module_name: None,
        component_rel_dir: Some(String::new()),
        assertions_package_name: assertions_package_name.map(str::to_owned),
        content: content.to_owned(),
    }
}

pub(crate) fn component(assertions_package_name: Option<&str>) -> G3RsTestComponentAstFacts {
    G3RsTestComponentAstFacts {
        rel_dir: String::new(),
        runtime_rel_dir: String::new(),
        runtime_package_name: Some("demo".to_owned()),
        assertions_rel_dir: "assertions".to_owned(),
        assertions_exists: assertions_package_name.is_some(),
        assertions_package_name: assertions_package_name.map(str::to_owned),
    }
}

pub(crate) fn input(
    files: Vec<G3RsTestSourceFile>,
    assertions_package_name: Option<&str>,
) -> G3RsTestAstChecksInput {
    G3RsTestAstChecksInput {
        root_rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        files,
        components: vec![component(assertions_package_name)],
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
