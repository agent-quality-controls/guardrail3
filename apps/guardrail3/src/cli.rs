use clap::{Parser, Subcommand};

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
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Rust guardrails
    Rs {
        #[command(subcommand)]
        command: RsCommands,
    },

    /// TypeScript guardrails
    Ts {
        #[command(subcommand)]
        command: TsCommands,
    },

    /// Generate `GUARDRAIL3_GUIDE.md` in the current directory
    DumpGuide,

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
        /// Output format: text or json
        #[arg(long, default_value = "text")]
        format: String,
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
    /// Generate Rust config files (clippy.toml, deny.toml, etc.) from guardrail3.toml
    Generate(GenerateArgs),
    /// Validate Rust project guardrails
    Validate(ValidateArgs),
    /// Verify generated Rust configs are current (for CI)
    Check(PathArg),
    /// Show what rs generate would change (dry run)
    Diff(PathArg),
    /// Install Rust pre-commit hook
    HooksInstall(GenerateArgs),
    /// Validate Rust pre-commit hook configuration
    HooksValidate(ValidateArgs),
    /// List embedded Rust config modules
    ListModules,
    /// Show contents of an embedded Rust module
    ShowModule(ShowModuleArgs),
}

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
    Validate(ValidateArgs),
    /// Show what ts generate would change (dry run)
    Diff(PathArg),
    /// Install TypeScript pre-commit hook
    HooksInstall(GenerateArgs),
    /// Validate TypeScript pre-commit hook configuration
    HooksValidate(ValidateArgs),
}

#[derive(Parser, Debug, Clone, garde::Validate)]
#[allow(clippy::struct_excessive_bools)] // reason: CLI argument struct — each bool is an independent flag
pub struct ValidateArgs {
    /// Output format
    #[arg(long, default_value = "text")]
    #[garde(custom(validate_format))]
    pub format: String,

    /// Only check staged files (git diff --cached)
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub staged: bool,

    /// Only check dirty files (staged + unstaged)
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub dirty: bool,

    /// Only check files changed in last N commits
    #[arg(long)]
    #[garde(skip)] // reason: type-validated by clap
    pub commits: Option<usize>,

    /// Specific files to check
    #[arg(long)]
    #[garde(inner(length(min = 1)))] // reason: each file path must be non-empty
    pub files: Vec<String>,

    /// Project path (defaults to current directory)
    #[arg(default_value = ".")]
    #[garde(length(min = 1))] // reason: path must be non-empty
    pub path: String,

    /// Only run code quality checks
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub code: bool,

    /// Only run architecture checks
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub architecture: bool,

    /// Only run release readiness checks
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub release: bool,

    /// Only run test quality checks
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub tests: bool,

    /// Only run garde boundary validation checks
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub garde: bool,

    /// Run slow checks (cargo publish --dry-run, etc.)
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub thorough: bool,

    /// Show passing confirmation checks (e.g., 'clippy.toml exists', 'lint X correct'). These are hidden by default because they require no action.
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub inventory: bool,

    /// Show all individual items for summarized checks (e.g., list each #[allow] instead of showing count). Without this, checks with more than 5 items are summarized to a single line.
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub verbose: bool,
}

#[derive(Parser, Debug, garde::Validate)]
pub struct GenerateArgs {
    /// Project path
    #[arg(default_value = ".")]
    #[garde(length(min = 1))] // reason: path must be non-empty
    pub path: String,

    /// Show what would change without writing (dry run)
    #[arg(long)]
    #[garde(skip)] // reason: boolean flag, inherently valid
    pub dry_run: bool,
}

#[derive(Parser, Debug, garde::Validate)]
pub struct PathArg {
    /// Project path
    #[arg(default_value = ".")]
    #[garde(length(min = 1))] // reason: path must be non-empty
    pub path: String,
}

#[derive(Parser, Debug, garde::Validate)]
pub struct ShowModuleArgs {
    /// Module name
    #[garde(length(min = 1))] // reason: module name must be non-empty
    pub name: String,
}
