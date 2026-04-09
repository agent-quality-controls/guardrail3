use std::path::Path;

use g3rs_code_ingestion_types::G3RsCodeExceptionCommentFact;
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntryKind};

use crate::run::IngestionError;

const CONFIG_COMMENT_FILE_NAMES: &[&str] = &[
    "guardrail3.toml",
    "clippy.toml",
    ".clippy.toml",
    "deny.toml",
    ".deny.toml",
    "Cargo.toml",
    "rustfmt.toml",
    "rust-toolchain.toml",
    "rust-toolchain",
];

pub(crate) fn collect_exception_comments(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsCodeExceptionCommentFact>, IngestionError> {
    let mut comments = Vec::new();

    for entry in crawl.entries.iter().filter(|entry| {
        entry.kind == G3RsWorkspaceEntryKind::File
            && Path::new(entry.path.rel_path.as_str())
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| CONFIG_COMMENT_FILE_NAMES.contains(&name))
    }) {
        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }

        let content =
            crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;

        for (index, line) in content.lines().enumerate() {
            let Some(comment_text) = extract_exception_comment(line) else {
                continue;
            };
            let normalized = comment_text
                .trim_start_matches('#')
                .trim_start_matches('/')
                .trim_start();
            if normalized.to_ascii_uppercase().starts_with("EXCEPTION:") {
                comments.push(G3RsCodeExceptionCommentFact {
                    rel_path: entry.path.rel_path.clone(),
                    line: index.saturating_add(1),
                    line_text: comment_text.to_owned(),
                });
            }
        }
    }

    Ok(comments)
}

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

    while index < bytes.len() {
        match state {
            State::Normal => {
                if bytes[index] == b'#' {
                    return line.get(index..).map(str::trim_start);
                }
                if bytes[index] == b'/' && bytes.get(index.saturating_add(1)) == Some(&b'/') {
                    return line.get(index..).map(str::trim_start);
                }
                if bytes[index] == b'"' {
                    state = State::DoubleQuoted { escaped: false };
                } else if bytes[index] == b'\'' {
                    state = State::SingleQuoted;
                }
            }
            State::DoubleQuoted { escaped } => {
                if escaped {
                    state = State::DoubleQuoted { escaped: false };
                } else if bytes[index] == b'\\' {
                    state = State::DoubleQuoted { escaped: true };
                } else if bytes[index] == b'"' {
                    state = State::Normal;
                }
            }
            State::SingleQuoted => {
                if bytes[index] == b'\'' {
                    state = State::Normal;
                }
            }
        }
        index = index.saturating_add(1);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::extract_exception_comment;

    #[test]
    fn ignores_comment_markers_inside_quotes() {
        assert_eq!(
            extract_exception_comment("key = \"# not a comment\" // EXCEPTION: real"),
            Some("// EXCEPTION: real")
        );
        assert_eq!(
            extract_exception_comment("key = '# also not a comment' # EXCEPTION: real"),
            Some("# EXCEPTION: real")
        );
    }
}
