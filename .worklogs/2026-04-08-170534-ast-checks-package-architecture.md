# AST Checks Package Architecture

**Date:** 2026-04-08 17:05
**Scope:** `.plans/todo/checks/2026-04-08-ast-checks-package-architecture.md`

## Summary
Added a dedicated AST package architecture note for the new `g3rs` pipeline. This note separates AST ingestion responsibilities from AST checks-runtime responsibilities and locks the intended package boundary before building the first real AST checks package.

## Context & Problem
The new general `g3rs` architecture note established the overall crawl -> ingestion -> checks -> rules pipeline, but AST needed its own explicit contract. The repo already had AST-heavy behavior in old app families and one extracted garde AST package, but the current implementation details could not be treated as final architecture.

The user wanted a fresh design note that defines how AST packages should work now, not a description of what any transitional implementation happens to do.

## Decisions Made

### Make AST ingestion own file reading and scope selection
- **Chose:** AST ingestion selects the correct bounded scope and reads source files into the public AST input.
- **Why:** This keeps file access and workspace discovery in ingestion instead of letting AST checks packages reach back into the filesystem.
- **Alternatives considered:**
  - Let AST checks packages read files from path lists — rejected as a transitional convenience, not the clean target boundary.
  - Push AST parsing into ingestion too — rejected because checks runtimes still need family-local mapping and fan-out inside their own bounded input.

### Make AST checks runtime own parse-once and semantic mapping
- **Chose:** AST checks runtime parses source contents once and performs heavy intra-scope mapping there.
- **Why:** This is the correct place for support-layer AST logic without pushing it down into rules or up into ingestion.
- **Alternatives considered:**
  - Put more semantic mapping into ingestion — rejected because it would make ingestion own too much family-specific AST semantics.
  - Let rules parse and inspect AST directly — rejected because rules must stay tiny and pure.

### Make AST scope explicitly variable
- **Chose:** Document allowed AST scopes as file, crate, root, or package, with the smallest bounded scope chosen per family.
- **Why:** AST is not universally one-file, and the architecture needs to say that plainly.
- **Alternatives considered:**
  - Treat AST as one-file-only — rejected because it does not fit bounded cross-file families.
  - Treat AST as repo-global by default — rejected because it is too broad and breaks bounded family inputs.

### Use source contents as the public AST input surface
- **Chose:** Public AST inputs should carry source text, not absolute file paths.
- **Why:** It keeps the package contract explicit, bounded, and free of implicit filesystem reads.
- **Alternatives considered:**
  - Public input as path + later runtime read — rejected as looser than the target boundary.

### Allow a small runtime-local parsed container
- **Chose:** Document a small runtime-local parsed shape for the first `code` specimen:
  - `G3RsCodeSourceFileAst { source_file, ast }`
- **Why:** It keeps parsed AST attached to the exact source input while staying internal to the runtime/support layer.
- **Alternatives considered:**
  - No runtime-local parsed container at all — rejected because runtime code becomes noisier once parse and support helpers are split.
  - Make the parsed container the public package input — rejected because the public contract should stay source-text based.

## Architectural Notes
The note defines two AST package shapes:

- single-file AST package
- scoped multi-file AST package

It also sets implementation order:

1. `code` as the first clean single-file AST specimen
2. `garde` as the first clean bounded multi-file AST specimen

This keeps the first implementation small and avoids starting AST packaging with cross-file complexity.

## Information Sources
- `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md`
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `.plans/todo/checks/rs/code.md`
- `.plans/todo/checks/rs/garde.md`
- `packages/rs/garde/g3rs-garde-ast-checks/**`
- `packages/rs/cargo/g3rs-cargo-config-checks/**`

## Open Questions / Future Considerations
- The first actual AST package build still needs a concrete per-family plan for `code`.
- `test` may eventually want more than one AST package or more than one AST scope.
- `hexarch` likely needs crate-scoped module-walk AST inputs rather than simple file bundles.

## Key Files for Context
- `.plans/todo/checks/2026-04-08-ast-checks-package-architecture.md` — the AST package template
- `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md` — the broader `g3rs` architecture note
- `.plans/todo/checks/rs/code.md` — current single-file AST family specimen
- `.plans/todo/checks/rs/garde.md` — current multi-file AST family ledger
- `packages/rs/garde/g3rs-garde-ast-checks/src/lib.rs` — current extracted AST package surface

## Next Steps / Continuation Plan
1. Commit this architecture note so package work can follow a fixed AST contract.
2. Create a concrete plan for the first AST package only: `g3rs-code-ast-checks`.
3. Scaffold `packages/rs/code/g3rs-code-ast-checks`.
4. Start with a small set of clearly single-file AST rules and prove parity against the old code-family rule tests.
