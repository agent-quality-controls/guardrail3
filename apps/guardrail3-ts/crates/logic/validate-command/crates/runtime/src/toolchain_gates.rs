//! Toolchain-gate construction and execution: which external tool commands
//! the CLI invokes after the static rule pipeline.

use std::fmt::Write as _;
use std::path::Path;

use g3ts_hooks_contract_types::{G3TsHookCommandRequirement, G3TsHookRequirement, PackageManager};
use guardrail3_ts_app_types::SupportedFamily;

/// Filename of the npm/pnpm package manifest at an adopted TS unit root.
const PACKAGE_JSON: &str = "package.json";

/// Entry in the legacy fallback table mapping a family to its required
/// toolchain command until that family's hook-contract crate is created.
type LegacyGateEntry = (SupportedFamily, G3TsHookCommandRequirement);

/// Aggregated stdout/stderr/exit-code of all gate spawns inside one validate run.
#[derive(Debug, Default)]
pub(crate) struct ToolchainOutcome {
    /// Text written to stdout by all gates, prefixed by a `== label ==` header.
    pub stdout: String,
    /// Text written to stderr by all gates, plus spawn-failure lines.
    pub stderr: String,
    /// Exit code: `0` if every gate succeeded; otherwise `1`.
    pub exit_code: i32,
}

/// One concrete toolchain gate to invoke: a human-readable label and the
/// resolved argv to spawn.
#[derive(Debug, Clone)]
pub(crate) struct ToolchainGate {
    /// Human-readable label for this gate (e.g. `lint`, `typecheck`).
    pub label: &'static str,
    /// Resolved argv to spawn: `[bin, arg1, arg2, ...]`.
    pub argv: Vec<String>,
}

/// Invokes TS toolchain gates from inside `path`.
///
/// Best-effort: missing tools surface via stderr but do not crash the CLI.
/// Gates are sourced only from enabled families, so `[checks]` opt-outs cover
/// both static rules and external toolchain gates.
#[must_use]
pub(crate) fn run_toolchain_gates(
    path: &Path,
    enabled_families: &[SupportedFamily],
    include_inventory: bool,
) -> ToolchainOutcome {
    let mut outcome = ToolchainOutcome::default();
    let manager = detect_package_manager(path);
    if include_inventory {
        let _ = writeln!(&mut outcome.stdout, "== toolchain ==");
        let _ = writeln!(
            &mut outcome.stdout,
            "[Info] g3ts-toolchain/package-manager package manager detected"
        );
        let _ = writeln!(
            &mut outcome.stdout,
            "  detected: {}",
            package_manager_name(manager)
        );
    }
    for gate in toolchain_gate_list(path, manager, enabled_families) {
        let label = gate.label;
        let argv = gate.argv;
        if include_inventory {
            let _ = writeln!(
                &mut outcome.stdout,
                "[Info] g3ts-toolchain/gate {label} toolchain gate scheduled"
            );
            let _ = writeln!(&mut outcome.stdout, "  command: {}", argv.join(" "));
        }
        let Some((bin, rest)) = argv.split_first() else {
            continue;
        };
        let result = spawn_gate(bin, rest, path);
        match result {
            Ok(output) => {
                if !output.status.success() {
                    ensure_toolchain_header(&mut outcome.stdout);
                    let _ = writeln!(
                        &mut outcome.stdout,
                        "[Error] g3ts-toolchain/gate {label} toolchain gate failed"
                    );
                    let _ = writeln!(&mut outcome.stdout, "  command: {}", argv.join(" "));
                    let _ = writeln!(
                        &mut outcome.stdout,
                        "  exit-code: {}",
                        output.status.code().unwrap_or(1)
                    );
                    push_command_output("stdout", &output.stdout, &mut outcome.stdout);
                    push_command_output("stderr", &output.stderr, &mut outcome.stdout);
                    outcome.exit_code = 1;
                }
            }
            Err(error) => {
                ensure_toolchain_header(&mut outcome.stdout);
                let _ = writeln!(
                    &mut outcome.stdout,
                    "[Error] g3ts-toolchain/gate {label} toolchain gate failed"
                );
                let _ = writeln!(&mut outcome.stdout, "  command: {}", argv.join(" "));
                let _ = writeln!(&mut outcome.stdout, "  spawn-error: {bin}: {error}");
                outcome.exit_code = 1;
            }
        }
    }
    outcome
}

/// Adds the toolchain section header unless it is already present.
fn ensure_toolchain_header(output: &mut String) {
    if output.contains("== toolchain ==") {
        return;
    }
    let _ = writeln!(output, "== toolchain ==");
}

