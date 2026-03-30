use clap::{Parser, Subcommand};

use guardrail3_validation_model::RustValidateFamily;

fn validate_format(value: &str, _ctx: &()) -> garde::Result {
    match value {
        "text" | "json" | "md" | "markdown" => Ok(()),
        _ => Err(garde::Error::new(format!(
            "invalid format '{value}', must be text|json|md|markdown"
        ))),
    }
}

#[derive(Parser, Debug, garde::Validate)]
#[command(name = "guardrail3")]
#[command(about = "Composable code guardrails for Rust and TypeScript projects")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    #[garde(skip)] // reason: subcommand validated by clap
    command: Commands,
}

impl Cli {
    #[must_use]
    pub fn into_command(self) -> Commands {
        self.command
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Rust guardrails
    Rs {
        #[command(subcommand)]
        command: RsCommands,
    },

    #[cfg(feature = "product-ts")]
    /// TypeScript guardrails
    Ts {
        #[command(subcommand)]
        command: TsCommands,
    },

    /// Generate `GUARDRAIL3_GUIDE.md` in the current directory
    DumpGuide,

    /// Dump the project tree as JSON (structure + cached config content)
    DumpTree {
        /// Project path
        #[arg(default_value = ".")]
        path: String,
    },

    /// Crawl project and show discovered structure
    Map {
        /// Project path
        #[arg(default_value = ".")]
        path: String,
        /// Show clippy.toml coverage map
        #[arg(long)]
        clippy: bool,
        /// Show deny.toml coverage map
        #[arg(long)]
        deny: bool,
        /// Show rustfmt.toml coverage map
        #[arg(long)]
        rustfmt: bool,
        /// Show `ESLint` coverage map
        #[arg(long)]
        eslint: bool,
        /// Show Stylelint coverage map
        #[arg(long)]
        stylelint: bool,
        /// Show Prettier coverage map
        #[arg(long)]
        prettier: bool,
        /// Show cspell coverage map
        #[arg(long)]
        cspell: bool,
        /// Show jscpd coverage map
        #[arg(long)]
        jscpd: bool,
        /// Show tsconfig coverage map
        #[arg(long)]
        tsconfig: bool,
        /// Show rust-toolchain coverage map
        #[arg(long)]
        rust_toolchain: bool,
        /// Show .npmrc coverage map
        #[arg(long)]
        npmrc: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum RsCommands {
    /// Initialize Rust guardrail3 configuration
    Init {
        /// Profile: "service" (HTTP/CLI binary) or "library" (pure logic, no I/O)
        #[arg(long, default_value = "service")]
        profile: String,
        /// Project path
        #[arg(default_value = ".")]
        path: String,
        /// Overwrite existing files
        #[arg(long)]
        force: bool,
        /// Show what would change without writing (dry run)
        #[arg(long)]
        dry_run: bool,
    },
    #[cfg(feature = "product-rs-generate")]
    /// Generate Rust config files (clippy.toml, deny.toml, etc.) from guardrail3.toml
    Generate(GenerateArgs),
    /// Validate Rust project guardrails
    Validate(RsValidateArgs),
    #[cfg(feature = "product-rs-generate")]
    /// Verify generated Rust configs are current (for CI)
    Check(PathArg),
    #[cfg(feature = "product-rs-generate")]
    /// Install Rust pre-commit hook
    HooksInstall(GenerateArgs),
    /// List embedded Rust config modules
    ListModules,
    /// Show contents of an embedded Rust module
    ShowModule(ShowModuleArgs),
}

#[cfg(feature = "product-ts")]
#[derive(Subcommand, Debug)]
pub enum TsCommands {
    /// Initialize TypeScript guardrail3 configuration
    Init {
        /// Project path
        #[arg(default_value = ".")]
        path: String,
        /// Overwrite existing files
        #[arg(long)]
        force: bool,
        /// Show what would change without writing (dry run)
        #[arg(long)]
        dry_run: bool,
    },
    /// Generate TypeScript config files (eslint, tsconfig, etc.) from guardrail3.toml
    Generate(GenerateArgs),
    /// Validate TypeScript project guardrails
    Validate(TsValidateArgs),
    /// Install TypeScript pre-commit hook
    HooksInstall(GenerateArgs),
    #[cfg(feature = "product-hooks")]
    /// Validate TypeScript pre-commit hook configuration
    HooksValidate(TsValidateArgs),
}

#[derive(Parser, Debug, Clone, garde::Validate)]
#[allow(clippy::struct_excessive_bools)] // reason: CLI argument struct — each bool is an independent flag
pub struct RsValidateArgs {
    /// Output format
    #[arg(long, default_value = "text")]
    #[garde(custom(validate_format))]
    format: String,

    /// Only check staged files (git diff --cached)
    #[arg(long, group = "scope")]
    #[garde(skip)] // reason: boolean flag, inherently valid
    staged: bool,

    /// Only check dirty files (staged + unstaged)
    #[arg(long, group = "scope")]
    #[garde(skip)] // reason: boolean flag, inherently valid
    dirty: bool,

    /// Only check files changed in last N commits
    #[arg(long, group = "scope")]
    #[garde(skip)] // reason: type-validated by clap
    commits: Option<usize>,

    /// Specific files to check
    #[arg(long, group = "scope")]
    #[garde(inner(length(min = 1)))] // reason: each file path must be non-empty
    files: Vec<String>,

    /// Project path (defaults to current directory)
    #[arg(default_value = ".")]
    #[garde(length(min = 1))] // reason: path must be non-empty
    path: String,

    /// Restrict Rust validation to the selected family. Repeatable.
    #[arg(long = "family", value_enum)]
    #[garde(skip)] // reason: clap validates enum values
    families: Vec<RustValidateFamilyArg>,

    /// Run slow checks (cargo publish --dry-run, etc.)
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    thorough: bool,

    /// Show passing confirmation checks (e.g., 'clippy.toml exists', 'lint X correct'). These are hidden by default because they require no action.
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    inventory: bool,

    /// Show all individual items for summarized checks (e.g., list each #[allow] instead of showing count). Without this, checks with more than 5 items are summarized to a single line.
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    verbose: bool,
}

impl RsValidateArgs {
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    #[must_use]
    pub const fn staged(&self) -> bool {
        self.staged
    }

    #[must_use]
    pub const fn dirty(&self) -> bool {
        self.dirty
    }

    #[must_use]
    pub const fn commits(&self) -> Option<usize> {
        self.commits
    }

    #[must_use]
    pub fn files(&self) -> &[String] {
        &self.files
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub fn families(&self) -> &[RustValidateFamilyArg] {
        &self.families
    }

    #[must_use]
    pub const fn thorough(&self) -> bool {
        self.thorough
    }

    #[must_use]
    pub const fn inventory(&self) -> bool {
        self.inventory
    }

    #[must_use]
    pub const fn verbose(&self) -> bool {
        self.verbose
    }
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustValidateFamilyArg {
    Arch,
    Fmt,
    Toolchain,
    Clippy,
    Deny,
    Cargo,
    Code,
    Hexarch,
    Libarch,
    Deps,
    Garde,
    Test,
    Release,
    HooksShared,
    HooksRs,
}

impl From<RustValidateFamilyArg> for RustValidateFamily {
    fn from(value: RustValidateFamilyArg) -> Self {
        match value {
            RustValidateFamilyArg::Arch => Self::Arch,
            RustValidateFamilyArg::Fmt => Self::Fmt,
            RustValidateFamilyArg::Toolchain => Self::Toolchain,
            RustValidateFamilyArg::Clippy => Self::Clippy,
            RustValidateFamilyArg::Deny => Self::Deny,
            RustValidateFamilyArg::Cargo => Self::Cargo,
            RustValidateFamilyArg::Code => Self::Code,
            RustValidateFamilyArg::Hexarch => Self::Hexarch,
            RustValidateFamilyArg::Libarch => Self::Libarch,
            RustValidateFamilyArg::Deps => Self::Deps,
            RustValidateFamilyArg::Garde => Self::Garde,
            RustValidateFamilyArg::Test => Self::Test,
            RustValidateFamilyArg::Release => Self::Release,
            RustValidateFamilyArg::HooksShared => Self::HooksShared,
            RustValidateFamilyArg::HooksRs => Self::HooksRs,
        }
    }
}

#[derive(Parser, Debug, Clone, garde::Validate)]
#[allow(clippy::struct_excessive_bools)] // reason: CLI argument struct — each bool is an independent flag
pub struct TsValidateArgs {
    /// Output format
    #[arg(long, default_value = "text")]
    #[garde(custom(validate_format))]
    format: String,

    /// Only check staged files (git diff --cached)
    #[arg(long, group = "scope")]
    #[garde(skip)] // reason: boolean flag, inherently valid
    staged: bool,

    /// Only check dirty files (staged + unstaged)
    #[arg(long, group = "scope")]
    #[garde(skip)] // reason: boolean flag, inherently valid
    dirty: bool,

    /// Only check files changed in last N commits
    #[arg(long, group = "scope")]
    #[garde(skip)] // reason: type-validated by clap
    commits: Option<usize>,

    /// Specific files to check
    #[arg(long, group = "scope")]
    #[garde(inner(length(min = 1)))] // reason: each file path must be non-empty
    files: Vec<String>,

    /// Project path (defaults to current directory)
    #[arg(default_value = ".")]
    #[garde(length(min = 1))] // reason: path must be non-empty
    path: String,

    /// Show passing confirmation checks (e.g., 'config exists'). These are hidden by default because they require no action.
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    inventory: bool,

    /// Show all individual items for summarized checks instead of a single summary line.
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    verbose: bool,
}

impl TsValidateArgs {
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    #[must_use]
    pub const fn staged(&self) -> bool {
        self.staged
    }

    #[must_use]
    pub const fn dirty(&self) -> bool {
        self.dirty
    }

    #[must_use]
    pub const fn commits(&self) -> Option<usize> {
        self.commits
    }

    #[must_use]
    pub fn files(&self) -> &[String] {
        &self.files
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub const fn inventory(&self) -> bool {
        self.inventory
    }

    #[must_use]
    pub const fn verbose(&self) -> bool {
        self.verbose
    }
}

#[derive(Parser, Debug, garde::Validate)]
pub struct GenerateArgs {
    /// Project path
    #[arg(default_value = ".")]
    #[garde(length(min = 1))] // reason: path must be non-empty
    path: String,

    /// Show what would change without writing (dry run)
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    dry_run: bool,

    /// Dump generated files to this directory instead of just showing summary (requires --dry-run)
    #[arg(long)]
    #[garde(inner(length(min = 1)))] // reason: if provided, path must be non-empty
    dump_dir: Option<String>,
}

impl GenerateArgs {
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub const fn dry_run(&self) -> bool {
        self.dry_run
    }

    #[must_use]
    pub fn dump_dir(&self) -> Option<&str> {
        self.dump_dir.as_deref()
    }
}

#[derive(Parser, Debug, garde::Validate)]
pub struct PathArg {
    /// Project path
    #[arg(default_value = ".")]
    #[garde(length(min = 1))] // reason: path must be non-empty
    path: String,
}

impl PathArg {
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Parser, Debug, garde::Validate)]
pub struct ShowModuleArgs {
    /// Module name
    #[garde(length(min = 1))] // reason: module name must be non-empty
    name: String,
}

impl ShowModuleArgs {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
