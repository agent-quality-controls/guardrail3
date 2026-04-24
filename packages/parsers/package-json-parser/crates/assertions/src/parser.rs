use package_json_parser_runtime::types::{
    PackageDependencySection, PackageDependencySpecParseState, PackageJsonBoolFieldState,
    PackageJsonDocument, SemverVersion,
};

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
        (actual, expected) => assert!(
            false,
            "unexpected bool field state for {field}; actual: {actual:?}, expected: {expected:?}"
        ),
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
    let Some(snapshot) = package_json_parser_runtime::typed(document) else {
        assert!(
            false,
            "expected parsed package.json document, got: {document:#?}"
        );
        return;
    };

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

pub fn assert_dependency_spec_exact(
    document: &PackageJsonDocument,
    name: &str,
    section: PackageDependencySection,
    expected_version: SemverVersion,
) {
    let Some(spec) = dependency_spec(document, name, section) else {
        assert!(false, "dependency spec should exist for {name}");
        return;
    };
    assert_eq!(
        spec.parsed,
        PackageDependencySpecParseState::Exact {
            version: expected_version,
        },
        "dependency spec parse state mismatch for {name}",
    );
}

pub fn assert_dependency_spec_range_minimum(
    document: &PackageJsonDocument,
    name: &str,
    section: PackageDependencySection,
    expected_minimum: Option<SemverVersion>,
) {
    let Some(spec) = dependency_spec(document, name, section) else {
        assert!(false, "dependency spec should exist for {name}");
        return;
    };
    match &spec.parsed {
        PackageDependencySpecParseState::Range { minimum, .. } => {
            assert_eq!(
                minimum, &expected_minimum,
                "dependency range minimum mismatch for {name}"
            );
        }
        actual => assert!(
            false,
            "expected range dependency spec for {name}, got {actual:?}"
        ),
    }
}

pub fn assert_dependency_spec_range_allows_below_unknown(
    document: &PackageJsonDocument,
    name: &str,
    section: PackageDependencySection,
    expected: bool,
) {
    let Some(spec) = dependency_spec(document, name, section) else {
        assert!(false, "dependency spec should exist for {name}");
        return;
    };
    match &spec.parsed {
        PackageDependencySpecParseState::Range {
            allows_below_minimum_unknown,
            ..
        } => {
            assert_eq!(
                *allows_below_minimum_unknown, expected,
                "dependency range unknown-below-minimum state mismatch for {name}"
            );
        }
        actual => assert!(
            false,
            "expected range dependency spec for {name}, got {actual:?}"
        ),
    }
}

pub fn assert_dependency_spec_kind(
    document: &PackageJsonDocument,
    name: &str,
    section: PackageDependencySection,
    expected_kind: &str,
) {
    let Some(spec) = dependency_spec(document, name, section) else {
        assert!(false, "dependency spec should exist for {name}");
        return;
    };
    let actual_kind = match &spec.parsed {
        PackageDependencySpecParseState::Exact { .. } => "exact",
        PackageDependencySpecParseState::Range { .. } => "range",
        PackageDependencySpecParseState::Workspace { .. } => "workspace",
        PackageDependencySpecParseState::File { .. } => "file",
        PackageDependencySpecParseState::Link { .. } => "link",
        PackageDependencySpecParseState::Catalog { .. } => "catalog",
        PackageDependencySpecParseState::Unsupported { .. } => "unsupported",
    };
    assert_eq!(actual_kind, expected_kind, "spec kind mismatch for {name}");
}

fn dependency_spec<'document>(
    document: &'document PackageJsonDocument,
    name: &str,
    section: PackageDependencySection,
) -> Option<&'document package_json_parser_runtime::types::PackageDependencySpec> {
    let Some(snapshot) = package_json_parser_runtime::typed(document) else {
        assert!(
            false,
            "expected parsed package.json document, got: {document:#?}"
        );
        return None;
    };
    snapshot
        .dependency_specs
        .iter()
        .find(|spec| spec.name == name && spec.section == section)
}
