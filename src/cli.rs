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

    /// Initialize guardrail3 configuration
    Init(InitArgs),

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
}

#[derive(Subcommand, Debug)]
pub enum TsCommands {
    /// Validate TypeScript project guardrails
    Validate(ValidateArgs),
    /// Generate TypeScript config files from guardrail3.toml
    Generate(GenerateArgs),
}

#[derive(Subcommand, Debug)]
pub enum HooksCommands {
    /// Validate pre-commit hook configuration
    Validate(ValidateArgs),
    /// Install pre-commit hooks from guardrail3.toml
    Install(GenerateArgs),
}

#[derive(Parser, Debug, Clone)]
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
}

#[derive(Parser, Debug)]
pub struct InitArgs {
    /// Profile to use
    #[arg(long, default_value = "service")]
    pub profile: String,

    /// Overwrite existing files
    #[arg(long)]
    pub force: bool,

    /// Project path
    #[arg(default_value = ".")]
    pub path: String,
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
