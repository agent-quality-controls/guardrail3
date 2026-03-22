# Split guardrail3 into real hex arch crates

**Date:** 2026-03-20 11:45
**Task:** Replace the monolithic single-crate with real independent crates that Cargo compiles separately

## Goal

guardrail3 must pass its own R-ARCH-01 check — no files at `crates/` root, every subdir is a real workspace member. The Rust compiler enforces layer boundaries because cross-crate imports require explicit `[dependencies]`.

## Current state

One monolithic crate. `lib.rs` at `crates/` root defines the entire module tree with `#[path]` hacks. Stub Cargo.tomls exist in subdirs but aren't workspace members. Everything uses `use crate::` imports. R-ARCH-01 now correctly flags `lib.rs`, `main.rs`, `fs.rs` at `crates/` root.

## Dependency violations to fix BEFORE splitting

These are hex arch violations in the current code — domain calling adapters, etc. Must be fixed first or the crate split is impossible (circular deps).

### V1: domain/modules → adapters (domain depends on adapters)

`domain/modules/clippy.rs` and `domain/modules/deny.rs` call `crate::adapters::inbound::cli::generate::deduplicated_override()`.

**Fix:** Move `deduplicated_override()` from `adapters/inbound/cli/generate_helpers.rs` to `domain/modules/`. It's a pure string deduplication function — it has no adapter dependencies. It belongs in domain.

### V2: `crate::fs` — centralized filesystem at crates root

`fs.rs` at `crates/` root provides free functions (`read_file`, `write_file`, etc.) wrapping `std::fs`. Used by:
- `adapters/outbound/fs/mod.rs` (RealFileSystem delegates to it)
- `adapters/inbound/cli/` (generate, init, check, diff, coverage, map — ~50 call sites)
- `app/core/project_map.rs` (~5 call sites)

**Fix:** Merge `fs.rs` into `adapters/outbound/fs/`. The free functions become methods or associated functions on `RealFileSystem`, or stay as module-level functions inside that crate. `app/core/project_map.rs` currently calls `crate::fs::read_file` directly — that's an app→adapter violation. It should use the `FileSystem` port trait instead (it already receives `&dyn FileSystem` in some paths but bypasses it in others).

### V3: app/core/project_map.rs bypasses FileSystem port

`project_map.rs` calls `crate::fs::read_file()` directly instead of going through the `FileSystem` port. This is a dependency direction violation (app → adapter).

**Fix:** Pass `&dyn FileSystem` into the functions that need it, same pattern as all other app-layer code.

## Target crates (12)

### Domain layer — zero internal dependencies

| Crate | Contents | External deps |
|---|---|---|
| `domain/report` | CheckResult, Severity, Section, Report, categories, ValidateDomains | none |
| `domain/config` | GuardrailConfig, CrateConfig, profile types | `serde`, `toml` |
| `domain/modules` | Embedded module content (clippy, deny, canonical, cspell, eslint, etc.), `deduplicated_override` | `domain/report` (for TsAppType), `domain/config` |

### Ports layer — depends on domain only

| Crate | Contents | Internal deps |
|---|---|---|
| `ports/outbound/traits` | `FileSystem` trait, `ToolChecker` trait | none (traits only reference std types) |

### App layer — depends on domain + ports

| Crate | Contents | Internal deps |
|---|---|---|
| `app/core` | crawl, discover, gitignore, project_map | `domain/report`, `ports/outbound/traits` |
| `app/rs` | Rust validation (57+ checks) | `domain/report`, `domain/config`, `ports/outbound/traits`, `app/core` |
| `app/ts` | TypeScript validation (78+ checks) | `domain/report`, `domain/config`, `ports/outbound/traits`, `app/core` |
| `app/hooks` | Hook + deployment validation | `domain/report`, `ports/outbound/traits`, `app/core` |

### Adapters layer — depends on everything above

