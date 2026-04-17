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
