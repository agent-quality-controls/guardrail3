//! Hooks contract types: requirements, trigger patterns, and command bindings.

use serde::Serialize;

/// A hook requirement describing what must run for a given trigger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct G3HookRequirement {
    /// Stable identifier for the requirement.
    pub id: String,
    /// Family that owns this requirement.
    pub owner_family: String,
    /// File path patterns that activate this requirement.
    pub trigger_patterns: Vec<G3HookTriggerPattern>,
    /// Cargo gate commands required to satisfy this requirement.
    pub required_commands: Vec<G3HookCommandRequirement>,
    /// Critical commands that must not fail open.
    pub critical_commands: Vec<G3HookCriticalCommand>,
}

/// A pattern used to match files that should activate a hook requirement.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum G3HookTriggerPattern {
    /// Exact repo-relative path match.
    ExactPath(String),
    /// Glob pattern match.
    Glob(String),
    /// File extension match.
    Extension(String),
}

/// A required cargo or external command identifier referenced by a hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum G3HookCommandRequirement {
    /// `cargo fmt --all -- --check`.
    CargoFmtCheck,
    /// `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
    CargoClippyDenyWarnings,
    /// `cargo deny check`.
    CargoDenyCheck,
    /// Concrete lockfile command (`cargo metadata --locked --format-version 1`).
    ConcreteLockfileCommand,
    /// `cargo test --workspace -- --test-threads=1`.
    CargoTest,
    /// `cargo-machete`.
    CargoMachete,
    /// `cargo dupes check` with default thresholds.
    CargoDupes,
    /// `cargo dupes check` with tests excluded.
    CargoDupesExcludeTests,
    /// `gitleaks protect --staged` (executed inline by the hook).
    Gitleaks,
    /// `g3rs validate --path <ws>` (the in-binary validator entry point).
    G3RsValidatePath,
}

impl G3HookCommandRequirement {
    /// Returns the concrete argv that satisfies this requirement, or `None`
    /// when the requirement does not map to a runnable cargo gate command.
    ///
    /// Variants that return `None`:
    /// - `Gitleaks`: the hook runs `gitleaks protect --staged` inline before
    ///   per-unit dispatch; the in-binary validator does not invoke gitleaks.
    /// - `G3RsValidatePath`: the in-binary validator IS the entry point that
    ///   receives this delegation, so it does not re-invoke itself.
    #[must_use]
    pub const fn concrete_command(self) -> Option<&'static [&'static str]> {
        match self {
            Self::CargoFmtCheck => Some(&["cargo", "fmt", "--all", "--", "--check"]),
            Self::CargoClippyDenyWarnings => Some(&[
                "cargo",
                "clippy",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ]),
            Self::CargoDenyCheck => Some(&["cargo", "deny", "check"]),
            Self::ConcreteLockfileCommand => {
                Some(&["cargo", "metadata", "--locked", "--format-version", "1"])
            }
            Self::CargoTest => Some(&["cargo", "test", "--workspace", "--", "--test-threads=1"]),
            Self::CargoMachete => Some(&["cargo-machete"]),
            Self::CargoDupes => Some(&[
                "cargo",
                "dupes",
                "check",
                "--max-exact",
                "85",
                "--max-exact-percent",
                "10",
            ]),
            Self::CargoDupesExcludeTests => Some(&[
                "cargo",
                "dupes",
                "check",
                "--max-exact",
                "85",
                "--max-exact-percent",
                "10",
                "--exclude-tests",
            ]),
            Self::Gitleaks | Self::G3RsValidatePath => None,
        }
    }
}

/// Identifier of a critical command that must not fail open.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum G3HookCriticalCommand {
    /// External binary referenced by name.
    Binary(String),
    /// Cargo subcommand referenced by name.
    CargoSubcommand(String),
}
