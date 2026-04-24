# Rust Rule Native-Tool Audit

## Goal

- audit every implemented Rust runtime rule under `packages/rs`
- decide, per rule, whether the rule should stay in guardrail3, move to a native Rust tool, or move to a custom lint/plugin surface
- rewrite `.plans/2026-04-23-163711-rust-tool-migration-audit.md` so it contains only concrete rule-level findings

## Approach

1. inventory runtime rule files only
   - exclude assertion mirrors
   - map rule counts by family
2. read implemented rule bodies by family
   - `fmt`
   - `toolchain`
   - `clippy`
   - `deny`
   - `cargo`
   - `code`
   - `garde`
   - `test`
   - `deps`
   - `release`
   - `hooks`
   - `topology`
3. classify each rule against the real external owner
   - rustfmt
   - Cargo
   - Clippy config
   - cargo-deny
   - release-plz
   - cargo-semver-checks
   - cargo publish
   - cargo-nextest
   - cargo-mutants
   - custom lint surface such as rustc lint crates or Dylint
4. keep only rule-level findings that change ownership or narrow scope
   - each finding must state the current rule id
   - what the rule does today
   - exact target owner
   - exact replacement rule or config surface
   - exact edit to make
   - why the change does not weaken enforcement

## Key Decisions

- audit implemented runtime rules, not plan-only rule ids
- treat "plugin-capable" separately from "already owned by a native tool"
- do not write family summaries unless they produce at least one concrete rule-level action

## Files To Modify

- `.plans/2026-04-23-163711-rust-tool-migration-audit.md`
- `.plans/2026-04-23-180619-rust-rule-native-tool-audit.md`
