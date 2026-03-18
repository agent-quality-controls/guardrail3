# tsconfig.base.json + per-app/package tsconfig.json

Two distinct files with different treatment.

---

## tsconfig.base.json

### Location

One at repo root. Per-app/package tsconfigs extend it via `"extends": "../../tsconfig.base.json"`.

**In steady-parent:** `tsconfig.base.json` (26 lines). Contains ONLY compilerOptions — no include, exclude, paths, or plugins.

### Contents (verified)

All 14 keys are compilerOptions:
```
target: "ES2022", lib: ["ES2022"], module: "ESNext", moduleResolution: "bundler",
esModuleInterop: true, resolveJsonModule: true, isolatedModules: true, skipLibCheck: true,
strict: true, noUncheckedIndexedAccess: true, noPropertyAccessFromIndexSignature: true,
exactOptionalPropertyTypes: true, noImplicitReturns: true, noImplicitOverride: true,
noFallthroughCasesInSwitch: true, forceConsistentCasingInFileNames: true,
allowUnreachableCode: false, allowUnusedLabels: false,
noUnusedLocals: true, noUnusedParameters: true
```

**guardrail3-managed keys (strict enforcement flags — 12 booleans):**
- `strict: true`
- `noUncheckedIndexedAccess: true`
- `noPropertyAccessFromIndexSignature: true`
- `exactOptionalPropertyTypes: true`
- `noImplicitReturns: true`
- `noImplicitOverride: true`
- `noFallthroughCasesInSwitch: true`
- `forceConsistentCasingInFileNames: true`
- `allowUnreachableCode: false`
- `allowUnusedLabels: false`
- `noUnusedLocals: true`
- `noUnusedParameters: true`

**User-owned keys (project choices):**
- `target` — depends on deployment target (ES2022, ES2017, ESNext)
- `lib` — depends on runtime (ES2022 for Node, add DOM for browsers). NOTE: guardrail3 currently generates `["ES2022", "DOM", "DOM.Iterable"]` but steady-parent has `["ES2022"]` only. DOM should NOT be in the base — apps add DOM in their own tsconfig.
- `module` — project choice (ESNext, CommonJS)
- `moduleResolution` — project choice (bundler, node, etc.)
- `esModuleInterop`, `resolveJsonModule`, `isolatedModules`, `skipLibCheck` — reasonable defaults but project might override
- `declaration`, `declarationMap`, `noEmit` — project choice. guardrail3 generates these but steady-parent doesn't have them in the base.

### Category: Merge-managed

- Ensure 12 strict boolean flags present with correct values
- LEAVE target, lib, module, moduleResolution, and any other user keys
- Do NOT add keys that aren't in the existing file (declaration, declarationMap, noEmit, $schema, _comment)

### Algorithm

```
1. Parse tsconfig.base.json as JSON (serde_json)
2. Get or create compilerOptions object
3. For each of 12 strict boolean flags:
   - If missing: ADD
   - If present with correct value: LEAVE
   - If present with wrong value (e.g., strict: false): UPDATE and validate warns "strict flag weakened"
4. All other keys: LEAVE
5. Write back as formatted JSON (preserve indentation style)
```

### Edge cases
- File doesn't exist: scaffold with all 12 flags + reasonable defaults (target ES2022, module ESNext, etc.)
- File has `$schema` key: preserve it
- File has comments (JSON5/JSONC): tsconfig.json supports `//` comments by convention. Standard JSON parsers strip them. Need a JSON parser that preserves comments, OR accept that comments are lost on merge. This is a known limitation — document it.

---

## Per-app/package tsconfig.json

### Location

Each app and package has its own tsconfig.json:
- `apps/landing/tsconfig.json` — extends base, adds Next.js jsx, plugins, path aliases
- `apps/admin/tsconfig.json` — STANDALONE (does not extend base), all flags duplicated
- `packages/content-constraints/tsconfig.json` — extends base, adds outDir/rootDir/include
- `packages/spec/tsconfig.json` — STANDALONE, all flags duplicated
- `packages/generator/tsconfig.json` — extends base
- `packages/generator/pipeline/*/tsconfig.json` — 12 pipeline stages, each extends base
- `tools/freebie-renderer/tsconfig.json` — presumably extends base
- 26 files total in steady-parent

### Three patterns
1. **Extends base** (~20 files) — inherits all strict flags, adds project-specific paths/includes/plugins
2. **Standalone** (~3 files: admin, spec, validator-types) — all strict flags inline, does not extend base
3. **App-specific additions** — jsx, Next.js plugin, path aliases (@/*, @modules/*, @domain/*, etc.)

### Category: Validate-only

guardrail3 NEVER generates or modifies per-app/package tsconfig.json. These are 100% project-specific (paths, includes, excludes, plugins, path aliases).

**What validate checks:**
- File exists (warn if app/package has no tsconfig.json)
- If extends base: OK — inherits strict flags
- If standalone: check all 12 strict boolean flags are present and correct
- If neither extends nor has all flags: ERROR — strict enforcement gap

### Algorithm (validate only)

```
1. Discover all apps and packages
2. For each: check for tsconfig.json
3. If missing: WARN
4. If present:
   a. Parse JSON
   b. Check for "extends" key containing "tsconfig.base"
   c. If extends: OK (strict flags inherited)
   d. If standalone: check all 12 strict flags in compilerOptions
   e. Missing flags: ERROR with specific flag names
```

### Edge cases
- `extends` points to a package (`"extends": "@my-org/tsconfig/base"`) not a relative path — still valid, don't flag
- `extends` is an array (TypeScript 5.0+) — each element is a path, check if any references the base
- Pipeline stage tsconfigs extend with `"../../../../tsconfig.base.json"` (deep relative path) — should still be detected
- Standalone tsconfig with strict flags set DIFFERENTLY than base — admin has `strict: true` but also `target: "ES2017"` which is looser than base's ES2022. `target` is a project choice, not a guardrail.

## Parser

tsconfig.base.json: `serde_json` for merge. Limitation: standard JSON parsers strip comments. If the file has comments, they're lost on rewrite. This is a known limitation — tsconfig.json supports comments (JSONC) but serde_json doesn't.

Alternative: use a JSONC parser that preserves comments. Rust options: `jsonc-parser` crate, or do string-level manipulation for the specific keys we need to add.

Per-app tsconfig.json: `serde_json` for read-only validation. No writing needed.

## Summary

| File | Category | guardrail3 action |
|---|---|---|
| tsconfig.base.json | Merge-managed | Ensure 12 strict flags. Leave everything else. |
| Per-app/package tsconfig.json | Validate-only | Check extends base OR has all strict flags. Never write. |
