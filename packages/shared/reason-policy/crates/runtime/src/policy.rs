/// Default minimum number of characters required for a reason text.
pub const DEFAULT_MIN_REASON_CHARS: usize = 12;
/// Default minimum number of words required for a reason text.
pub const DEFAULT_MIN_REASON_WORDS: usize = 2;

/// Configurable thresholds applied when validating reason text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReasonPolicy {
    /// Minimum number of characters a reason text must contain.
    min_chars: usize,
    /// Minimum number of whitespace-separated words a reason text must contain.
    min_words: usize,
}

impl ReasonPolicy {
    /// Constructs a new `ReasonPolicy` from explicit thresholds.
    #[must_use]
    pub const fn new(min_chars: usize, min_words: usize) -> Self {
        Self {
            min_chars,
            min_words,
        }
    }

    /// Returns the minimum number of characters required by this policy.
    #[must_use]
    pub const fn min_chars(self) -> usize {
        self.min_chars
    }

    /// Returns the minimum number of words required by this policy.
    #[must_use]
    pub const fn min_words(self) -> usize {
        self.min_words
    }
}

impl Default for ReasonPolicy {
    /// Constructs a policy from the family-wide default thresholds.
    fn default() -> Self {
        Self::new(DEFAULT_MIN_REASON_CHARS, DEFAULT_MIN_REASON_WORDS)
    }
}
