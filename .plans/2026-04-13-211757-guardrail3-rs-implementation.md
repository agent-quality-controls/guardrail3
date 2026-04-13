Goal

Implement `apps/guardrail3-rs` as the new minimal Rust-only CLI app that runs the extracted package families directly. The app should support only `validate` for now, target one pointed Rust workspace root per invocation, and default to all supported non-hexarch families.

Approach

1. Scaffold the new app workspace locally:
   - root `Cargo.toml`
   - shared apparch crate layout
   - minimal crate manifests and READMEs
2. Define tiny shared app types:
   - CLI request types
   - family enum
   - report item/result types
3. Implement the shared execution path locally:
   - CLI parse and dispatch
   - family selection
   - workspace crawl once
   - report rendering
   - exit code from highest severity
4. Delegate one family adapter per worker in waves, one family per task:
   - each adapter wires family ingestion + checks into the shared report shape
   - families:
     - topology
     - toolchain
     - fmt
     - cargo
     - clippy
     - deny
     - code
     - arch
     - deps
     - garde
     - test
     - release
     - hooks
     - apparch
5. Integrate worker results into `io/outbound/packages` and `logic/family-runner`.
6. Add app-level tests for:
   - family parsing
   - default family selection
   - per-family dispatch
   - `--inventory`
   - highest-severity exit code
7. Verify the whole app workspace and write a standalone worklog.

Key decisions

- Keep `hexarch` out of the active app surface.
- One family adapter module per family, even if some share wiring shape.
- No backward-compatibility bridge to the old `apps/guardrail3`.
- Use package crates directly; do not wrap them in old family runtime crates.
- Minimal human-readable reporter first, machine formats later if needed.

Files to modify

- `apps/guardrail3-rs/**`
- possibly top-level workspace registration if needed
- later: `packages/rs/hexarch/**` move to legacy after app wiring is complete
