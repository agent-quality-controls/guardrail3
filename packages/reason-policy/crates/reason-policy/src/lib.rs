#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonIssue {
    TooShort,
    TooFewWords,
    Placeholder,
}

pub fn validate_reason_text(reason: &str) -> Result<(), ReasonIssue> {
    let trimmed = reason.trim();
    if trimmed.len() < 12 {
        return Err(ReasonIssue::TooShort);
    }
    if trimmed.split_whitespace().count() < 2 {
        return Err(ReasonIssue::TooFewWords);
    }
    let normalized = trimmed.to_ascii_lowercase();
    if matches!(
        normalized.as_str(),
        "ok" | "okay"
            | "present"
            | "works"
            | "valid"
            | "value"
            | "error"
            | "failed"
            | "failure"
            | "test"
            | "reason"
            | "tbd"
            | "todo"
            | "fixme"
            | "temp"
            | "temporary"
            | "legacy"
    ) {
        return Err(ReasonIssue::Placeholder);
    }
    Ok(())
}

pub fn reason_text_is_useful(reason: &str) -> bool {
    validate_reason_text(reason).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{ReasonIssue, reason_text_is_useful, validate_reason_text};

    #[test]
    fn rejects_short_reasons() {
        assert_eq!(validate_reason_text("temp"), Err(ReasonIssue::TooShort));
        assert!(!reason_text_is_useful("legacy"));
    }

    #[test]
    fn rejects_single_word_reasons_even_when_long_enough() {
        assert_eq!(
            validate_reason_text("compatibility"),
            Err(ReasonIssue::TooFewWords)
        );
    }

    #[test]
    fn accepts_nontrivial_reasons() {
        assert_eq!(validate_reason_text("compatibility shim"), Ok(()));
        assert!(reason_text_is_useful("outer workflow validation"));
    }
}
