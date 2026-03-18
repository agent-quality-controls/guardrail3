# Additional TypeScript Pre-Build Analysis

Tools that fill gaps not covered by ESLint, stylelint, or guardrail3's existing checks.

---

## 1. `cspell` — Spell Checking in Code

Catches typos in identifiers, comments, and strings. Agent-generated code has more typos than human code — `retreive`, `caluculate`, `seperate` become API contracts that are painful to rename later.

### Package

```bash
pnpm add -Dw cspell
```

### What it checks

- Variable/function/class names
- Comments and JSDoc
- String literals
- File names
- Markdown content

### Config file: `cspell.json`

```json
{
  "version": "0.2",
  "language": "en",
  "words": [],
  "ignorePaths": [
    "node_modules/**",
    ".next/**",
    "target/**",
    "legacy/**",
    "dist/**",
    "coverage/**",
    ".velite/**",
    "pnpm-lock.yaml",
    "**/*.min.js"
  ],
  "enableFiletypes": ["typescript", "typescriptreact", "css", "json", "markdown"],
  "dictionaries": ["typescript", "node", "npm", "css", "html"],
  "minWordLength": 4
}
```

### Pre-commit hook addition

```bash
SPELL_CHANGED=$(echo "$STAGED_FILES" | grep -cE '\.(ts|tsx|css|md|json)$' || true)

if [ "$SPELL_CHANGED" -gt 0 ]; then
    echo "Running spell check..."
    if ! pnpm exec cspell --no-progress --no-summary $(echo "$STAGED_FILES" | grep -E '\.(ts|tsx|css|md|json)$'); then
        echo "Spell check failed. Fix typos before committing."
        exit 1
    fi
fi
```

### Performance

~1-2s for staged files. Negligible.

### Notes

- Project-specific terms go in the `words` array (e.g., `"velite"`, `"oklch"`, `"pnpm"`)
- Per-file overrides via `// cspell:ignore word` comments
- Works for both Rust (`.rs`) and TypeScript — can extend to Rust later

---

## 2. `type-coverage` — Type Explicitness Metric

Measures what percentage of code has explicit types vs. relying on type inference. Even with `strict: true`, TypeScript infers types in many places — and some of those inferences are wider than intended (e.g., `const x = []` infers `any[]`).

### Package

```bash
pnpm add -Dw type-coverage
```

### What it measures

- Identifiers with explicit type annotations vs. inferred
- Reports a percentage (e.g., 94.5%)
- Can enforce a minimum threshold

### Usage

```bash
# Check coverage
pnpm exec type-coverage --project apps/landing/tsconfig.json

# Enforce minimum (fail if below)
pnpm exec type-coverage --project apps/landing/tsconfig.json --at-least 95
```

### Integration

Run periodically (not pre-commit — too slow on full project). Add as a `package.json` script:

```json
"type-coverage": "type-coverage --project apps/landing/tsconfig.json --at-least 95"
```

### Notes

- Not a pre-commit tool — full project analysis takes 10-30s
- Useful as a periodic health check or CI gate
- The `--at-least` flag prevents regression

---

## 3. `license-checker` — TS Dependency License Compliance

Scans `node_modules` for license compliance. Fills the gap that `cargo-deny licenses` fills for Rust — guardrail3 has no equivalent for TypeScript dependencies.

### Package

```bash
pnpm add -Dw license-checker
```

### What it checks

- License type of every installed npm package
- Can allowlist/blocklist specific licenses
- Reports unlicensed packages

### Usage

```bash
# List all licenses
pnpm exec license-checker --summary

# Fail on forbidden licenses
pnpm exec license-checker --failOn "GPL-3.0;AGPL-3.0;SSPL-1.0;EUPL-1.1"

# Only allow known-good licenses
pnpm exec license-checker --onlyAllow "MIT;ISC;BSD-2-Clause;BSD-3-Clause;Apache-2.0;0BSD;CC0-1.0;Unlicense;BlueOak-1.0.0"
```

### Integration

Add as a `package.json` script:

```json
"license-check": "license-checker --onlyAllow 'MIT;ISC;BSD-2-Clause;BSD-3-Clause;Apache-2.0;0BSD;CC0-1.0;Unlicense;BlueOak-1.0.0'"
```

Run periodically or after dependency updates. Not pre-commit (scans full node_modules, ~3-5s).

### Notes

- Mirror the license allowlist from the Rust `deny.toml` `[licenses]` section for consistency
- `--production` flag to only check production deps (skip devDependencies)
- Alternative: `license-checker-rspack` (Rust-based, faster) or `licensee`

---

## 4. `pnpm audit` — Dependency Vulnerability Scanning

Scans npm dependencies for known CVEs. Fills the gap that `cargo-deny advisories` fills for Rust. Zero install — built into pnpm.

### Why warning, not blocker

Many npm advisories have no fix yet — you can't drop Next.js because of a vuln with no patch. Some are dev-only deps that don't ship. Some are theoretical. Blocking commits on unfixable vulns just trains people to ignore the check.

The right approach: **visibility by default, block only on critical if desired.**

### Pre-commit hook addition (informational)

```bash
# --- Dependency vulnerability scan (production deps only, informational) ---
echo "Running dependency audit..."
AUDIT_CRITICAL=$(pnpm audit --prod 2>&1 | grep -c "critical" || true)
if [ "$AUDIT_CRITICAL" -gt 0 ]; then
    echo "⚠ Critical vulnerabilities found in production dependencies."
    echo "  Run 'pnpm audit --prod' for details."
    # Uncomment to block commits on critical vulns:
    # exit 1
fi
```

### Notes

