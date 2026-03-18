# Monorepo Handling — Universal Design

**Date:** 2026-03-16 22:55

## Problem
guardrail3 needs to handle monorepos with multiple apps and packages, each potentially having different profiles, configs, and exclusions. Current approach is ad-hoc.

## Project Types We Must Support

1. **Single Rust crate** (binary or library)
2. **Rust workspace** (multiple crates in apps/ + packages/)
3. **Single TypeScript app** (Next.js, standalone)
4. **TypeScript monorepo** (multiple apps in apps/, packages in packages/)
5. **Mixed Rust + TypeScript monorepo** (both stacks)

## Architecture Convention (enforced)

```
project-root/
  apps/                    Services that do I/O
    app-a/                 Each app is a service
      (Rust: crates/ with hex arch)
      (TS: src/modules/ with hex arch)
    app-b/
  packages/                Shared libraries (no I/O)
    lib-x/
    lib-y/
  tests/                   Shared test infrastructure
  legacy/                  Frozen old code (gitignored)
```

## What Needs Per-App Configuration

### Rust (via guardrail3.toml [rust.crates.*])
- **Profile**: service vs library (affects clippy bans, deny bans)
- **Layer**: composition-root vs pure (affects global state allowance)
- **Allowed deps**: dependency allowlist for libraries
- **Hex arch**: R-ARCH checks enforce structure

### TypeScript (currently NO per-app config)
The TS side currently has no equivalent of `[rust.crates.*]`. It applies the same rules to all apps. This needs fixing:

**Proposed: [typescript.apps.*] config**
```toml
[typescript.apps.admin]
type = "service"            # service (needs hex arch) or "static" (marketing site, no hex arch)

[typescript.apps.landing]
type = "static"             # no hex arch required

[typescript.packages.generator]
type = "library"            # shared package
```

This would control:
- T-ARCH-01: only fire on apps with type = "service"
- T-ARCH-02: only scan imports for type = "service" apps
- ESLint rules: potentially different per app
- Test requirements: different for static sites vs services

## Exclusion Strategy

### Gitignore (DONE)
- Scanner reads `.gitignore` at project root
- Directories listed there are excluded from all scans
- Solves: legacy/, build/, dist/, node_modules/ etc.
- Universal: works for any project without config

### Tests
- `tests/` directories excluded from source scan checks (R30-R44, R58, T23-T31)
- `tests/fixtures/` excluded from all scans
- Test files still checked for: file length (R38/T32), todo macros (R43)

### Per-app type (FUTURE)
- Config declares each app's type (service, static, library)
- Checks adapt based on type
- Default if not configured: service (strictest)

## Implementation Plan

### Phase 1 (DONE)
- Gitignore respect in TS scanner
- T-ARCH-02 alias import detection (@adapters, @domain, etc.)

### Phase 2 (next)
- Add `[typescript.apps.*]` config to types.rs
- T-ARCH-01/02 skip apps with type = "static"
- Per-app package.json checks

### Phase 3 (future)
- Per-app ESLint config overrides
- Per-app test runner detection
- Cross-app dependency validation (packages/ can't import from apps/)
- Shared package allowlist (like Rust R-DEPS-01)

## Open Questions
- Should static sites get ANY architecture checks? Or just code quality?
- Should packages/ have a TS equivalent of allowed_deps?
- How to handle apps that are both Rust and TS (e.g., a Rust backend with a TS frontend in the same directory)?
