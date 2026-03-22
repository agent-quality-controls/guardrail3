# Rewrite hex arch structural enforcement

**Date:** 2026-03-20 09:45
**Task:** Replace current weak hex arch checks with strict structural enforcement based on directory layout

## Goal
Every service app under `apps/` must follow the exact hex arch directory template. The structure itself IS the rule — no guardrail3.toml config needed to determine layers. Layer assignment is derived from position in the tree.

## Input Information

### Required structure
```
apps/{name}/crates/
├── adapters/
│   ├── inbound/
│   │   └── {crate}/Cargo.toml   (each subdir is a crate)
│   └── outbound/
│       └── {crate}/Cargo.toml
├── app/
│   └── {crate}/Cargo.toml
├── domain/
│   └── {crate}/Cargo.toml
└── ports/
    ├── inbound/
    │   └── {crate}/Cargo.toml
    └── outbound/
        └── {crate}/Cargo.toml
```

### Structural constraints
1. `crates/` must contain exactly: `adapters/`, `app/`, `domain/`, `ports/` — nothing else
2. `adapters/` must contain exactly: `inbound/`, `outbound/` — nothing else
3. `ports/` must contain exactly: `inbound/`, `outbound/` — nothing else
4. 6 container folders: `app/`, `domain/`, `ports/inbound/`, `ports/outbound/`, `adapters/inbound/`, `adapters/outbound/`
5. Every direct child directory of a container folder must be its own crate (has Cargo.toml)
6. Container folders can be empty (no crates yet) — that's valid (e.g., `ports/inbound/` with no crates means no inbound ports yet)

### What exists today
- R-ARCH-01: Only checks `domain` and `adapters` exist. Misses `ports` and `app`. Only looks at `crates/{layer}/Cargo.toml` or `src/{layer}/mod.rs`. Only runs for `profile = "service"` in guardrail3.toml.
- R-ARCH-02: Dependency flow. Uses `layer_from_path()` which is fragile string matching. Depends on guardrail3.toml config for layer assignment.
- R-ARCH-03: Library depends on service internals. Simple path check.
- R-ARCH-04: Unconfigured workspace members. Depends entirely on guardrail3.toml.

### Reference implementation: pipelin3r
```
apps/shedul3r/crates/adapters/inbound/rest/Cargo.toml
apps/shedul3r/crates/adapters/inbound/mcp/Cargo.toml
apps/shedul3r/crates/adapters/outbound/db/Cargo.toml
apps/shedul3r/crates/adapters/outbound/subprocess/Cargo.toml
apps/shedul3r/crates/app/commands/Cargo.toml
apps/shedul3r/crates/domain/types/Cargo.toml
apps/shedul3r/crates/ports/outbound/repo/Cargo.toml
```

### Our own app (guardrail3) — currently broken
Single crate at `apps/guardrail3/` with `src/` containing module directories (`src/app/`, `src/domain/`, `src/ports/`, `src/adapters/`). No `crates/` directory at all. This should produce multiple errors once the new checks are in place.

## Approach

### How service apps are detected
A directory under `apps/` is a service app if it has a `Cargo.toml`. No guardrail3.toml config needed — position in the tree (`apps/`) is the signal. The check walks `apps/*/` looking for `Cargo.toml`.

### New check: R-ARCH-01 — Hex arch structure (rewrite)
For each service app detected under `apps/`:

1. **`crates/` must exist** — Error if missing
2. **`crates/` contents must be exactly `{adapters, app, domain, ports}`** — Error for each missing dir, Error for each unexpected entry (file or directory)
3. **`crates/adapters/` contents must be exactly `{inbound, outbound}`** — Error for each missing dir, Error for each unexpected entry
4. **`crates/ports/` contents must be exactly `{inbound, outbound}`** — Error for each missing dir, Error for each unexpected entry
5. **Every direct child dir of a container folder must have `Cargo.toml`** — Error if a subdir exists without Cargo.toml (it's not a crate)
6. **`.gitkeep` is the only file allowed** at the root of any structural or container folder. Any other file → Error.
7. **Empty containers are valid** — a container with just `.gitkeep` or truly empty is fine (no crates yet)

### R-ARCH-02 — Dependency flow (update layer detection)
Layer is now derived purely from path position:
- `crates/domain/**` → Domain
- `crates/ports/**` → Ports
- `crates/app/**` → App
- `crates/adapters/**` → Adapters

No config lookup needed. The `layer_from_path` function and `resolve_layer` config fallback get replaced with position-based detection from the `crates/` root.

Dependency rules unchanged:
- Domain → can't import Ports, App, Adapters
- Ports → can't import App, Adapters
- App → can't import Adapters
- Adapters → no restrictions

### R-ARCH-03, R-ARCH-04 — Keep as-is
These checks (library-service boundary, unconfigured members) are orthogonal to the structural enforcement. They still work.

### What triggers the new checks
The structural check runs for every directory under `apps/` that contains a `Cargo.toml`. No guardrail3.toml config needed — it's auto-detected from the directory layout. If a project has no `apps/` directory, the structural checks don't run.

### Check IDs
- R-ARCH-01: Rewrite — all structural violations (missing crates/, wrong contents, etc.)
- R-ARCH-02: Update — layer detection from path position instead of config

## Key decisions
- **No guardrail3.toml dependency for structure checks:** Structure is enforced by convention, not configuration. Having `apps/{name}/Cargo.toml` is enough to trigger all structural checks.
- **Error severity for all structural violations:** Missing layers and unexpected entries are all Errors, not Warnings.
- **Empty containers are valid:** `ports/inbound/` with no subdirectories is fine — it means no inbound ports yet.
- **Cargo.toml required in every crate dir:** A subdirectory inside a container without Cargo.toml is an error — it's not a crate.

## Files to Modify
- `apps/guardrail3/src/app/rs/validate/hex_arch_checks.rs` — rewrite R-ARCH-01, update R-ARCH-02 layer detection
- `apps/guardrail3/src/app/rs/validate/mod.rs` — pass filesystem root to new check (may need `apps/` dir list from crawler or discovery)
