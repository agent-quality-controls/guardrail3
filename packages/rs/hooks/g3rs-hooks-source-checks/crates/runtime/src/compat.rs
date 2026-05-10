pub(crate) use guardrail3_check_types::G3Severity;

/// `G3CheckResult` struct.
#[derive(Debug, Clone)]
pub(crate) struct G3CheckResult(guardrail3_check_types::G3CheckResult);

impl G3CheckResult {
    /// `from_parts` function.
    pub(crate) const fn from_parts(
        id: String,
        severity: G3Severity,
        title: String,
        message: String,
        file: Option<String>,
        line: Option<usize>,
        _inventory: bool,
    ) -> Self {
        Self(guardrail3_check_types::G3CheckResult::new(
            id, severity, title, message, file, line,
        ))
    }

    /// `into_inventory` method.
    pub(crate) fn into_inventory(self) -> Self {
        Self(self.0.into_inventory())
    }

    /// `into_inner` method.
    pub(crate) fn into_inner(self) -> guardrail3_check_types::G3CheckResult {
        self.0
    }
}

/// `finish` function.
pub(crate) fn finish(results: Vec<G3CheckResult>) -> Vec<guardrail3_check_types::G3CheckResult> {
    results.into_iter().map(G3CheckResult::into_inner).collect()
}
