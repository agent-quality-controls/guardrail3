Goal

Build a new minimal CLI app at `apps/guardrail3-rs` that runs the extracted Rust package families directly. The app must use the new `apparch` layout, must itself satisfy all migrated Rust families except `hexarch`, and must treat `hexarch` as legacy/out of the active runtime.

Approach

1. Create a new workspace at `apps/guardrail3-rs` with an apparch crate layout:
   - `types/`
   - `logic/`
   - `io/inbound/cli`
   - `io/outbound/packages`
   - `io/outbound/report`
2. Keep the first CLI surface minimal:
   - `guardrail3-rs validate --path <workspace-root> [--family <name> ...]`
   - default: run all supported Rust families except `hexarch`
3. Put all package orchestration in `logic/`:
   - family enum and selection
   - crawl once
   - one package-orchestrator function per family
   - result collection and exit-code decision
4. Put all package invocation details in `io/outbound/packages`:
   - one module per family that wires:
     - `g3rs-workspace-crawl`
     - family ingestion
     - family checks
   - return shared report facts to `logic/`
5. Put CLI parsing and stdout/stderr rendering in `io/inbound/cli`.
6. Put report rendering adapters in `io/outbound/report`.
7. Do not carry over old app machinery:
   - no old family mapper
   - no old family selection
   - no old project tree
   - no old app-global policy plumbing
8. Move `packages/rs/hexarch` out of the active package surface after the new app is wired, likely under a `legacy/` subtree or equivalent non-active location.

Key decisions

- Minimal command surface first:
  - only `validate`
  - no `init`, `generate`, `dump-tree`, coverage maps, or TS support
- One pointed workspace root per invocation.
- Family selection is explicit and simple:
  - `--family arch --family cargo`
  - or no flag to mean "all supported families"
- Package execution order is fixed inside `logic`:
  - `topology`
  - `toolchain`
  - `fmt`
  - `cargo`
  - `clippy`
  - `deny`
  - `code`
  - `arch`
  - `deps`
  - `garde`
  - `test`
  - `release`
  - `hooks`
  - `apparch`
- The app should fail fast on crawl/setup misuse, but family results should be aggregated once the crawl succeeds.
- Report model should be app-local and tiny:
  - family id
  - check results
  - final highest severity

Proposed crate layout

```text
apps/guardrail3-rs/
  Cargo.toml
  crates/
    types/
      cli-types/
      report-types/
    logic/
      validate-command/
      family-runner/
    io/
      inbound/
        cli/
      outbound/
        packages/
        report/
```

Minimal CLI contract

```text
guardrail3-rs validate --path <workspace-root> [--family <family> ...] [--inventory]
```

Arguments

- `--path <workspace-root>`
  - required
  - must point to one Rust workspace root
- `--family <family>`
  - repeatable
  - allowed values:
    - `apparch`
    - `arch`
    - `cargo`
    - `clippy`
    - `code`
    - `deps`
    - `deny`
    - `fmt`
    - `garde`
    - `hooks`
    - `release`
    - `test`
    - `toolchain`
    - `topology`
- `--inventory`
  - include inventory/info findings in output

Initial execution flow

```text
cli
  -> parse args
  -> logic validate command
  -> crawl workspace once
  -> for each selected family:
       outbound package adapter
         -> family ingestion
         -> family checks
  -> report adapter
  -> exit code from highest severity
```

Files to create later

- `apps/guardrail3-rs/Cargo.toml`
- `apps/guardrail3-rs/crates/types/cli-types/**`
- `apps/guardrail3-rs/crates/types/report-types/**`
- `apps/guardrail3-rs/crates/logic/validate-command/**`
- `apps/guardrail3-rs/crates/logic/family-runner/**`
- `apps/guardrail3-rs/crates/io/inbound/cli/**`
- `apps/guardrail3-rs/crates/io/outbound/packages/**`
- `apps/guardrail3-rs/crates/io/outbound/report/**`
