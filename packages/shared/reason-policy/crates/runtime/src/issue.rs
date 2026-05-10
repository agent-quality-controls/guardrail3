/// One reason validation issue surfaced by the policy validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonIssue {
    /// Reason text is empty after trimming.
    Empty,
    /// Reason text is shorter than the required minimum character count.
    TooShort {
        /// Minimum number of characters required by the active policy.
        min_chars: usize,
        /// Number of characters observed in the trimmed reason text.
        actual_chars: usize,
    },
    /// Reason text contains fewer words than the required minimum.
    TooFewWords {
        /// Minimum number of words required by the active policy.
        min_words: usize,
        /// Number of words observed in the trimmed reason text.
        actual_words: usize,
    },
    /// Reason text is a known placeholder phrase.
    Placeholder,
}

impl ReasonIssue {
    /// Returns a human-readable explanation of this reason validation issue.
    #[must_use]
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
