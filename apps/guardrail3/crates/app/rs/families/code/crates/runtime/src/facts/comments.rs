use std::collections::BTreeSet;

use guardrail3_domain_project_tree::ProjectTree;

use super::{ExceptionCommentFacts, owning_root_dir};

pub(super) fn collect_exception_comments(
    tree: &ProjectTree,
    root_dirs: &[String],
) -> Vec<ExceptionCommentFacts> {
    let mut comments = Vec::new();

    for rel_path in config_comment_rels(tree) {
        if !config_comment_owned(&rel_path, root_dirs) {
            continue;
        }
        let Some(content) = tree.file_content(&rel_path) else {
            continue;
        };
        for (index, line) in content.lines().enumerate() {
            let Some(comment_text) = extract_exception_comment(line) else {
                continue;
            };
            let normalized = comment_text
                .trim_start_matches('#')
                .trim_start_matches('/')
                .trim_start();
            if normalized.to_ascii_uppercase().starts_with("EXCEPTION:") {
                comments.push(ExceptionCommentFacts {
                    rel_path: rel_path.clone(),
                    line: index.saturating_add(1),
                    line_text: comment_text.to_owned(),
                });
            }
        }
    }

    comments
}

fn config_comment_owned(rel_path: &str, root_dirs: &[String]) -> bool {
    if owning_root_dir(rel_path, root_dirs).is_some() {
        return true;
    }

    super::file_parent_rel(rel_path).is_empty() && !root_dirs.is_empty()
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

fn config_comment_rels(tree: &ProjectTree) -> Vec<String> {
    let config_names = [
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
    let mut rels = BTreeSet::new();

    for (dir_rel, entry) in tree.structure() {
        for file_name in entry.files() {
            if config_names.contains(&file_name.as_str()) {
                let _ = rels.insert(ProjectTree::join_rel(dir_rel, file_name));
            }
        }
    }

    rels.into_iter().collect()
}
