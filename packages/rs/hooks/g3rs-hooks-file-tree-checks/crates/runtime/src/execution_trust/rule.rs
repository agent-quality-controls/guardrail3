use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `ID` constant.
const ID: &str = "g3rs-hooks/execution-trust";

/// `check` function.
pub(crate) fn check(trust_risks: &[String], results: &mut Vec<G3CheckResult>) {
    if trust_risks.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                "no competing hook systems detected".to_owned(),
                "No obvious alternate hook system or shadowing risk was found.".to_owned(),
                None,
                None,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "competing hook system detected".to_owned(),
        format!(
            "Found alternate hook surfaces that can shadow or confuse hook execution: {}",
            trust_risks.join(", ")
        ),
        None,
        None,
    ));
}
