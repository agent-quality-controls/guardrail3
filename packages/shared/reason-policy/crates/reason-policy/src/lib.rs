pub const DEFAULT_MIN_REASON_CHARS: usize = 12;
pub const DEFAULT_MIN_REASON_WORDS: usize = 2;

const PLACEHOLDERS: &[&str] = &[
    "ok",
    "okay",
    "present",
    "works",
    "valid",
    "value",
    "error",
    "failed",
    "failure",
    "test",
    "reason",
    "tbd",
    "todo",
    "fixme",
    "temp",
    "temporary",
    "legacy",
    "fix later",
    "...",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonIssue {
    Empty,
    TooShort {
        min_chars: usize,
        actual_chars: usize,
    },
    TooFewWords {
        min_words: usize,
        actual_words: usize,
    },
    Placeholder,
}

impl ReasonIssue {
    pub fn message(self) -> String {
        match self {
            Self::Empty => "reason must not be empty".to_owned(),
            Self::TooShort {
                min_chars,
                actual_chars,
            } => format!("reason must be at least {min_chars} characters; found {actual_chars}"),
            Self::TooFewWords {
                min_words,
                actual_words,
            } => format!("reason must be at least {min_words} words; found {actual_words}"),
            Self::Placeholder => "reason must not be a placeholder".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReasonPolicy {
    pub min_chars: usize,
    pub min_words: usize,
}

impl Default for ReasonPolicy {
    fn default() -> Self {
        Self {
            min_chars: DEFAULT_MIN_REASON_CHARS,
            min_words: DEFAULT_MIN_REASON_WORDS,
        }
    }
}

pub fn validate_reason_text(reason: &str) -> Result<(), ReasonIssue> {
    validate_reason_text_with_policy(reason, ReasonPolicy::default())
}

pub fn validate_reason_text_with_policy(
    reason: &str,
    policy: ReasonPolicy,
) -> Result<(), ReasonIssue> {
    let trimmed = reason.trim();
    if trimmed.is_empty() {
        return Err(ReasonIssue::Empty);
    }

    let normalized = trimmed.to_ascii_lowercase();
    if PLACEHOLDERS.contains(&normalized.as_str()) {
        return Err(ReasonIssue::Placeholder);
    }

    let actual_chars = trimmed.chars().count();
    if actual_chars < policy.min_chars {
        return Err(ReasonIssue::TooShort {
            min_chars: policy.min_chars,
            actual_chars,
        });
    }

    let actual_words = trimmed.split_whitespace().count();
    if actual_words < policy.min_words {
        return Err(ReasonIssue::TooFewWords {
            min_words: policy.min_words,
            actual_words,
        });
    }

    Ok(())
}

pub fn reason_text_is_useful(reason: &str) -> bool {
    validate_reason_text(reason).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_MIN_REASON_CHARS, DEFAULT_MIN_REASON_WORDS, ReasonIssue, reason_text_is_useful,
        validate_reason_text,
    };

    #[test]
    fn rejects_short_reasons() {
        assert_eq!(
            validate_reason_text("two words"),
            Err(ReasonIssue::TooShort {
                min_chars: DEFAULT_MIN_REASON_CHARS,
                actual_chars: 9,
            })
        );
        assert!(!reason_text_is_useful("two words"));
    }

    #[test]
    fn rejects_single_word_reasons_even_when_long_enough() {
        assert_eq!(
            validate_reason_text("compatibility"),
            Err(ReasonIssue::TooFewWords {
                min_words: DEFAULT_MIN_REASON_WORDS,
                actual_words: 1,
            })
        );
    }

    #[test]
    fn rejects_empty_reasons() {
        assert_eq!(validate_reason_text("   "), Err(ReasonIssue::Empty));
    }

    #[test]
    fn rejects_placeholder_reasons() {
        assert_eq!(validate_reason_text("temp"), Err(ReasonIssue::Placeholder));
        assert_eq!(
            validate_reason_text("fix later"),
            Err(ReasonIssue::Placeholder)
        );
    }

    #[test]
    fn accepts_nontrivial_reasons() {
        assert_eq!(validate_reason_text("compatibility shim"), Ok(()));
        assert!(reason_text_is_useful("outer workflow validation"));
    }
}
