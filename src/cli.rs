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
    /// Validate project guardrails (auto-detects stacks)
    Validate(ValidateArgs),

    /// Generate config files from guardrail3.toml
    Generate(GenerateArgs),

    /// Check if generated files are current
    Check(PathArg),

    /// Show what generate would change (dry run)
    Diff(PathArg),

    /// List available modules
    ListModules,

    /// Show contents of a module
    ShowModule(ShowModuleArgs),

    /// Rust-specific commands
    Rs {
        #[command(subcommand)]
        command: RsCommands,
    },

    /// TypeScript-specific commands
    Ts {
        #[command(subcommand)]
        command: TsCommands,
    },

    /// Pre-commit hook commands
    Hooks {
        #[command(subcommand)]
        command: HooksCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum RsCommands {
    /// Validate Rust project guardrails
    Validate(ValidateArgs),
    /// Generate Rust config files from guardrail3.toml
    Generate(GenerateArgs),
    /// Initialize Rust guardrail3 configuration
    Init {
        /// Profile to use
        #[arg(long, default_value = "service")]
        profile: String,
        /// Project path
        #[arg(default_value = ".")]
        path: String,
        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum TsCommands {
    /// Validate TypeScript project guardrails
    Validate(ValidateArgs),
    /// Generate TypeScript config files from guardrail3.toml
    Generate(GenerateArgs),
    /// Initialize TypeScript guardrail3 configuration
    Init {
        /// Project path
        #[arg(default_value = ".")]
        path: String,
        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum HooksCommands {
    /// Validate pre-commit hook configuration
    Validate(ValidateArgs),
    /// Install pre-commit hooks from guardrail3.toml
    Install(GenerateArgs),
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
