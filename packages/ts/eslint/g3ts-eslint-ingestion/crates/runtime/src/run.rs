use eslint_config_parser::{parse_document, parse_error_reason};
use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_eslint_types::{G3TsEslintConfigChecksInput, G3TsEslintConfigState, snapshot_from_parser};

/// Ingests the active eslint root config and returns the typed checks input.
///
/// # Panics
/// Panics when the parser succeeds with no error reason but the typed view is `None`; this state is unreachable per the parser contract.
#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsEslintConfigChecksInput {
    let Some(entry) = crate::select::select_active_root_eslint_config(crawl) else {
        return G3TsEslintConfigChecksInput {
            config: G3TsEslintConfigState::Missing,
        };
    };

    if !entry.readable {
        return G3TsEslintConfigChecksInput {
            config: G3TsEslintConfigState::Unreadable {
                rel_path: entry.path.rel_path.clone(),
                reason: "workspace crawl marked the selected config unreadable".to_owned(),
            },
        };
    }

    let probes = crate::select::probe_targets(crawl, &entry.path.rel_path);
    let document = match parse_document(&crawl.root_abs_path, &entry.path.rel_path, &probes) {
        Ok(document) => document,
        Err(err) => {
            return G3TsEslintConfigChecksInput {
                config: G3TsEslintConfigState::ParseError {
                    rel_path: entry.path.rel_path.clone(),
                    reason: err.to_string(),
                },
            };
        }
    };

    let config = parse_error_reason(&document).map_or_else(
        || {
            let Some(typed) = eslint_config_parser::typed(&document) else {
                return G3TsEslintConfigState::ParseError {
                    rel_path: entry.path.rel_path.clone(),
                    reason: "eslint typed parser view unavailable after successful parse"
                        .to_owned(),
                };
            };
            G3TsEslintConfigState::Parsed {
                snapshot: snapshot_from_parser(typed),
            }
        },
        |reason| G3TsEslintConfigState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        },
    );

    G3TsEslintConfigChecksInput { config }
}
