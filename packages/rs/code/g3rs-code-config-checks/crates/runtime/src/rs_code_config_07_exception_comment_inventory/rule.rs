use g3rs_code_config_checks_types::G3RsCodeConfigFile;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-CODE-CONFIG-07";

pub(crate) fn check(file: &G3RsCodeConfigFile, results: &mut Vec<G3CheckResult>) {
    for (index, line) in file.content.lines().enumerate() {
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

        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "EXCEPTION comment inventory".to_owned(),
            format!("Config exception comment: {comment_text}"),
            Some(file.rel_path.clone()),
            Some(index.saturating_add(1)),
        ));
    }
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
#[path = "rule_tests/mod.rs"]
mod rule_tests;
