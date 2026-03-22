# Tool Inventory

Every tool guardrail3 orchestrates, how it discovers config, and how it should be invoked.

All claims verified by web research + source code reading (March 2026). See `edge-cases/` for detailed findings per tool.

## Rust Tools

| Tool | Config file | Discovery | Shadowing? |
|---|---|---|---|
| clippy | clippy.toml / .clippy.toml | Walk-up from CARGO_MANIFEST_DIR | YES — nearest wins |
| cargo-deny | deny.toml / .deny.toml / .cargo/deny.toml | Walk-up from manifest directory | YES — nearest wins |
| rustfmt | rustfmt.toml / .rustfmt.toml | Walk-up from source file (rustfmt) or workspace root (cargo fmt) | YES — nearest wins |
| cargo-machete | none | Workspace Cargo.toml | N/A |
| cargo test | none | Workspace Cargo.toml | N/A |
| gitleaks | .gitleaks.toml | Target path (no walk-up) | NO |

## TypeScript Tools

| Tool | Config file | Discovery | Shadowing? |
|---|---|---|---|
| ESLint (v10+) | eslint.config.{js,mjs,ts,...} | Walk-up from each linted file (DEFAULT since v10, Feb 2026) | YES — nearest wins |
| TypeScript (tsc) | tsconfig.json | Walk-up from CWD (when no -p or files given), explicit -p flag | YES — nearest wins |
| Stylelint | .stylelintrc.* / stylelint.config.* | Walk-up from each linted file (cosmiconfig) | YES — nearest wins |
| Prettier | .prettierrc.* / prettier.config.* | Walk-up from each file (cosmiconfig, goes to $HOME) | YES — nearest wins, NO extends |
| cspell | cspell.json / .cspell.json / cspell.config.* | Walk-up from each checked file | YES — nearest wins |
| jscpd | .jscpd.json | CWD only (cosmiconfig v9 default: no walk-up) | NO |
| pnpm audit | none | Root lockfile | N/A |

## Key Insight: Almost Everything Walks Up

Most tools walk up from the file/crate being processed and use the nearest config. This means:
- A config in a subdirectory SHADOWS the root config completely
- No merging happens (except stylelint `extends` which merges rules)
- Per-app/per-crate configs are a real enforcement concern — they can silently drop all guardrails

Only jscpd and gitleaks DON'T walk up. Everything else does.

## Three Invocation Patterns

1. **Per Rust workspace:** clippy, cargo-deny, rustfmt, cargo-machete, cargo test. Must `cd` into each workspace root. Hook needs to discover all workspace roots.

2. **Per TS app (for type checking):** TypeScript (tsc). Must run with `-p <tsconfig>` for each app. Hook needs to discover all tsconfig.json files.

3. **Project-wide from root:** ESLint, Stylelint, Prettier, cspell, jscpd, pnpm audit, gitleaks. Run once from root. BUT — ESLint v10 now walks up per file, so per-app configs ARE active even when running from root. Same for Stylelint, Prettier, cspell.

**Pattern 3 is deceptive.** Running from root doesn't mean root config applies to everything. For walk-up tools (ESLint v10, Stylelint, Prettier, cspell), per-app configs shadow the root for files in that subtree. The hook runs once from root but enforcement varies by file location.

## Individual Tool Plans

See one file per tool for detailed discovery rules, config resolution, and edge cases.
See `edge-cases/` for web-researched verification of each tool's behavior.