| Crate | Contents | Internal deps |
|---|---|---|
| `adapters/outbound/fs` | RealFileSystem impl + centralized fs functions | `ports/outbound/traits` |
| `adapters/outbound/tool-runner` | RealToolChecker impl | `ports/outbound/traits` |
| `adapters/outbound/report` | text/json/markdown formatters | `domain/report` |
| `adapters/inbound/cli` | clap defs, help_gen, commands/*, **main.rs** | everything — this is the composition root |

## Dependency graph

```
domain/report  ←  domain/config
     ↑                 ↑
     ├─────────────────┤
     ↑                 ↑
domain/modules         │
     ↑                 │
ports/outbound/traits  │
     ↑                 │
     ├─────────────────┤
     ↑                 ↑
app/core ──────────────┤
     ↑                 │
     ├────────┬────────┤
     ↑        ↑        ↑
  app/rs   app/ts   app/hooks
     ↑        ↑        ↑
     └────────┼────────┘
              ↑
  adapters/outbound/fs
  adapters/outbound/tool-runner
  adapters/outbound/report
              ↑
  adapters/inbound/cli (composition root, main.rs)
```

## Execution phases

### Phase 0: Fix dependency violations (V1, V2, V3)

Before any crate splitting:
1. Move `deduplicated_override` from `adapters/inbound/cli/generate_helpers.rs` to `domain/modules/`
2. Merge `crates/fs.rs` into `adapters/outbound/fs/`
3. Fix `app/core/project_map.rs` to use `&dyn FileSystem` instead of `crate::fs::` calls
4. Verify no domain→app, domain→adapter, ports→app, ports→adapter, app→adapter imports remain (except via port traits)

### Phase 1: Bottom-up crate extraction — domain + ports

Start with crates that have no internal dependencies:

1. **`domain/report`** — Extract `domain/report/mod.rs` into `domain/report/src/lib.rs`. Create real `Cargo.toml`. Add to workspace members.
2. **`ports/outbound/traits`** — Extract `ports/outbound/traits/mod.rs` into `ports/outbound/traits/src/lib.rs`. Create real `Cargo.toml`.
3. **`domain/config`** — Needs `serde`, `toml`. Depends on nothing internal.
4. **`domain/modules`** — Depends on `domain/report`, `domain/config`.

After each extraction: update the remaining monolithic crate's `Cargo.toml` to depend on the new crate via `path = "..."`. Replace `use crate::domain::report::` with `use guardrail3_domain_report::`. Verify `cargo build && cargo test`.

### Phase 2: App layer crates

5. **`app/core`** — Depends on `domain/report`, `ports/outbound/traits`. External: `walkdir`, `ignore`, `toml`, `glob`.
6. **`app/hooks`** — Depends on `domain/report`, `ports/outbound/traits`, `app/core`.
7. **`app/rs`** — Largest crate (~25 files). Depends on `domain/*`, `ports/outbound/traits`, `app/core`. External: `syn`, `toml`, `walkdir`.
8. **`app/ts`** — Depends on `domain/*`, `ports/outbound/traits`, `app/core`. External: `tree-sitter-*`, `serde_json`.

### Phase 3: Adapter crates

9. **`adapters/outbound/fs`** — Depends on `ports/outbound/traits`. This is where `std::fs` lives.
10. **`adapters/outbound/tool-runner`** — Depends on `ports/outbound/traits`.
11. **`adapters/outbound/report`** — Depends on `domain/report`. External: `colored`, `serde_json`.

### Phase 4: CLI composition root

12. **`adapters/inbound/cli`** — Depends on everything. Contains `main.rs` (binary entry point), clap defs, commands, help_gen. This is the only `[[bin]]` crate.

### Phase 5: Delete monolithic crate

- Remove `crates/lib.rs`, `crates/main.rs`, `crates/fs.rs`
- Update workspace root `Cargo.toml` — the old `apps/guardrail3` member becomes the set of 12 new members
- Update `apps/guardrail3/Cargo.toml` — becomes a workspace manifest (or delete if workspace is at repo root)
- Run R-ARCH-01 on self — must pass with 0 violations

### Phase 6: Verification

- `cargo build` — compiles all 12 crates
- `cargo test` — all 306+ tests pass
- `cargo run -p guardrail3-adapters-inbound-cli -- rs validate .` — 0 R-ARCH-01 violations
- R-ARCH-02 dependency flow — 0 violations (compiler enforces this now)
- No `use crate::` imports cross crate boundaries — compiler would reject them

## Key decisions

- **Bottom-up extraction:** Extract leaf crates first (domain/report, ports/outbound/traits), then work up. At each step the remaining monolithic code shrinks and depends on the extracted crates. This keeps the build working at every step.
- **One crate at a time:** Each extraction is a separate commit. If anything breaks, the blast radius is one crate.
- **Binary in CLI adapter:** `main.rs` lives in `adapters/inbound/cli/src/main.rs`. This is the composition root — it wires adapters to ports, creates the app, and runs.
- **`fs.rs` merges into adapters/outbound/fs:** The centralized fs module IS the outbound filesystem adapter. No separate existence needed.
- **`deduplicated_override` moves to domain:** It's a pure string function with no adapter dependencies. Domain/modules needs it, so it lives there.

## Files affected

Every `.rs` file in the crate changes — all `use crate::` imports become cross-crate imports (`use guardrail3_domain_report::`, etc.). The workspace Cargo.toml gains 12 members. 12 new `Cargo.toml` files replace the stubs. 12 new `src/lib.rs` files (or `src/main.rs` for CLI).
