use guardrail3_domain_report::Severity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleFinding<'a> {
    pub(crate) severity: Severity,
    pub(crate) title: &'a str,
    pub(crate) message: &'a str,
    pub(crate) file: Option<&'a str>,
    pub(crate) line: Option<usize>,
    pub(crate) inventory: bool,
}

impl<'a> RuleFinding<'a> {
    #[must_use]
    pub const fn new(
        severity: Severity,
        title: &'a str,
        message: &'a str,
        file: Option<&'a str>,
        line: Option<usize>,
        inventory: bool,
    ) -> Self {
        Self {
            severity,
            title,
            message,
            file,
            line,
            inventory,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding<'a> {
    pub(crate) id: &'a str,
    pub(crate) severity: Severity,
    pub(crate) title: &'a str,
    pub(crate) message: &'a str,
    pub(crate) file: Option<&'a str>,
    pub(crate) line: Option<usize>,
    pub(crate) inventory: bool,
}

impl<'a> Finding<'a> {
    #[must_use]
    pub const fn new(
        id: &'a str,
        severity: Severity,
        title: &'a str,
        message: &'a str,
        file: Option<&'a str>,
        line: Option<usize>,
        inventory: bool,
    ) -> Self {
        Self {
            id,
            severity,
            title,
            message,
            file,
            line,
            inventory,
        }
    }
}
