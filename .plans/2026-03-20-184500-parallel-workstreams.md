# Parallel Testing Workstreams

**Date:** 2026-03-20 18:45

## Current State

RS-ARCH-01 testing is in progress: rules 01-04 done (162 tests), rules 05-12 remaining. This document outlines 3 independent workstreams that can run in parallel.

All workstreams share the same golden fixture at `tests/fixtures/r_arch_01/golden/` and the same test infrastructure (copy golden → mutate → run check → assert).

## Golden Fixture Structure

```
golden/
├── Cargo.toml                    # root (NOT workspace, just marker)
├── packages/
│   ├── shared-types/             # Rust lib (Cargo.toml + src/lib.rs)
│   └── ui-kit/                   # TS package (package.json + src/index.ts)
├── apps/
│   ├── devctl/                   # Rust CLI, simple hex arch (5 crates)
│   │   ├── Cargo.toml            # [workspace]
│   │   └── crates/{domain/types, app/core, ports/{inbound/.gitkeep, outbound/traits}, adapters/{inbound/cli, outbound/fs}}
│   ├── backend/                  # Rust server, hex-in-hex (MCP adapter)
│   │   ├── Cargo.toml            # [workspace]
│   │   └── crates/...            # 10 outer crates + mcp/crates/ inner hex (3 crates + 3 .gitkeep)
│   ├── worker/                   # Rust async worker, simple hex (6 crates)
│   │   ├── Cargo.toml            # [workspace]
│   │   └── crates/...
│   ├── admin/                    # Next.js service app, TS hex arch
│   │   ├── package.json          # has "next" in deps
│   │   └── src/
│   │       ├── app/              # Next.js App Router
│   │       ├── modules/          # TS hex arch
│   │       │   ├── domain/types/
│   │       │   ├── ports/{inbound/use-cases/, outbound/validator/}
│   │       │   ├── application/commands/
│   │       │   └── adapters/{inbound/api/, outbound/validator/}
│   │       ├── components/
│   │       └── lib/
│   └── landing/                  # Next.js content site, NO hex arch
│       ├── package.json          # has "velite" in devDeps (content signal)
│       ├── content/              # content dir (content signal)
│       └── src/{app/, components/, lib/, types/, i18n/}
```

## Test Infrastructure

Every test file imports from `super::helpers` which provides:
- `copy_golden()` → `tempfile::TempDir` with golden copied
- `run_check(root)` → `Vec<CheckResult>` (runs R-ARCH-01 for Rust checks)
- `arch_01_errors(results)` → filters to R-ARCH-01 errors
- `assert_single_error`, `remove_dir`, `remove_file`, `write_file`

For new workstreams, create equivalent helpers that run the relevant check function.

## Test Quality Standards (established by RS-ARCH-01)

Every test MUST:
1. Break the thing in EVERY place it can break simultaneously
2. Assert EXACT error count (`assert_eq!`, never `>=` or `!is_empty()`)
3. Assert title content (violation type + specific content)
4. Assert file field points to correct path
5. Assert per-app attribution (each app name in errors)
6. Assert TS apps NOT flagged by RS checks (and vice versa)
7. Assert packages NOT flagged
8. Assert inner hex label_prefix correct in title
9. Also break same structure in places the check should NOT flag, assert 0 false positives

After writing, send adversarial agents:
- Agent 1: Does each test break everywhere + assert no false positives?
- Agent 2: What scenarios are missing?
- Agent 3: Parity with RS-ARCH-01 patterns where applicable
- Agent 4: Verify all findings implemented

---

## Workstream 1: TS-ARCH-01/02 (TypeScript Hex Arch)

### What it checks

**T-ARCH-01**: TS service apps must have `src/modules/{domain, ports/{inbound,outbound}, application, adapters/{inbound,outbound}}`. Structural rules mirror RS-ARCH-01:
- `modules/` must contain exactly `{domain, ports, application, adapters}`
- `adapters/` and `ports/` must contain exactly `{inbound, outbound}`
- Only `.gitkeep` in structural/container dirs
- Container subdirs must have `.ts`/`.tsx` files, `.gitkeep`, or `modules/` (hex-in-hex)
- Hex-in-hex recursion via `modules/` (TS equivalent of `crates/`)

