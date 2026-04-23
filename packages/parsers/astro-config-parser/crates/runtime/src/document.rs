use astro_config_parser_types::document::{
    AstroConfigDocument, AstroConfigParseState, AstroConfigSnapshot, AstroIntegrationSnapshot,
};

pub fn typed(document: &AstroConfigDocument) -> Option<&AstroConfigSnapshot> {
    match &document.typed {
        AstroConfigParseState::Parsed(snapshot) => Some(snapshot),
        AstroConfigParseState::Invalid(_) => None,
    }
}

pub fn parse_error_reason(document: &AstroConfigDocument) -> Option<&str> {
    match &document.typed {
        AstroConfigParseState::Parsed(_) => None,
        AstroConfigParseState::Invalid(reason) => Some(reason),
    }
}

pub fn integration<'a>(
    document: &'a AstroConfigDocument,
    module: &str,
) -> Option<&'a AstroIntegrationSnapshot> {
    typed(document)?
        .integrations
        .iter()
        .find(|integration| integration.source_module.as_deref() == Some(module))
}

pub fn has_integration(document: &AstroConfigDocument, module: &str) -> bool {
    integration(document, module).is_some()
}
