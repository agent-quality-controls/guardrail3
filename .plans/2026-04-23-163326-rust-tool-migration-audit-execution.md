Goal
- Produce a family-by-family Rust tool-migration audit grounded in the repo's current rule inventory and current official Rust tooling capabilities.

Approach
- Read the active Rust family ledgers, hook ledgers, and recent worklogs to recover the intended and implemented rule surfaces.
- Inventory the concrete runtime rule files under `packages/rs` so the audit is based on actual checks, not only planning docs.
- Verify native-tool capability from primary sources for rustfmt, Clippy, cargo-deny, Cargo, cargo metadata, and other relevant official Rust tooling.
- Classify each family as `Stay`, `Move`, or `Split`, with exact target tool ownership and exact residual `guardrail3` ownership.
- Write the audit result as a markdown handoff artifact in `.plans/`.

Key decisions
- Treat `hooks` as part of scope even though the original Rust family list separates shared hooks from core Rust families.
- Distinguish tool-config presence/wiring checks from semantic findings owned by the tool itself.
- Prefer `Split` when native tooling can own semantic checks but `guardrail3` still has to own repo policy, topology, or enforcement wiring.
- Use only primary sources for external-tool capability claims.

Files to modify
- `.plans/2026-04-23-163326-rust-tool-migration-audit-execution.md`
- `.plans/2026-04-23-<timestamp>-rust-tool-migration-audit.md`
