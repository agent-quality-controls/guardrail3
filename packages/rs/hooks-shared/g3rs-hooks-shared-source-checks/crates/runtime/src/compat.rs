pub(crate) use guardrail3_check_types::G3Severity;

#[derive(Debug, Clone)]
pub(crate) struct G3CheckResult(guardrail3_check_types::G3CheckResult);

impl G3CheckResult {
    pub(crate) fn from_parts(
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

    pub(crate) fn into_inventory(self) -> Self {
        Self(self.0.into_inventory())
    }

    pub(crate) fn into_inner(self) -> guardrail3_check_types::G3CheckResult {
        self.0
    }
}

pub(crate) fn finish(results: Vec<G3CheckResult>) -> Vec<guardrail3_check_types::G3CheckResult> {
    results.into_iter().map(G3CheckResult::into_inner).collect()
}
