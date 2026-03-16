use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "guardrail3")]
#[command(about = "Composable code guardrails for Rust and TypeScript projects")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
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

    /// Generate GUARDRAIL3_GUIDE.md in the current directory
    Guide,
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
    },
    /// Generate TypeScript config files (eslint, tsconfig, etc.) from guardrail3.toml
    Generate(GenerateArgs),
    /// Validate TypeScript project guardrails
    Validate(ValidateArgs),
    /// Install TypeScript pre-commit hook
    HooksInstall(GenerateArgs),
    /// Validate TypeScript pre-commit hook configuration
    HooksValidate(ValidateArgs),
}

#[derive(Parser, Debug, Clone)]
#[allow(clippy::struct_excessive_bools)] // reason: CLI argument struct — each bool is an independent flag
pub struct ValidateArgs {
    /// Output format
    #[arg(long, default_value = "text")]
    pub format: String,

    /// Only check staged files (git diff --cached)
    #[arg(long)]
    pub staged: bool,

    /// Only check dirty files (staged + unstaged)
    #[arg(long)]
    pub dirty: bool,

    /// Only check files changed in last N commits
    #[arg(long)]
    pub commits: Option<usize>,

    /// Specific files to check
    #[arg(long)]
    pub files: Vec<String>,

    /// Project path (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: String,

    /// Only run code quality checks
    #[arg(long)]
    pub code: bool,

    /// Only run architecture checks
    #[arg(long)]
    pub architecture: bool,

    /// Only run release readiness checks
    #[arg(long)]
    pub release: bool,

    /// Only run test quality checks
    #[arg(long)]
    pub tests: bool,

    /// Run slow checks (cargo publish --dry-run, etc.)
    #[arg(long)]
    pub thorough: bool,
}

#[derive(Parser, Debug)]
pub struct GenerateArgs {
    /// Project path
    #[arg(default_value = ".")]
    pub path: String,
}

#[derive(Parser, Debug)]
pub struct PathArg {
    /// Project path
    #[arg(default_value = ".")]
    pub path: String,
}

#[derive(Parser, Debug)]
pub struct ShowModuleArgs {
    /// Module name
    pub name: String,
}