/// Appends a bounded stdout or stderr snippet to one toolchain finding.
fn push_command_output(label: &str, bytes: &[u8], output: &mut String) {
    let text = String::from_utf8_lossy(bytes);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return;
    }
    let snippet: String = trimmed.chars().take(1200).collect();
    let _ = writeln!(output, "  {label}: {snippet}");
}

/// Returns the CLI-visible name for one detected package manager.
const fn package_manager_name(manager: PackageManager) -> &'static str {
    match manager {
        PackageManager::Pnpm => "pnpm",
        PackageManager::Yarn => "yarn",
        PackageManager::Npm => "npm",
        PackageManager::Bun => "bun",
    }
}

/// Centralized process-spawn boundary for toolchain gates. This is the one
/// place this module reaches out to `std::process::Command`: every gate
/// invocation flows through here so the ban on shell execution has a single
/// audited entry point.
#[expect(
    clippy::disallowed_methods,
    reason = "this function is the centralized process-spawn boundary for toolchain-gate execution; gate argv comes from typed G3TsHookCommandRequirement::concrete_command resolution, not user-supplied shell input"
)]
fn spawn_gate(bin: &String, rest: &[String], path: &Path) -> std::io::Result<std::process::Output> {
    std::process::Command::new(bin)
        .args(rest)
        .current_dir(path)
        .output()
}

/// Returns the hook contract requirements owned by the given family. Each
/// family's contract is the single source of truth for what runnable toolchain
/// gate commands that family demands. Families with no concrete toolchain
/// gate (`Hooks`, the Astro families, etc.) return an empty vector.
///
/// Gap report: families whose hook-contract crate does not yet exist
/// (`Tsconfig`, `Eslint`, `Package`, `Jscpd`) return an empty vector here.
/// Their toolchain gates are sourced from
/// `LEGACY_HARDCODED_GATES_FOR_MISSING_CONTRACTS` below until those crates
/// are created. Once a family's hook-contract crate exists, add the
/// appropriate `G3TsHookCommandRequirement` to its `required_commands` and
/// remove its entry from the legacy fallback.
fn family_hook_contract(family: SupportedFamily) -> Vec<G3TsHookRequirement> {
    match family {
        SupportedFamily::Fmt => g3ts_fmt_hook_contract::hook_contract(),
        SupportedFamily::Spelling => g3ts_spelling_hook_contract::hook_contract(),
        SupportedFamily::Style => g3ts_style_hook_contract::hook_contract(),
        SupportedFamily::Typecov => g3ts_typecov_hook_contract::hook_contract(),
        SupportedFamily::Tsconfig
        | SupportedFamily::Eslint
        | SupportedFamily::Package
        | SupportedFamily::Jscpd
        | SupportedFamily::Hooks
        | SupportedFamily::Topology
        | SupportedFamily::Arch
        | SupportedFamily::Apparch
        | SupportedFamily::Npmrc
        | SupportedFamily::AstroSetup
        | SupportedFamily::AstroContent
        | SupportedFamily::AstroMdx
        | SupportedFamily::AstroI18n
        | SupportedFamily::AstroMedia
        | SupportedFamily::AstroSeo
        | SupportedFamily::AstroState => Vec::new(),
    }
}

/// Resolves a `G3TsHookCommandRequirement` to a human-readable gate label.
/// Variants that do not map to a runnable toolchain gate (`G3TsValidatePath`,
/// `AppValidateScript`) return `None`.
const fn requirement_label(requirement: G3TsHookCommandRequirement) -> Option<&'static str> {
    match requirement {
        G3TsHookCommandRequirement::G3TsValidatePath
        | G3TsHookCommandRequirement::AppValidateScript => None,
        G3TsHookCommandRequirement::Tsc => Some("typecheck"),
        G3TsHookCommandRequirement::Eslint => Some("lint"),
        G3TsHookCommandRequirement::Prettier => Some("format-check"),
        G3TsHookCommandRequirement::Cspell => Some("spelling"),
        G3TsHookCommandRequirement::Stylelint => Some("stylelint"),
        G3TsHookCommandRequirement::SyncpackLint => Some("package-policy"),
        G3TsHookCommandRequirement::TypeCoverage => Some("typecov"),
    }
}

/// Returns true when this requirement's gate should be skipped given the
/// workspace state at `path`. Mirrors the "skip when disabled" logic the
/// previous hardcoded gate list expressed: syncpack needs a `package.json`,
/// stylelint needs a stylelint config, typecov needs a type-coverage config.
fn requirement_disabled_for_path(requirement: G3TsHookCommandRequirement, path: &Path) -> bool {
    match requirement {
        G3TsHookCommandRequirement::Stylelint => !has_stylelint_config(path),
        G3TsHookCommandRequirement::TypeCoverage => !has_typecov_config(path),
        G3TsHookCommandRequirement::SyncpackLint => !path.join(PACKAGE_JSON).is_file(),
        G3TsHookCommandRequirement::G3TsValidatePath
        | G3TsHookCommandRequirement::AppValidateScript
        | G3TsHookCommandRequirement::Tsc
        | G3TsHookCommandRequirement::Eslint
        | G3TsHookCommandRequirement::Prettier
        | G3TsHookCommandRequirement::Cspell => false,
    }
}

