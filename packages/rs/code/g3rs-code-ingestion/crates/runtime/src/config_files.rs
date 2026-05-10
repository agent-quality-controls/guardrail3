use cargo_toml_parser::parse as parse_cargo_toml;
use clippy_toml_parser::parse as parse_clippy_toml;
use deny_toml_parser::parse as parse_deny_toml;
use g3rs_code_types::{
    G3RsCodeConfigChecksInput, G3RsCodeConfigFile, G3RsCodeConfigFileKind, G3RsCodeExceptionComment,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use guardrail3_rs_toml_parser::parse as parse_guardrail3_toml;
use rust_toolchain_toml_parser::parse as parse_rust_toolchain_toml;
use rustfmt_toml_parser::parse as parse_rustfmt_toml;

use crate::run::IngestionError;

/// Constant value used by the surrounding module.
const CONFIG_FILE_NAMES: &[&str] = &[
    "guardrail3-rs.toml",
    "clippy.toml",
    ".clippy.toml",
    "deny.toml",
    ".deny.toml",
    "Cargo.toml",
    "rustfmt.toml",
    "rust-toolchain.toml",
    "rust-toolchain",
];

/// Implements `collect config files`.
pub(crate) fn collect_config_files(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsCodeConfigChecksInput, IngestionError> {
    let mut files = Vec::new();
    let mut exception_comments = Vec::new();

    for entry in crate::config_scope::select_owned_config_entries(crawl, CONFIG_FILE_NAMES)? {
        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }

        let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| {
            IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            }
        })?;

        exception_comments.extend(extract_exception_comments(&entry.path.rel_path, &content));

        if let Some(kind) =
            parse_config_file_kind(&entry.path.rel_path, &entry.path.abs_path, &content)?
        {
            files.push(G3RsCodeConfigFile {
                rel_path: entry.path.rel_path.clone(),
                kind,
            });
        }
    }

    Ok(G3RsCodeConfigChecksInput {
        files,
        exception_comments,
    })
}

/// Optional config-file kind. `None` means the file name was recognized but the file
/// does not contribute a typed entry (e.g. legacy `rust-toolchain` text file).
type ParsedConfigFileKind = Result<Option<G3RsCodeConfigFileKind>, IngestionError>;

/// Implements `parse config file kind`.
fn parse_config_file_kind(
    rel_path: &str,
    abs_path: &std::path::Path,
    content: &str,
) -> ParsedConfigFileKind {
    let file_name = file_name(rel_path);

    let kind = match file_name {
        "guardrail3-rs.toml" => Some(G3RsCodeConfigFileKind::Guardrail3RsToml {
            guardrail3: parse_guardrail3_toml(content).map_err(|err| {
                IngestionError::ParseFailed {
                    path: abs_path.to_path_buf(),
                    reason: err.to_string(),
                }
            })?,
        }),
        "clippy.toml" | ".clippy.toml" => Some(G3RsCodeConfigFileKind::ClippyToml {
            clippy: parse_clippy_toml(content).map_err(|err| IngestionError::ParseFailed {
                path: abs_path.to_path_buf(),
                reason: err.to_string(),
            })?,
        }),
        "deny.toml" | ".deny.toml" => Some(G3RsCodeConfigFileKind::DenyToml {
            deny: parse_deny_toml(content).map_err(|err| IngestionError::ParseFailed {
                path: abs_path.to_path_buf(),
                reason: err.to_string(),
            })?,
        }),
        "Cargo.toml" => Some(G3RsCodeConfigFileKind::CargoToml {
            cargo: parse_cargo_toml(content).map_err(|err| IngestionError::ParseFailed {
                path: abs_path.to_path_buf(),
                reason: err.to_string(),
            })?,
        }),
        "rustfmt.toml" => Some(G3RsCodeConfigFileKind::RustfmtToml {
            rustfmt: parse_rustfmt_toml(content).map_err(|err| IngestionError::ParseFailed {
                path: abs_path.to_path_buf(),
                reason: err.to_string(),
            })?,
        }),
        "rust-toolchain.toml" => Some(G3RsCodeConfigFileKind::RustToolchainToml {
            toolchain: parse_rust_toolchain_toml(content).map_err(|err| {
                IngestionError::ParseFailed {
                    path: abs_path.to_path_buf(),
                    reason: err.to_string(),
                }
            })?,
        }),
        _ => None,
    };

    Ok(kind)
}

/// Implements `extract exception comments`.
fn extract_exception_comments(rel_path: &str, content: &str) -> Vec<G3RsCodeExceptionComment> {
    let mut comments = Vec::new();

    for (index, line) in content.lines().enumerate() {
        let Some(comment_text) = extract_exception_comment(line) else {
            continue;
        };
        let normalized = comment_text
            .trim_start_matches('#')
            .trim_start_matches('/')
            .trim_start();
        if !normalized.to_ascii_uppercase().starts_with("EXCEPTION:") {
            continue;
        }

        comments.push(G3RsCodeExceptionComment {
            rel_path: rel_path.to_owned(),
            line: index.saturating_add(1),
            text: comment_text.to_owned(),
        });
    }

    comments
}

/// Implements `extract exception comment`.
fn extract_exception_comment(line: &str) -> Option<&str> {
    #[derive(Clone, Copy)]
    enum State {
        Normal,
        DoubleQuoted { escaped: bool },
        SingleQuoted,
    }

    let bytes = line.as_bytes();
    let mut index = 0usize;
    let mut state = State::Normal;

    while let Some(&byte) = bytes.get(index) {
        match state {
            State::Normal => {
                if byte == b'#' {
                    return line.get(index..).map(str::trim_start);
                }
                if byte == b'/' && bytes.get(index.saturating_add(1)) == Some(&b'/') {
                    return line.get(index..).map(str::trim_start);
                }
                if byte == b'"' {
                    state = State::DoubleQuoted { escaped: false };
                } else if byte == b'\'' {
                    state = State::SingleQuoted;
                }
            }
            State::DoubleQuoted { escaped } => {
                if escaped {
                    state = State::DoubleQuoted { escaped: false };
                } else if byte == b'\\' {
                    state = State::DoubleQuoted { escaped: true };
                } else if byte == b'"' {
                    state = State::Normal;
                }
            }
            State::SingleQuoted => {
                if byte == b'\'' {
                    state = State::Normal;
                }
            }
        }
        index = index.saturating_add(1);
    }

    None
}

/// Implements `file name`.
fn file_name(rel_path: &str) -> &str {
    rel_path.rsplit('/').next().unwrap_or(rel_path)
}
