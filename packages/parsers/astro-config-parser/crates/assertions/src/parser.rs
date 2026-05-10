pub use astro_config_parser_runtime::types::{
    AstroConfigParseState, AstroOutputMode, AstroStaticValue, AstroTrailingSlashPolicy,
};

use astro_config_parser_runtime::types::AstroConfigDocument;
use std::path::Path;

pub fn parse_document(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
) -> Result<AstroConfigDocument, astro_config_parser_runtime::Error> {
    astro_config_parser_runtime::parse_document(workspace_root, config_rel_path)
}

pub fn assert_parsed_document(document: &AstroConfigDocument) {
    assert!(
        astro_config_parser_runtime::typed(document).is_some(),
        "expected parsed Astro config document, got: {document:#?}"
    );
}

pub fn assert_invalid_document(document: &AstroConfigDocument, expected_reason_fragment: &str) {
    let Some(reason) = astro_config_parser_runtime::parse_error_reason(document) else {
        unreachable!("expected invalid Astro config document, got parsed: {document:#?}");
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
    expected_out_dir: Option<&str>,
    expected_trailing_slash: Option<AstroTrailingSlashPolicy>,
    expected_integrations: &[&str],
    expected_adapter: Option<&str>,
) {
    let Some(snapshot) = astro_config_parser_runtime::typed(document) else {
        unreachable!("expected parsed Astro config document, got: {document:#?}");
    };

    assert_eq!(snapshot.site.as_deref(), expected_site);
    assert_eq!(snapshot.output, expected_output);
    assert_eq!(snapshot.out_dir.as_deref(), expected_out_dir);
    assert_eq!(snapshot.trailing_slash, expected_trailing_slash);
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
