/// Workspace profile — determines which guardrail policy variant to apply.
///
/// An `Application` workspace is deployed and consumed by end users.
/// A `Library` workspace is published and consumed by other developers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum G3Profile {
    /// Deployed application (binary, service, CLI tool).
    /// Stricter rules: require input validation, ban raw deserialization, etc.
    Application,
    /// Published library (imported by other crates).
    /// Library-appropriate rules: allow API stability suppression, no validation framework required.
    Library,
}

impl std::fmt::Display for G3Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Application => write!(f, "application"),
            Self::Library => write!(f, "library"),
        }
    }
}
