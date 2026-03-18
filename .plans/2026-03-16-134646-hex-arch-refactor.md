# Hex Architecture Refactor — Ports, Adapters, App Layer

**Date:** 2026-03-16 13:46
**Task:** Full hexagonal architecture refactor of guardrail3

## Goal
Restructure the codebase into proper hex arch layers:
- `src/domain/` — pure types only (report.rs, config/, modules/)
- `src/ports/` — trait definitions (FileSystem, ToolChecker)
- `src/app/` — business logic (validation, discovery), depends on domain+ports
- `src/adapters/` — I/O implementations (real FS, real tool runner, report formatters)

## Approach

### Phase 1: Create ports (src/ports/)
- Create `src/ports/mod.rs` and `src/ports/outbound.rs`
- Define `FileSystem` and `ToolChecker` traits

### Phase 2: Move validation to src/app/
- `git mv src/domain/validate/ src/app/` (preserves git history)
- `git mv src/domain/discover.rs src/app/discover.rs`
- Update all module paths from `crate::domain::validate::` to `crate::app::`
- Update `crate::domain::discover` to `crate::app::discover`

### Phase 3: Thread FileSystem and ToolChecker through validation
- Every function calling `crate::fs::*` gets `fs: &dyn FileSystem` param
- Every function calling `Command::new("which")` gets `tc: &dyn ToolChecker` param
- Orchestrators pass these down to check functions
- ~37 files use `crate::fs::`, ~6 files use `Command` for tool checking

### Phase 4: Move adapters
- `git mv src/report/ src/adapters/inbound/report/`
- Move `src/fs.rs` content to `src/adapters/outbound/fs.rs` (implement FileSystem trait)
- Create `src/adapters/outbound/tool_runner.rs` (implement ToolChecker trait)
- Keep original `src/fs.rs` as a thin wrapper or remove

### Phase 5: Wire up in commands/ and main.rs
- Create RealFileSystem and RealToolChecker instances in command handlers
- Pass them to app layer orchestrators
- Update all import paths in main.rs

### Phase 6: Update lib.rs
- Declare new modules: `ports`, `app`, `adapters`
- Remove old module declarations

## Key Decisions
- **Keep `commands/` at top level:** They're the CLI inbound adapter, keeping them at top level is fine for now
- **Thread traits through function params:** Not using Arc/global state, just &dyn trait references passed top-down
- **discover.rs moves to app/:** It uses fs operations, so it's application logic not pure domain

## Risks
- Many files to update (37+ use crate::fs, 6+ use Command)
- Test modules reference crate::fs directly — tests can use the real adapter
- Property tests and integration tests reference old paths
- Self-validation checks (R58) scan for `crate::fs::` usage — need to update the check