- Use `--prod` to skip devDependencies (they don't ship to users)
- ~2 seconds. Fast enough for pre-commit.
- Actionable vulns (patch available) should be fixed immediately. Non-actionable ones (no patch) are tracked but don't block.
- Alternative: add as a `package.json` script for periodic runs: `"audit": "pnpm audit --prod"`

---

## 5. Code Formatting — Biome / Prettier / dprint

Rust has `cargo fmt`. TypeScript has **no formatter at all**. Agents produce inconsistent formatting — different indentation, quote styles, trailing commas, line breaks — that creates noisy diffs.

### Options

| Tool | Speed | Notes |
|------|-------|-------|
| **Biome** | Very fast (Rust-based) | Formatter + linter in one. Use formatter-only mode to avoid overlap with ESLint. |
| **Prettier** | Moderate | The standard. Huge ecosystem. Another tool in the chain. |
| **dprint** | Very fast (Rust-based) | Pluggable. Less opinionated than Prettier. |

### Pre-commit integration

```bash
# Example with Biome (formatter-only)
if [ "$TS_CHANGED" -gt 0 ]; then
    echo "Checking formatting..."
    if ! pnpm exec biome format --check $(echo "$STAGED_FILES" | grep -E '\.(ts|tsx|mjs|json)$'); then
        echo "Formatting check failed. Run 'pnpm exec biome format --write .' to fix."
        exit 1
    fi
fi
```

### Notes

- Whichever tool is chosen, run as `--check` in pre-commit (verify, don't auto-fix)
- Initial adoption requires a one-time `format --write .` pass on the whole codebase
- ~1-3s for staged files

---

## 6. Merge Conflict Marker Detection

Agents sometimes commit unresolved merge conflicts. 3 lines of bash, zero packages.

### Pre-commit hook addition

```bash
# --- Merge conflict marker detection ---
echo "Checking for merge conflict markers..."
CONFLICT_FILES=""
for file in $STAGED_FILES; do
    [ -f "$file" ] || continue
    if grep -qE '^(<{7}|={7}|>{7})' "$file" 2>/dev/null; then
        CONFLICT_FILES="$CONFLICT_FILES $file"
    fi
done
if [ -n "$CONFLICT_FILES" ]; then
    echo "Merge conflict markers found in:$CONFLICT_FILES"
    exit 1
fi
```

---

## 7. Lockfile Integrity

Agents sometimes edit `package.json` without running `pnpm install`, causing the lockfile to be out of sync. This breaks CI and other developers.

### Pre-commit hook addition

```bash
# --- Lockfile integrity (only if package.json changed) ---
PKG_CHANGED=$(echo "$STAGED_FILES" | grep -cE 'package\.json$' || true)
if [ "$PKG_CHANGED" -gt 0 ]; then
    echo "Checking lockfile integrity..."
    if ! pnpm install --frozen-lockfile 2>/dev/null; then
        echo "pnpm-lock.yaml is out of sync with package.json. Run 'pnpm install' first."
        exit 1
    fi
fi
```

---

## 8. Bundle Size Budget

For SEO-critical sites, JavaScript bundle size directly impacts Core Web Vitals. No enforcement currently — bundle can grow silently.

### Option A: `size-limit` (per-route budgets)

```bash
pnpm add -Dw size-limit @size-limit/preset-app
```

Config in `package.json`:
```json
"size-limit": [
  { "path": "apps/landing/.next/static/**/*.js", "limit": "250 kB" }
]
```

### Option B: `@next/bundle-analyzer` (visualization, not a gate)

```bash
pnpm add -Dw @next/bundle-analyzer
```

### Notes

- Not pre-commit (requires a build). Run periodically or in CI.
- `size-limit` can fail the build if budget is exceeded.

---

## 9. i18n Completeness

> **Gate:** Only enable this check if the project uses `next-intl`, `react-intl`, `i18next`, or similar. Skip if no i18n is configured.

Missing translation keys cause runtime fallbacks or blank text. A pre-build check verifies every locale has the same keys.

### Approaches

- **`next-intl` built-in:** `next-intl` v4+ has a `--strict` mode that errors on missing keys at build time
- **`i18n-check`** / **`typesafe-i18n`:** standalone tools that compare message files across locales
- **Custom script:** diff the key sets of `en.json` vs other locale JSON files

### Notes

- Only relevant if the project has multiple locales with separate message files
- For projects with a single locale, this check adds no value — should auto-skip
- Can be a build-time check (Velite/Next.js build validates) or pre-commit

---

## Summary

| Tool | What it catches | When to run | Performance | Packages needed |
|------|----------------|-------------|-------------|-----------------|
| `cspell` | Typos in identifiers/comments/strings | Pre-commit (staged files) | ~1-2s | `cspell` |
| `type-coverage` | Untyped code that passes `strict: true` | Periodic / CI | ~10-30s | `type-coverage` |
| `license-checker` | Forbidden licenses in npm dependencies | After dep updates / CI | ~3-5s | `license-checker` |
| `pnpm audit` | Known CVEs in npm dependencies | Pre-commit (informational) | ~2s | None (built-in) |
| Biome/Prettier/dprint | Inconsistent formatting | Pre-commit (staged files) | ~1-3s | One of the three |
| Merge conflict markers | Unresolved `<<<<<<<` in files | Pre-commit | ~0s | None (bash) |
| Lockfile integrity | `pnpm-lock.yaml` out of sync | Pre-commit (if package.json changed) | ~1s | None (built-in) |
| Bundle size budget | JS bundle exceeding size threshold | Periodic / CI | Requires build | `size-limit` |
| i18n completeness | Missing translation keys | Build-time / periodic | ~1s | Varies |

### Install all packages

```bash
pnpm add -Dw cspell type-coverage license-checker size-limit @size-limit/preset-app
```

(`pnpm audit`, merge conflict markers, and lockfile integrity need no packages.)
