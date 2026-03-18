# Hex Arch Refactor + Comprehensive CLI Help

**Date:** 2026-03-16 13:20
**Task:** Restructure guardrail3 as hex arch, add graf-style CLI help

## Phase 1: Hex Arch Refactor

### Current structure (flat, CLI-coupled)
```
src/
  main.rs          — CLI entry + orchestration
  cli.rs           — clap definitions
  lib.rs           — re-exports
  fs.rs            — filesystem I/O
  discover.rs      — stack detection
  config/          — guardrail3.toml parsing
  commands/        — CLI command handlers (validate, generate, init, check, diff)
  modules/         — embedded config content
  report/          — output formatting (text, json, markdown)
  rs/validate/     — Rust validation logic
  ts/validate/     — TypeScript validation logic
  hooks/           — Hook validation logic
```

### Target structure (hex arch)
```
src/
  main.rs              — CLI adapter (thin: parse args → call lib)
  cli.rs               — clap definitions + help injection
  help_gen.rs          — runtime help text generation
  lib.rs               — public API (the "port")

  domain/              — pure logic, no I/O
    validate/          — validation orchestration
      rs/              — Rust checks (moved from src/rs/validate/)
      ts/              — TypeScript checks (moved from src/ts/validate/)
      hooks/           — Hook checks (moved from src/hooks/)
    modules/           — embedded config content (moved from src/modules/)
    config/            — config types (moved from src/config/)
    report/            — report types (CheckResult, Section, Report)
    discover.rs        — stack detection logic (pure: takes file list, returns stacks)

  adapters/
    fs.rs              — filesystem adapter (the only place std::fs is used)
    cli_commands.rs    — CLI command handlers (moved from src/commands/)
    text_reporter.rs   — colored terminal output
    json_reporter.rs   — JSON output
    markdown_reporter.rs — markdown output
```

### Key principle
- `domain/` has ZERO I/O. It receives content as &str, returns CheckResults.
- `adapters/` does all I/O: reading files, writing output, CLI parsing.
- `lib.rs` exposes a clean public API that adapters call.
- The validation functions already take `content: &str` — they're ALREADY pure. The refactor is mostly moving files and updating imports.

### What actually needs to change
1. Move `src/rs/`, `src/ts/`, `src/hooks/` into `src/domain/validate/`
2. Move `src/modules/` into `src/domain/modules/`
3. Move `src/config/` into `src/domain/config/`
4. Move `src/report/types.rs` into `src/domain/report/`
5. Move `src/report/{text,json,markdown}.rs` into `src/adapters/`
6. Move `src/commands/` into `src/adapters/cli_commands/`
7. Move `src/fs.rs` into `src/adapters/fs.rs`
8. Update `src/discover.rs` — split pure logic from I/O
9. Update ALL imports across the entire codebase
10. Update `lib.rs` to expose clean public API

## Phase 2: Comprehensive CLI Help

### Approach (from graf)
- `help_gen.rs` module that builds `after_help` text programmatically
- Inject into clap command tree at runtime: `help_gen::inject_help(Cli::command())`
- NO build.rs, NO codegen — pure runtime assembly

### Help content for each command
- Top-level: full command tree, workflow guide, what each command does
- `rs validate`: all Rust check IDs with descriptions, examples
- `ts validate`: all TypeScript check IDs with descriptions, examples
- `rs init`: what it creates, profiles explained, examples
- `ts init`: what it creates, examples
- `generate`: what files it produces, when to re-run
- `check`: CI usage, staleness detection
- `diff`: dry-run usage

### Check list generation
- Each check module defines its check IDs as constants or in a registry
- `help_gen.rs` reads the registry and formats the check list
- This keeps help in sync with actual checks — add a check, help updates automatically

## Risks
- Import changes touch EVERY file in the project
- Test imports change too
- The domain/ prefix adds verbosity to import paths
