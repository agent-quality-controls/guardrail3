use super::constants::*;
use super::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum G3TsAstroRawEslintConfigState {
    Missing {
        rel_path: String,
    },
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        rel_path: String,
        snapshot: eslint_config_parser::types::EslintConfigSnapshot,
    },
}

#[must_use]
pub fn read_eslint_config_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    probes: &[eslint_config_parser::types::EslintProbeTarget],
) -> G3TsAstroRawEslintConfigState {
    let Some(entry) = crate::select::select_active_eslint_config(crawl, app_root_rel_path) else {
        return G3TsAstroRawEslintConfigState::Missing {
            rel_path: missing_eslint_config_rel_path(app_root_rel_path),
        };
    };
    if !entry.readable {
        return G3TsAstroRawEslintConfigState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected eslint config unreadable".to_owned(),
        };
    }

    let document = match parse_eslint_document(&crawl.root_abs_path, &entry.path.rel_path, &probes)
    {
        Ok(document) => document,
        Err(error) => {
            return G3TsAstroRawEslintConfigState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };
    if let Some(reason) = eslint_parse_error_reason(&document) {
        return G3TsAstroRawEslintConfigState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let Some(snapshot) = eslint_config_parser::typed(&document) else {
        return G3TsAstroRawEslintConfigState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "parsed eslint config document did not provide a typed snapshot".to_owned(),
        };
    };

    G3TsAstroRawEslintConfigState::Parsed {
        rel_path: entry.path.rel_path.clone(),
        snapshot: snapshot.clone(),
    }
}

#[must_use]
fn missing_eslint_config_rel_path(app_root_rel_path: &str) -> String {
    if app_root_rel_path == "." {
        ESLINT_CONFIG_PATTERN.to_owned()
    } else {
        format!("{app_root_rel_path}/{ESLINT_CONFIG_PATTERN}")
    }
}
