/// Severity of a check result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum G3Severity {
    /// Hard failure — must be fixed.
    Error,
    /// Documented escape hatch or degraded condition — should be investigated.
    Warn,
    /// Informational inventory item — hidden by default.
    Info,
}

impl std::fmt::Display for G3Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warn => write!(f, "warn"),
            Self::Info => write!(f, "info"),
        }
    }
}
