# Tool Config Resolution Edge Cases — Verified Research

**Date:** 2026-03-18
**Purpose:** Document actual config resolution behavior for 11 tools used in monorepo projects. All answers verified via web search against official docs, GitHub issues, and release notes.

---

## 1. cargo clippy + clippy.toml

### Q1: Root AND subfolder config?
**First match wins, no merging.** Clippy walks up from the crate being checked and uses the first `clippy.toml` or `.clippy.toml` it finds. If a crate at `crates/adapters/outbound/` has its own `clippy.toml`, that one is used and the root one is ignored. There is no merging — the closest config shadows everything above it.

- Source: [GitHub Issue #7353](https://github.com/rust-lang/rust-clippy/issues/7353) — "the first `.clippy.toml` found in one of the parent directories of the package being checked being used for all settings."

### Q2: Config in intermediate directory (e.g. `crates/adapters/clippy.toml`)?
**Yes, it gets picked up.** Clippy walks up the directory tree from the crate's `Cargo.toml` location. If `crates/adapters/clippy.toml` exists and there's no `clippy.toml` in `crates/adapters/outbound/`, the intermediate one will be found first during the walk-up and used. If TWO `clippy.toml` files exist in the walk-up path (e.g. at `crates/adapters/` and at root), the closest one wins — the intermediate one shadows the root one.

### Q3: No config file at all?
**Clippy uses built-in defaults.** No error is produced.

### Q4: Run from a different directory?
Clippy resolves config relative to `CARGO_MANIFEST_DIR` (the directory containing the crate's `Cargo.toml`), not CWD. So running from a different directory doesn't matter — it finds config based on the crate location.

### Q5: `--config` or equivalent?
**No `--config` flag.** But you can set `CLIPPY_CONF_DIR` environment variable to point to a specific directory. Resolution order:
1. `CLIPPY_CONF_DIR` env var
2. `CARGO_MANIFEST_DIR` env var (then walks up)
3. Current directory

- Source: [Clippy Configuration Docs](https://doc.rust-lang.org/clippy/configuration.html)

### Q6: Workspace monorepo behavior?
Each crate in the workspace gets its own config resolution (walk-up from its own `Cargo.toml`). There is no workspace-aware merging. A workspace root `clippy.toml` applies to all crates that don't have a closer one. This is a known pain point — [Issue #7353](https://github.com/rust-lang/rust-clippy/issues/7353) requests the ability to have workspace-root defaults + per-crate overrides (not yet implemented as of 2026).

---

## 2. cargo-deny + deny.toml

### Q1: Root AND subfolder config?
**Walk-up from Cargo.toml, first match wins.** The config is "recursively searched for in parent directories starting in the same directory as the Cargo.toml." First `deny.toml` found wins.

- Source: [cargo-deny CLI docs](https://embarkstudios.github.io/cargo-deny/cli/common.html)

### Q2: Config in intermediate directory?
**Yes, it would be found** during the walk-up if there's no `deny.toml` closer to the `Cargo.toml`. The walk starts at the manifest directory and goes up.

### Q3: No config file at all?
**cargo-deny will error.** It requires a config file to run checks. You can create one with `cargo deny init`.

### Q4: Run without `--manifest-path`?
Without `--manifest-path`, cargo-deny uses the `Cargo.toml` in CWD. If CWD has no `Cargo.toml`, it will error (same as any cargo subcommand). The `deny.toml` search starts from the manifest's directory.

### Q5: `--config` flag?
**Yes.** `--config <path>` specifies the config file explicitly, bypassing the walk-up search. There is also a `--manifest-path` flag to control which `Cargo.toml` is used as the starting point.

### Q6: Can deny.toml be in `.cargo/deny.toml`?
**Not for the main config.** However, cargo-deny does support project-local **exceptions files** in `.cargo/deny.exceptions.toml` (alongside `deny.exceptions.toml` and `.deny.exceptions.toml` in the manifest directory).

- Source: [cargo-deny config docs](https://embarkstudios.github.io/cargo-deny/checks/cfg.html)

### Workspace behavior?
In a workspace, you typically have one `deny.toml` at the workspace root. cargo-deny operates on the entire dependency graph of the workspace (or subset via `--manifest-path`). Per-crate deny configs are unusual.

---

## 3. rustfmt + rustfmt.toml

### Q1: Root AND subfolder config?
**First match wins during walk-up, no merging.** rustfmt walks up from the file being formatted and uses the first `rustfmt.toml` or `.rustfmt.toml` it finds.

- Source: [rustfmt Configurations.md](https://github.com/rust-lang/rustfmt/blob/main/Configurations.md) — "place it in the project or any other parent directory"

### Q2: Config in intermediate directory (e.g. `crates/rustfmt.toml`)?
**It depends on whether you use `cargo fmt` or `rustfmt` directly.**

- **`rustfmt` directly:** Walks up from the file's directory. An intermediate `crates/rustfmt.toml` WILL be found for files under `crates/`.
- **`cargo fmt`:** Invokes rustfmt with *entry point files* (e.g. `src/lib.rs`). Config is resolved for the entry point's directory, NOT for individual source files. A `rustfmt.toml` in a subdirectory like `src/foo/` is **ignored** by `cargo fmt` because it resolves config from the entry point, not from each file.

- Source: [GitHub Issue #5814](https://github.com/rust-lang/rustfmt/issues/5814) — "cargo fmt invokes rustfmt with the entry point files for all the package targets in the workspace"

### Q3: No config file at all?
**rustfmt uses built-in defaults.** Additionally checks `~/.config/rustfmt/rustfmt.toml` as a global fallback. No error.

### Q4: Run from a different directory?
**`cargo fmt`:** Resolves config from the crate's entry point directory. CWD doesn't matter for config resolution.
**`rustfmt` directly:** Resolves config from each input file's directory. CWD doesn't matter for config resolution.

### Q5: `--config-path` flag?
**Yes.** rustfmt supports `--config-path <path>` to specify a config file or directory. When a directory is given, it searches that directory for `rustfmt.toml`. This bypasses the normal walk-up.

### Q6: `cargo fmt` vs `rustfmt` — edition handling?
**Critical difference:** `cargo fmt` reads the edition from `Cargo.toml` and passes it to rustfmt. Running `rustfmt` directly defaults to edition 2015 unless configured in `rustfmt.toml`. This can cause different formatting results.

- Source: [rustfmt docs](https://rust-lang.github.io/rustfmt/)

---

## 4. ESLint (flat config) + eslint.config.mjs

### Q1: Root AND subfolder config?
**As of ESLint v10 (Feb 2026): subdirectory config SHADOWS root config entirely.** ESLint now starts from the directory of the file being linted and walks up until it finds an `eslint.config.*` file. The first one found is used — no merging with parent configs.

If `apps/landing/eslint.config.mjs` exists AND root `eslint.config.mjs` exists, running `eslint apps/landing/src/file.ts` from root will use `apps/landing/eslint.config.mjs` only. The root config is NOT consulted.

- Source: [ESLint v10.0.0 Release](https://eslint.org/blog/2026/02/eslint-v10.0.0-released/) — "ESLint v10.0.0 locates eslint.config.* by starting from the directory of each linted file rather than the current working directory"

### Q2: `config_lookup_from_file` — now stable?
**Yes, it is the default in v10.** The `v10_config_lookup_from_file` feature flag from v9 has been removed. This is now the only behavior.

- Source: [ESLint Configuration Files docs](https://eslint.org/docs/latest/use/configure/configuration-files)

### Q3: `eslint --config <path>` flag?
**Yes.** Using `--config` or `-c` prevents the automatic directory search entirely. ESLint uses only the specified config file.

- Source: [ESLint docs](https://eslint.org/docs/latest/use/configure/configuration-files) — "You can prevent this search by using the -c or --config option"

### Q4: Does subdirectory config get picked up WITHOUT the experimental flag?
**In v9:** No, subdirectory configs were NOT picked up unless you enabled `v10_config_lookup_from_file`.
**In v10:** Yes, this is the default behavior now.

### Q5: Config file priority order?
When multiple config formats exist in the same directory:
1. `eslint.config.js`
2. `eslint.config.mjs`
3. `eslint.config.cjs`
4. `eslint.config.ts`
5. `eslint.config.mts`
6. `eslint.config.cts`

### Q6: Monorepo behavior?
v10's per-file lookup is explicitly designed for monorepos. Each workspace package can have its own `eslint.config.mjs`. If a package doesn't have one, the walk-up continues to the root config. Subdirectory configs must be self-contained (they can import shared configs manually, but there's no automatic merging).

- Source: [ESLint Discussion #16960](https://github.com/eslint/eslint/discussions/16960)

---

## 5. TypeScript (tsc) + tsconfig.json

### Q1: `tsc` without `-p` in a directory with no tsconfig.json?
**Walks up the directory tree.** When invoked without input files, tsc searches for `tsconfig.json` starting in CWD and continuing up the parent directory chain. The first `tsconfig.json` found is used.

**Important:** When input files ARE specified on the command line (e.g. `tsc foo.ts`), tsconfig.json files are completely ignored.

- Source: [TypeScript Handbook — tsconfig.json](https://www.typescriptlang.org/docs/handbook/tsconfig-json.html)

### Q2: `extends` — can it extend from node_modules?
**Yes.** TypeScript 3.2+ resolves `extends` paths from `node_modules`. You can use bare specifiers:
```json
{ "extends": "@tsconfig/recommended/tsconfig.json" }
```
TypeScript will check the package's `package.json` for a `"tsconfig"` field first, then fall back to `tsconfig.json` at the package root.

**TypeScript 5.0+** supports extending from multiple configs via array syntax:
```json
{ "extends": ["@tsconfig/node20/tsconfig.json", "./tsconfig.base.json"] }
```

Can also extend from relative paths outside the project (e.g. `"extends": "../../shared/tsconfig.base.json"`).

- Source: [TSConfig extends docs](https://www.typescriptlang.org/tsconfig/extends.html), [tsconfig/bases](https://github.com/tsconfig/bases)

### Q3: Composite projects and project references?
- `"composite": true` enables a project to be referenced by other projects. It forces `declaration: true` and `incremental: true`.
- `"references"` array lists dependent projects: `"references": [{ "path": "../core" }]`
- **`references` is NOT inherited via `extends`.** Each project must declare its own references.
- **`paths` defined in `extends` base are completely overwritten** (not merged) if the child also defines `paths`.
- Use `tsc --build` (or `tsc -b`) to build composite projects. It builds referenced projects in dependency order.

- Source: [TypeScript Project References](https://www.typescriptlang.org/docs/handbook/project-references.html)

### Q4: No tsconfig.json at all?
**tsc uses default compiler options** and treats CWD as the root. All `.ts` files in CWD and subdirectories are included.

### Q5: Workspace monorepo behavior?
TypeScript has no workspace awareness. Project references must be manually configured. Common pattern: root `tsconfig.json` with only `references`, per-package `tsconfig.json` files that `extend` a shared base.

---

## 6. Stylelint + .stylelintrc.*

### Q1: Root AND subfolder config?
**First match wins during walk-up, no merging.** Stylelint uses cosmiconfig which walks up from the file being linted. The first config found stops the search.

- Source: [Stylelint Configure docs](https://stylelint.io/user-guide/configure/)

### Q2: Running `stylelint "apps/**/*.css"` from root — which config applies?
**The config closest to each file.** If `apps/landing/.stylelintrc.mjs` exists, files under `apps/landing/` use that config. Files under `apps/other/` (with no local config) continue walking up to the root config. Each file gets its own config resolution.

### Q3: Does `extends` merge or replace?
**Merging with override.** When using `extends`, rules are explicitly merged — not replaced wholesale. When extending an array of configs, "each item in the array takes precedence over the previous item" (later items override earlier ones on a per-rule basis).

The `overrides` property allows applying different rules to file subsets via glob patterns. Multiple overrides are applied in order — the last override has highest precedence.

- Source: [Stylelint Configure docs](https://stylelint.io/user-guide/configure/)

### Q4: No config file?
**Stylelint will error** — it requires a configuration to run.

### Q5: `--config` flag?
**Yes.** `--config <path>` or `configFile` option bypasses cosmiconfig search entirely.

### Q6: Monorepo/workspace behavior?
No workspace awareness. Each file's config is resolved independently via cosmiconfig walk-up. Common pattern: shared config package + per-app `.stylelintrc.mjs` that extends it.

### Cosmiconfig search strategy note:
Cosmiconfig v9 introduced search strategies. The `"none"` strategy stops directory traversal (only checks the given directory). Stylelint's default behavior still walks up.

- Source: [Stylelint Issue #7224](https://github.com/stylelint/stylelint/issues/7224)

---

## 7. Prettier + .prettierrc.*

### Q1: Root AND subfolder config?
**Subdirectory config SHADOWS root config entirely, no merging.** Prettier uses cosmiconfig to walk up from the file being formatted. The first `.prettierrc` found is used — there is no merging with parent configs.

- Source: [Prettier Configuration docs](https://prettier.io/docs/configuration) — "The configuration file will be resolved starting from the location of the file being formatted, and searching up the file tree until a config file is (or isn't) found."

### Q2: Intermediate directory config?
**Yes, it gets picked up.** If `.prettierrc` exists at `apps/` (intermediate), files under `apps/landing/` (with no local config) will find and use the intermediate one.

### Q3: No config file?
**Prettier uses built-in defaults.** No error.

### Q4: Run from a different directory?
**CWD doesn't matter.** Config is resolved starting from the location of the file being formatted, not CWD.

### Q5: `--config` flag?
**Yes.** `--config <path>` specifies the config file explicitly, overriding the cosmiconfig search. Also available: `--no-config` to disable config file loading entirely, and `--find-config-path <file>` to show which config would be used for a given file.

### Q6: Monorepo/workspace behavior?
No workspace awareness. Each file gets independent config resolution. Common pattern: root `.prettierrc` for the whole repo, with optional per-package overrides that shadow the root.

**Important caveat:** `.prettierignore` does NOT follow the same walk-up pattern — only the `.prettierignore` at the project root is used, regardless of where the file being formatted lives. This is a known inconsistency.

- Source: [Prettier GitHub Issue #12923](https://github.com/prettier/prettier/issues/12923)

---

## 8. cspell + cspell.json

### Q1: Root AND subfolder config?
**First match wins during walk-up, no merging.** CSpell searches from the file's directory upward. The first config file found is used.

- Source: [CSpell Configuration docs](https://cspell.org/docs/Configuration)

### Q2: Does `import` auto-resolve?
**No, imports are explicit.** The `import` field in a cspell config lets you reference other config files. Imported files are merged in order: "each configuration file can `import` more configuration files. The files listed in the import are merged from first to last with the parent (the one that did the import) merged at the end." The parent config's settings take precedence over imported ones.

You can import from `node_modules` packages (e.g. `"import": ["@cspell/dict-typescript/cspell-ext.json"]`).

- Source: [CSpell Imports docs](https://cspell.org/docs/Configuration/imports)

### Q3: No config file?
**CSpell uses built-in defaults** (English dictionary). No error.

### Q4: `--config` flag?
**Yes.** `--config <path>` specifies the config file, bypassing the walk-up search.

Additional useful flags:
- `--no-config-search` — disables automatic walk-up search; only uses the explicitly specified config (if any)
- `--stop-config-search-at <dir>` — stops the walk-up at a specific directory

### Q5: Monorepo/workspace behavior?
No workspace awareness. CSpell's walk-up is purely filesystem-based. Common pattern: root `cspell.json` with project-wide dictionaries, per-package configs that `import` the root config.

### Config file formats supported:
`.cspell.json`, `cspell.json`, `.cspell.config.yaml`, `cspell.config.js`, `cspell.config.ts`, `cspell.config.toml`, `.vscode/cspell.json`, and `package.json` (`cspell` field). Files can be prefixed with `.` and/or `.config`.

- Source: [CSpell Configuration docs](https://cspell.org/docs/Configuration)

---

## 9. jscpd + .jscpd.json

### Q1: Per-directory configs?
**No.** jscpd does NOT walk up directories or support per-directory config files. It looks for `.jscpd.json` in the project root (the path you pass to jscpd) or uses the config specified via CLI flag. There is no cosmiconfig-style discovery.

- Source: [jscpd npm docs](https://www.npmjs.com/package/jscpd), [jscpd GitHub](https://github.com/kucherenko/jscpd)

### Q2: Config in intermediate directory?
**Ignored.** jscpd only checks the root/specified path for `.jscpd.json`.

### Q3: No config file?
**jscpd uses built-in defaults.** No error.

### Q4: `--config` flag?
**Yes.** `-c` or `--config <path>` specifies the config file path. Default is `.jscpd.json` in the provided path.

### Q5: Config formats?
- `.jscpd.json` file
- `jscpd` section in `package.json`

### Q6: Monorepo/workspace behavior?
No workspace awareness. jscpd operates on the paths you give it. For monorepo use, you'd either run it once from root with a single `.jscpd.json`, or run it per-package with `--config` pointing to each package's config.

---

## 10. pnpm audit

### Q1: Does it check all workspace packages or only root?
**All workspace packages.** `pnpm audit` checks the entire lockfile which includes all workspace packages' dependencies. It does not audit packages individually — it audits the combined dependency graph.

- Source: [pnpm audit docs](https://pnpm.io/cli/audit/)

### Q2: `pnpm audit --filter <package>`?
**`--filter` is NOT supported with `pnpm audit`.** The `pnpm audit -r` (recursive) flag is also not supported. Audit operates on the entire lockfile. You can filter by dependency type though:
- `--dev` / `-D` — only dev dependencies
- `--prod` / `-P` — only production dependencies
- `--no-optional` — exclude optional dependencies

### Q3: `--audit-level`?
**Yes.** `--audit-level <severity>` filters by severity (low, moderate, high, critical). Default is `low`.

### Q4: Configuration?
Audit behavior can be configured in `pnpm-workspace.yaml` via `auditConfig`:
```yaml
auditConfig:
  ignoreCves:
    - CVE-2024-xxxxx
  ignoreGhsas:
    - GHSA-xxxxx
  auditLevel: moderate
```

### Q5: `--fix`?
**Yes.** `--fix` adds overrides to `package.json` for non-vulnerable versions. Also: `--ignore-unfixable` to skip unfixable CVEs.

### Q6: Monorepo behavior?
pnpm audit is inherently workspace-aware because it operates on the shared lockfile (`pnpm-lock.yaml`). All workspace packages' dependencies are included in the audit. You cannot scope the audit to a single workspace package.

---

## 11. gitleaks + .gitleaks.toml

### Q1: Does `.gitleaks.toml` walk up?
**No.** Gitleaks does NOT walk up directories. It checks for `.gitleaks.toml` only in the **target/source directory** (the repo root when scanning a git repo, or the `--source` path).

Config resolution order:
1. `--config` / `-c` flag (explicit path)
2. `GITLEAKS_CONFIG` environment variable (path to config file)
3. `GITLEAKS_CONFIG_TOML` environment variable (inline TOML content)
4. `<source-directory>/.gitleaks.toml` (repo root)
5. Built-in default config (embedded in binary)

- Source: [gitleaks GitHub Issue #1557](https://github.com/gitleaks/gitleaks/issues/1557), [gitleaks config](https://github.com/gitleaks/gitleaks/blob/master/config/gitleaks.toml)

### Q2: `--config` flag?
**Yes.** `--config <path>` or `-c <path>` specifies the config file. Must be a local file path — remote URLs are NOT supported.

### Q3: No config file?
**Gitleaks uses its built-in default ruleset** (compiled into the binary). This includes ~90 rules for common secret patterns. No error.

### Q4: Intermediate directory config?
**Ignored.** Only the source/target root is checked for `.gitleaks.toml`.

### Q5: Run from a different directory?
CWD is irrelevant. Gitleaks uses the `--source` path (defaults to `.`) to determine where to look for `.gitleaks.toml`.

### Q6: Monorepo behavior?
No monorepo awareness. Single `.gitleaks.toml` at the repo root applies to the entire repo. For different rules in different parts of a monorepo, you'd need to run gitleaks multiple times with different `--config` and `--source` flags.

### Proposed future feature:
A proposed `~/.config/gitleaks/config.toml` user-level config is under discussion but not yet implemented as of 2026.

---

## Summary Table: Config Resolution Patterns

| Tool | Walk-up? | Merging? | `--config` flag? | Workspace-aware? |
|------|----------|----------|-------------------|------------------|
| clippy | Yes (from crate dir) | No, first wins | No (`CLIPPY_CONF_DIR` env var) | No |
| cargo-deny | Yes (from manifest dir) | No, first wins | Yes | No (whole workspace graph) |
| rustfmt | Yes (from file/entry point) | No, first wins | Yes (`--config-path`) | No |
| ESLint v10 | Yes (from linted file) | No, first wins | Yes (`--config`) | No (but designed for monorepos) |
| TypeScript | Yes (from CWD, no input files) | No, first wins | Yes (`-p` / `--project`) | No (manual project refs) |
| Stylelint | Yes (cosmiconfig) | No, first wins | Yes (`--config`) | No |
| Prettier | Yes (cosmiconfig) | No, first wins | Yes (`--config`) | No |
| cspell | Yes (from file dir) | No, first wins | Yes (`--config`) | No |
| jscpd | **No** | N/A | Yes (`--config`) | No |
| pnpm audit | N/A (lockfile) | N/A | N/A | Yes (whole lockfile) |
| gitleaks | **No** (source root only) | N/A | Yes (`--config`) | No |

### Key Pattern: "First Match Wins, No Merging"
Almost every tool that walks up directories uses the same pattern: find the first config, use it, ignore everything above. **No tool merges configs from multiple directory levels automatically.** If you want inheritance, you must use explicit mechanisms like `extends` (ESLint, TypeScript, Stylelint), `import` (cspell), or manual imports in your config files.

---

## Sources

- [Clippy Configuration](https://doc.rust-lang.org/clippy/configuration.html)
- [Clippy Issue #7353 — workspace configs](https://github.com/rust-lang/rust-clippy/issues/7353)
- [cargo-deny CLI docs](https://embarkstudios.github.io/cargo-deny/cli/common.html)
- [cargo-deny config docs](https://embarkstudios.github.io/cargo-deny/checks/cfg.html)
- [rustfmt Configurations.md](https://github.com/rust-lang/rustfmt/blob/main/Configurations.md)
- [rustfmt Issue #5814 — subdirectory configs](https://github.com/rust-lang/rustfmt/issues/5814)
- [ESLint v10.0.0 Release](https://eslint.org/blog/2026/02/eslint-v10.0.0-released/)
- [ESLint Configuration Files](https://eslint.org/docs/latest/use/configure/configuration-files)
- [ESLint Discussion #16960 — flat config monorepo](https://github.com/eslint/eslint/discussions/16960)
- [TypeScript tsconfig.json docs](https://www.typescriptlang.org/docs/handbook/tsconfig-json.html)
- [TypeScript extends docs](https://www.typescriptlang.org/tsconfig/extends.html)
- [TypeScript Project References](https://www.typescriptlang.org/docs/handbook/project-references.html)
- [tsconfig/bases](https://github.com/tsconfig/bases)
- [Stylelint Configure](https://stylelint.io/user-guide/configure/)
- [Stylelint Issue #7224 — cosmiconfig strategies](https://github.com/stylelint/stylelint/issues/7224)
- [Prettier Configuration](https://prettier.io/docs/configuration)
- [Prettier Issue #12923 — .prettierignore in subdirectories](https://github.com/prettier/prettier/issues/12923)
- [CSpell Configuration](https://cspell.org/docs/Configuration)
- [CSpell Imports](https://cspell.org/docs/Configuration/imports)
- [jscpd npm](https://www.npmjs.com/package/jscpd)
- [jscpd GitHub](https://github.com/kucherenko/jscpd)
- [pnpm audit docs](https://pnpm.io/cli/audit/)
- [gitleaks Issue #1557](https://github.com/gitleaks/gitleaks/issues/1557)
- [gitleaks config](https://github.com/gitleaks/gitleaks/blob/master/config/gitleaks.toml)
