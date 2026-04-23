use astro_config_parser_runtime::types::{AstroConfigDocument, AstroOutputMode};

pub fn assert_parsed_document(document: &AstroConfigDocument) {
    assert!(
        astro_config_parser_runtime::typed(document).is_some(),
        "expected parsed Astro config document, got: {document:#?}"
    );
}

pub fn assert_invalid_document(document: &AstroConfigDocument, expected_reason_fragment: &str) {
    let Some(reason) = astro_config_parser_runtime::parse_error_reason(document) else {
        assert!(
            false,
            "expected invalid Astro config document, got parsed: {document:#?}"
        );
        return;
    };
    assert!(
        reason.contains(expected_reason_fragment),
        "expected invalid reason to contain {expected_reason_fragment:?}, got {reason:?}"
    );
}

pub fn assert_snapshot(
    document: &AstroConfigDocument,
    expected_site: Option<&str>,
    expected_output: Option<AstroOutputMode>,
    expected_integrations: &[&str],
    expected_adapter: Option<&str>,
) {
    let Some(snapshot) = astro_config_parser_runtime::typed(document) else {
        assert!(
            false,
            "expected parsed Astro config document, got: {document:#?}"
        );
        return;
    };

    assert_eq!(snapshot.site.as_deref(), expected_site);
    assert_eq!(snapshot.output, expected_output);
    assert_eq!(
        snapshot
            .integrations
            .iter()
            .filter_map(|integration| integration.source_module.as_deref())
            .collect::<Vec<_>>(),
        expected_integrations
    );
    assert_eq!(
        snapshot
            .adapter
            .as_ref()
            .and_then(|adapter| adapter.source_module.as_deref()),
        expected_adapter
    );
}