**T-ARCH-02**: Import boundary violations between layers (domain can't import adapters, etc.)

### Check code

`crates/app/ts/validate/ts_arch_checks.rs` — already rewritten with full structural enforcement.

### Golden fixture coverage

- `admin` — TS service app with full hex arch in `src/modules/`. T-ARCH-01 should pass with 0 errors.
- `landing` — TS content site (velite in devDeps, `content/` dir). Auto-detected as content type, architecture checks DISABLED.
- Rust apps (devctl, backend, worker) — should NOT be checked by TS checks.

### Test structure

```
tests/unit/ts_arch_01/
├── mod.rs
├── helpers.rs          # copy_golden, run_ts_check, ts_arch_errors, etc.
├── golden.rs           # golden passes with 0 T-ARCH-01 errors
├── rule_01.rs          # modules/ must exist (TS service apps)
├── rule_02.rs          # exact contents {domain, ports, application, adapters}
├── rule_03.rs          # inbound/outbound in adapters/ and ports/
├── rule_04.rs          # loose files
├── rule_05.rs          # container not empty
├── rule_06.rs          # leaf valid (.ts files or modules/ hex-in-hex)
└── rule_07.rs          # import boundaries (T-ARCH-02)
```

### Key differences from RS-ARCH-01

- Root container: `src/modules/` not `crates/`
- App layer: `application` not `app` (Next.js takes `app/`)
- Leaf marker: `.ts`/`.tsx` files not `Cargo.toml`
- Hex-in-hex marker: `modules/` not `crates/`
- Content sites auto-detected and SKIPPED (velite/contentlayer in deps)
- `helpers.rs` needs: `run_ts_check(root)` that calls `ts validate` equivalent
- False positive: Rust apps should NOT be checked by TS checks
- The `check_single_app_structure` function and `auto_detect_app_type` function need to be called correctly

### Helper function

```rust
fn run_ts_check(root: &Path) -> Vec<CheckResult> {
    let fs = RealFileSystem;
    guardrail3::app::ts::validate::ts_arch_checks::check_hex_arch_structure(&fs, root)
}
```

---

## Workstream 2: Garde Boundary Validation (R-GARDE-01/02)

### What it checks

**R-GARDE-01**: `garde` must be in `[workspace.dependencies]` or `[dependencies]`. Every project MUST have garde for runtime input validation.

**R-GARDE-02**: `clippy.toml` must have garde method/type bans (serde_json::from_str, axum::Json, etc.) to force `Validated<T>::new()` wrapper pattern.

### Check code

`crates/app/rs/validate/garde_checks.rs`

### Golden fixture needs

The golden fixture currently has NO garde dependency in any app's Cargo.toml. This means R-GARDE-01 should fire for all Rust apps. To make the golden pass:
- Add `garde = { version = "0.22", features = ["derive"] }` to `[workspace.dependencies]` in each Rust app's workspace Cargo.toml
- Or: test that the golden correctly FAILS R-GARDE and fix accordingly

### Test structure

```
tests/unit/rs_garde/
├── mod.rs
├── helpers.rs
├── golden.rs
├── r_garde_01.rs       # garde dependency missing/present
└── r_garde_02.rs       # clippy.toml garde bans
```

### Scenarios

- garde in workspace.dependencies → pass
- garde missing entirely → error
- garde in one crate but not workspace → check behavior
- garde in devDependencies only → check behavior
- clippy.toml has garde bans → pass
- clippy.toml missing garde bans → error
- clippy.toml missing entirely → error
- TS apps should not be checked
- packages should not be checked

---

## Workstream 3: Test Quality Checks (R-TEST-01 through R-TEST-09)

### What it checks

- **R-TEST-02**: `.cargo/mutants.toml` exists
- **R-TEST-03**: `[profile.mutants]` in Cargo.toml
- **R-TEST-04**: At least one `#[test]` or `#[tokio::test]` function exists
- **R-TEST-06**: `tests/` directory with `.rs` files exists (integration tests)
- **R-TEST-08**: Mutation testing hook in `.claude/` or pre-commit
- **R-TEST-09**: Test organization (inline vs separate files)

### Check code

`crates/app/rs/validate/test_checks.rs`

### Golden fixture needs

The golden fixture has no test files (it's a structural skeleton). To make tests useful:
- Some tests verify ABSENCE (golden has no tests → R-TEST-04 fires)
- Some tests verify PRESENCE (add test files to golden, verify checks pass)
- Mutation tests copy golden and add/remove test infrastructure

### Test structure

```
tests/unit/rs_test_quality/
├── mod.rs
├── helpers.rs
├── r_test_02.rs        # mutants.toml
├── r_test_03.rs        # profile.mutants
├── r_test_04.rs        # tests exist
├── r_test_06.rs        # integration tests
├── r_test_08.rs        # mutation hook
└── r_test_09.rs        # test organization
```

### Scenarios

- No test files → R-TEST-04 fires
- Add `#[test]` function → R-TEST-04 passes
- Add `#[tokio::test]` → R-TEST-04 passes
- No `.cargo/mutants.toml` → R-TEST-02 fires
- Add mutants.toml → passes
- No `[profile.mutants]` → R-TEST-03 fires
- No mutation hook → R-TEST-08 fires
- Tests inline in src/ vs separate tests/ dir → R-TEST-09 behavior

---

## Coordination

All three workstreams:
1. Share the golden fixture — any changes to golden must be coordinated
2. Use the same test quality standards (exact count, title, file field, false positives)
3. Can run `cargo test` in parallel without conflicts (different test modules)
4. Should be committed on separate branches or in clearly labeled commits

The golden fixture may need additions:
- Workstream 1: admin already has TS hex arch ✓
- Workstream 2: need garde in workspace deps for golden to pass
- Workstream 3: may need test files in golden apps

Coordinate fixture changes to avoid conflicts.
