use package_json_parser_runtime::types::{PackageJsonBoolFieldState, PackageJsonDocument};

pub fn assert_parsed_document(document: &PackageJsonDocument) {
    assert!(
        package_json_parser_runtime::typed(document).is_some(),
        "expected parsed package.json document, got: {document:#?}"
    );
}

pub fn assert_invalid_document(document: &PackageJsonDocument, expected_reason_fragment: &str) {
    let Some(reason) = package_json_parser_runtime::parse_error_reason(document) else {
        assert!(
            false,
            "expected invalid package.json document, got parsed: {document:#?}"
        );
        return;
    };
    assert!(
        reason.contains(expected_reason_fragment),
        "expected invalid reason to contain {expected_reason_fragment:?}, got {reason:?}"
    );
}

pub fn assert_bool_field_state(
    document: &PackageJsonDocument,
    field: &str,
    expected: Option<bool>,
) {
    match (
        package_json_parser_runtime::bool_field_state(document, field),
        expected,
    ) {
        (PackageJsonBoolFieldState::Missing, None) => {}
        (PackageJsonBoolFieldState::Value(actual), Some(expected)) => {
            assert_eq!(actual, expected, "bool field mismatch for {field}");
        }
        (actual, expected) => {
            panic!(
                "unexpected bool field state for {field}; actual: {actual:?}, expected: {expected:?}"
            );
        }
    }
}

pub fn assert_snapshot_fields(
    document: &PackageJsonDocument,
    expected_package_manager: Option<&str>,
    expected_node_engine: Option<&str>,
    expected_pnpm_engine: Option<&str>,
    expected_lint_script: Option<&str>,
    expected_override_keys: &[&str],
    expected_only_built_dependencies: &[&str],
    expected_dependencies: &[&str],
    expected_dev_dependencies: &[&str],
) {
    let snapshot = package_json_parser_runtime::typed(document).unwrap_or_else(|| {
        assert!(
            false,
            "expected parsed package.json document, got: {document:#?}"
        );
        unreachable!()
    });

    assert_eq!(
        snapshot.package_manager.as_deref(),
        expected_package_manager
    );
    assert_eq!(snapshot.engines_node.as_deref(), expected_node_engine);
    assert_eq!(snapshot.engines_pnpm.as_deref(), expected_pnpm_engine);
    assert_eq!(
        snapshot.scripts.get("lint").map(String::as_str),
        expected_lint_script
    );
    assert_eq!(
        snapshot.pnpm_override_keys,
        expected_override_keys
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        snapshot.pnpm_only_built_dependencies,
        expected_only_built_dependencies
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        snapshot.dependencies,
        expected_dependencies
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        snapshot.dev_dependencies,
        expected_dev_dependencies
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>()
    );
}
