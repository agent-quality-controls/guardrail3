pub const DEFAULT_MIN_REASON_CHARS: usize = 12;
pub const DEFAULT_MIN_REASON_WORDS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReasonPolicy {
    min_chars: usize,
    min_words: usize,
}

impl ReasonPolicy {
    #[must_use]
    pub const fn new(min_chars: usize, min_words: usize) -> Self {
        Self {
            min_chars,
            min_words,
        }
    }

    #[must_use]
    pub const fn min_chars(self) -> usize {
        self.min_chars
    }

    #[must_use]
    pub const fn min_words(self) -> usize {
        self.min_words
    }
}

impl Default for ReasonPolicy {
    fn default() -> Self {
        Self::new(DEFAULT_MIN_REASON_CHARS, DEFAULT_MIN_REASON_WORDS)
    }
}
