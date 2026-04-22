Summary
- Cleaned up `rs/arch` ingestion after the facade-pairing repair by reducing the top-level import fan-in in `source.rs`.
- This removes the live `RS-CODE-SOURCE-11` warning on the package without changing behavior.

Decisions made
- Replaced the wide `g3rs_arch_types::types::{...}` import list with a module alias.
  - Why: the package warning was triggered by the top-level import fan-in, not by the runtime logic.
  - Rejected: splitting the file or introducing helper modules, because this was a one-file warning cleanup and the code path was already correct.

Key files for context
- [packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/source.rs)
- [packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_11_many_use_imports/rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_11_many_use_imports/rule.rs)

Next steps
- Continue the Rust boundary audit from confirmed defects only.
- The next review target is still `rs/code` source, but only to prove or rule out any remaining production-path boundary defect before changing it.
