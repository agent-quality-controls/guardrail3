# Rust Family Plan Surface

This directory is the current Rust family planning index.

It is also the per-family agent handoff surface for Rust-family testing,
scope-audit, and refactor work.

Use it first when you need to know:

- which Rust families are current
- where each family is implemented
- whether a family README exists
- which older plan files are still useful only as history or tactical execution notes
- what scope contract each family owns
- what an agent should test or refactor next for that family

Shared Rust authority:

1. [apps/guardrail3/crates/app/rs/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/README.md)
2. [.plans/todo/checks/2026-03-21-153251-checker-architecture.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md)
3. family-local READMEs
4. this directory for family-level planning/status reconciliation

Rust validation families covered here:

- `RS-TOPOLOGY`
- `RS-CARGO`
- `RS-CLIPPY`
- `RS-CODE`
- `RS-DENY`
- `RS-DEPS`
- `RS-FMT`
- `RS-GARDE`
- `RS-HEXARCH`
- `RS-LIBARCH`
- `RS-RELEASE`
- `RS-TEST`
- `RS-TOOLCHAIN`

Scope note:

- hook families still live primarily under:
  - `.plans/todo/checks/hooks/shared.md`
  - `.plans/todo/checks/hooks/rs.md`
- they are Rust-adjacent, but they are not folded into this first Rust-family cutover

Shared current audit frontier:

- whole-project target resolution and full-tree walking are now on the live path
- subtree correctness is no longer mainly a walker problem; it is now mostly a
  family-routing and family-consumption problem
- shared baseline now fixed:
  - git-worktree `.git` file handling is covered in `project_walker` tests
  - `guardrail3-app-rs-runtime` subtree/runtime tests compile and pass again
- family agents can now be launched on top of a stable shared baseline

Shared agent rules for this directory:

1. Read this file, then the family file, then the family README.
2. Audit the production path first:
   - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
   - the family `check(...)` entrypoint
   - family `facts.rs` / `discover.rs`
3. Do not treat test helpers as proof of production routing.
4. For every family, distinguish between:
   - global
   - workspace-local
   - workspace-local under an architecture zone
5. Done means:
   - the family scope contract is explicit
   - production code matches that contract
   - subtree or repo-global behavior is covered by tests
   - obvious bleed-through or over-narrowing bugs are fixed

Status snapshot / dispatch map:

| Family | Scope model | Current code root | Family README | Planning status |
|---|---|---|---|---|
| `topology` | global | `apps/guardrail3/crates/app/rs/families/topology/` | yes | current |
| `cargo` | workspace-local | `apps/guardrail3/crates/app/rs/families/cargo/` | yes | current |
| `clippy` | workspace-local | `apps/guardrail3/crates/app/rs/families/clippy/` | yes | current |
| `code` | global | `apps/guardrail3/crates/app/rs/families/code/` | yes | current |
| `deny` | workspace-local | `apps/guardrail3/crates/app/rs/families/deny/` | yes | current |
| `deps` | workspace-local | `apps/guardrail3/crates/app/rs/families/deps/` | yes | current |
| `fmt` | global | `apps/guardrail3/crates/app/rs/families/fmt/` | yes | current |
| `garde` | workspace-local | `apps/guardrail3/crates/app/rs/families/garde/` | yes | current |
| `hexarch` | workspace-local under `apps/*` | `apps/guardrail3/crates/app/rs/families/hexarch/` | yes | current |
| `libarch` | workspace-local under `packages/*` | `apps/guardrail3/crates/app/rs/families/libarch/` | yes | current |
| `release` | workspace-local | `apps/guardrail3/crates/app/rs/families/release/` | yes | current |
| `test` | global | `apps/guardrail3/crates/app/rs/families/test/` | yes | current |
| `toolchain` | workspace-local | `apps/guardrail3/crates/app/rs/families/toolchain/` | yes | current |

The old files under `.plans/todo/checks/rs/*.md` are now superseded as primary family plans.
Keep using them as detailed rule ledgers and migration history unless a family file here says otherwise.
