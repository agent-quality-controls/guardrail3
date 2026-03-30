# Rust Family Plan Surface

This directory is the current Rust family planning index.

Use it first when you need to know:

- which Rust families are current
- where each family is implemented
- whether a family README exists
- which older plan files are still useful only as history or tactical execution notes

Shared Rust authority:

1. [apps/guardrail3/crates/app/rs/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/README.md)
2. [.plans/todo/checks/2026-03-21-153251-checker-architecture.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md)
3. family-local READMEs
4. this directory for family-level planning/status reconciliation

Rust validation families covered here:

- `RS-ARCH`
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

Status snapshot:

| Family | Current code root | Family README | Planning status |
|---|---|---|---|
| `arch` | `apps/guardrail3/crates/app/rs/families/arch/` | yes | current |
| `cargo` | `apps/guardrail3/crates/app/rs/families/cargo/` | yes | current |
| `clippy` | `apps/guardrail3/crates/app/rs/families/clippy/` | yes | current |
| `code` | `apps/guardrail3/crates/app/rs/families/code/` | yes | current |
| `deny` | `apps/guardrail3/crates/app/rs/families/deny/` | yes | current |
| `deps` | `apps/guardrail3/crates/app/rs/families/deps/` | yes | current |
| `fmt` | `apps/guardrail3/crates/app/rs/families/fmt/` | yes | current |
| `garde` | `apps/guardrail3/crates/app/rs/families/garde/` | yes | current |
| `hexarch` | `apps/guardrail3/crates/app/rs/families/hexarch/` | yes | current |
| `libarch` | `apps/guardrail3/crates/app/rs/families/libarch/` | yes | current |
| `release` | `apps/guardrail3/crates/app/rs/families/release/` | yes | current |
| `test` | `apps/guardrail3/crates/app/rs/families/test/` | yes | current |
| `toolchain` | `apps/guardrail3/crates/app/rs/families/toolchain/` | yes | current |

The old files under `.plans/todo/checks/rs/*.md` are now superseded as primary family plans.
Keep using them as detailed rule ledgers and migration history unless a family file here says otherwise.