/// Toolchain gates for families whose hook-contract crate has not yet been
/// created. Temporary fallback that mirrors what their contracts would
/// declare. When the hook-contract crate for a family lands, plumb the
/// requirement into that crate's `required_commands` and remove the entry here.
const LEGACY_HARDCODED_GATES_FOR_MISSING_CONTRACTS: &[LegacyGateEntry] = &[
    (SupportedFamily::Tsconfig, G3TsHookCommandRequirement::Tsc),
    (SupportedFamily::Eslint, G3TsHookCommandRequirement::Eslint),
    (
        SupportedFamily::Package,
        G3TsHookCommandRequirement::SyncpackLint,
    ),
];

/// Walks each enabled family's `hook_contract()` and resolves each
/// `G3TsHookCommandRequirement` to its concrete argv.
///
/// Resolution goes through `G3TsHookCommandRequirement::concrete_command`.
/// Adding a new required command to a family's contract automatically
/// appears here. Variants whose `concrete_command()` returns `None` are
/// skipped because they do not map to a toolchain gate (e.g.
/// `G3TsValidatePath`, `AppValidateScript`).
///
/// Path-level skip rules (e.g. typecheck requires `tsconfig.json`) are
/// applied per-requirement via `requirement_disabled_for_path`.
#[must_use]
pub(crate) fn toolchain_gate_list(
    path: &Path,
    manager: PackageManager,
    enabled_families: &[SupportedFamily],
) -> Vec<ToolchainGate> {
    let mut gates: Vec<ToolchainGate> = Vec::new();
    let mut push_requirement = |requirement: G3TsHookCommandRequirement| {
        if requirement_disabled_for_path(requirement, path) {
            return;
        }
        let Some(label) = requirement_label(requirement) else {
            return;
        };
        let Some(argv) = requirement.concrete_command(manager) else {
            return;
        };
        if matches!(requirement, G3TsHookCommandRequirement::Tsc)
            && !path.join("tsconfig.json").is_file()
        {
            return;
        }
        if gates.iter().any(|gate| gate.argv == argv) {
            return;
        }
        gates.push(ToolchainGate { label, argv });
    };

    for family in enabled_families {
        for requirement in family_hook_contract(*family) {
            for command_requirement in requirement.required_commands() {
                push_requirement(*command_requirement);
            }
        }
    }
    for (family, requirement) in LEGACY_HARDCODED_GATES_FOR_MISSING_CONTRACTS {
        if !enabled_families.contains(family) {
            continue;
        }
        push_requirement(*requirement);
    }
    gates
}

/// Returns true when `path` contains any recognized stylelint config file.
fn has_stylelint_config(path: &Path) -> bool {
    [
        "stylelint.config.js",
        "stylelint.config.mjs",
        "stylelint.config.cjs",
        "stylelint.config.ts",
        ".stylelintrc",
        ".stylelintrc.json",
        ".stylelintrc.js",
        ".stylelintrc.cjs",
        ".stylelintrc.mjs",
        ".stylelintrc.yaml",
        ".stylelintrc.yml",
    ]
    .iter()
    .any(|name| path.join(name).is_file())
}

/// Returns true when `path` contains any recognized type-coverage config file.
fn has_typecov_config(path: &Path) -> bool {
    [
        "type-coverage.json",
        "type-coverage.config.js",
        "type-coverage.config.mjs",
        "type-coverage.config.cjs",
        "type-coverage.config.ts",
    ]
    .iter()
    .any(|name| path.join(name).is_file())
}

/// Walks `path` and its ancestors looking for a lockfile that identifies the
/// active package manager. Defaults to `Pnpm` when no lockfile is found.
fn detect_package_manager(path: &Path) -> PackageManager {
    for ancestor in path.ancestors() {
        if ancestor.join("pnpm-lock.yaml").is_file() {
            return PackageManager::Pnpm;
        }
        if ancestor.join("yarn.lock").is_file() {
            return PackageManager::Yarn;
        }
        if ancestor.join("bun.lockb").is_file() || ancestor.join("bun.lock").is_file() {
            return PackageManager::Bun;
        }
        if ancestor.join("package-lock.json").is_file() {
            return PackageManager::Npm;
        }
    }
    PackageManager::Pnpm
}
