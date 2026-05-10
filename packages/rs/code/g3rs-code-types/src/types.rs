//! Shared input contract types for the g3rs code family.

use cargo_toml_parser::types::CargoToml;
use clippy_toml_parser::types::ClippyToml;
use deny_toml_parser::types::DenyToml;
use guardrail3_rs_toml_parser::types::Guardrail3RsToml;
use rust_toolchain_toml_parser::types::RustToolchainToml;
use rustfmt_toml_parser::types::RustfmtToml;
use syn::File;

/// One of the recognised Rust-family TOML configuration file kinds.
///
/// Each variant intentionally ends with `Toml` because that suffix is the
/// stable public discriminator that callers across other workspaces match
/// on (e.g. `code-ingestion`, `code-config-checks`); renaming the variants
/// would break that public matching contract. The variant payloads carry
/// the parser's full struct directly: callers in other workspaces
/// construct these with bare values and pattern-match on bare bindings,
/// so boxing for size symmetry would also break the cross-workspace API.
#[allow(
    clippy::enum_variant_names,
    clippy::large_enum_variant,
    reason = "variant names and payload shape are part of the cross-workspace public matching API; changing them would break out-of-scope callers"
)]
#[derive(Debug, Clone)]
pub enum G3RsCodeConfigFileKind {
    /// A `guardrail3-rs.toml` adoption marker.
    Guardrail3RsToml {
        /// Parsed marker contents.
        guardrail3: Guardrail3RsToml,
    },
    /// A `clippy.toml` configuration file.
    ClippyToml {
        /// Parsed clippy configuration.
        clippy: ClippyToml,
    },
    /// A `deny.toml` configuration file.
    DenyToml {
        /// Parsed cargo-deny configuration.
        deny: DenyToml,
    },
    /// A `Cargo.toml` manifest.
    CargoToml {
        /// Parsed cargo manifest.
        cargo: CargoToml,
    },
    /// A `rustfmt.toml` configuration file.
    RustfmtToml {
        /// Parsed rustfmt configuration.
        rustfmt: RustfmtToml,
    },
    /// A `rust-toolchain.toml` toolchain pin.
    RustToolchainToml {
        /// Parsed toolchain pin.
        toolchain: RustToolchainToml,
    },
}

/// A discovered configuration file alongside its repo-relative path.
#[derive(Debug, Clone)]
pub struct G3RsCodeConfigFile {
    /// Repo-relative path of the configuration file.
    pub rel_path: String,
    /// Parsed configuration payload.
    pub kind: G3RsCodeConfigFileKind,
}

/// A `// g3rs-allow:` style exception comment found in a source file.
#[derive(Debug, Clone)]
pub struct G3RsCodeExceptionComment {
    /// Repo-relative path of the file containing the comment.
    pub rel_path: String,
    /// 1-based line number where the comment occurs.
    pub line: usize,
    /// Raw comment text.
    pub text: String,
}

/// Input contract for code-family configuration checks.
#[derive(Debug, Clone)]
pub struct G3RsCodeConfigChecksInput {
    /// Discovered configuration files to validate.
    pub files: Vec<G3RsCodeConfigFile>,
    /// Inline exception comments collected from source files.
    pub exception_comments: Vec<G3RsCodeExceptionComment>,
}

/// A single Rust source file fed into the source-checks pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsSourceFile {
    /// Repo-relative path of the source file.
    pub rel_path: String,
    /// Raw file contents.
    pub content: String,
    /// True when the file is a `#[cfg(test)]` or `tests/` artifact.
    pub is_test: bool,
    /// Family profile name owning this file, when applicable.
    pub profile_name: Option<String>,
    /// True when the file is the library root (`lib.rs`).
    pub is_library_root: bool,
}

/// Result of attempting to parse a source file with `syn`.
#[derive(Debug, Clone)]
pub enum G3RsCodeParsedSourceState {
    /// Parser produced a valid `syn::File`.
    Parsed(File),
    /// Parser failed; carries the human-readable failure message.
    Invalid {
        /// Human-readable parse error message.
        message: String,
    },
}

/// Input contract for code-family source checks.
#[derive(Debug, Clone)]
pub struct G3RsCodeSourceChecksInput {
    /// Source file under inspection.
    pub source_file: G3RsSourceFile,
    /// Parsed AST (or failure) for the file.
    pub parsed_source: G3RsCodeParsedSourceState,
    /// True when the file belongs to a shared (cross-family) crate.
    pub is_shared_crate: bool,
    /// Waiver entries that may suppress findings on this file.
    pub waivers: Vec<G3RsCodeWaiver>,
}

/// A waiver that suppresses a specific rule on a specific selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeWaiver {
    /// Rule identifier the waiver applies to.
    pub rule: String,
    /// Repo-relative path of the file the waiver applies to.
    pub file: String,
    /// Selector inside the file (item path or similar).
    pub selector: String,
    /// Justification text for the waiver.
    pub reason: String,
}

/// A structural cap root used for module-tree size checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeStructuralCapRoot {
    /// Repo-relative directory the cap applies to.
    pub root_rel_dir: String,
    /// Repo-relative path of the owning `Cargo.toml`.
    pub cargo_rel_path: String,
}

/// Input contract for code-family file-tree checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeFileTreeChecksInput {
    /// Roots to evaluate for structural caps.
    pub roots: Vec<G3RsCodeStructuralCapRoot>,
}
