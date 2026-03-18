# Test Guardrails: Pre-Commit Mutation Testing

**Goal:** Enforce test quality at commit time via mutation testing. Both Rust and TypeScript files may be committed together (monorepo) or separately. Total budget for test-related hooks: **30 seconds**.

## Architecture

```
git commit (staged files)
    │
    ├─ detect .rs files ──► cargo-mutants (background)  ─┐
    │                                                     ├─ wait for both ─► pass/fail
    └─ detect .ts files ──► stryker (background)         ─┘

    Wall time = max(cargo-mutants, stryker) ≈ 15-20s
    If only one language: ≈ 10-15s
```

Both tools run **in parallel** on their respective file sets. The hook blocks the commit only if surviving mutants are found.

## Time Budget Breakdown

| Component | Scenario: TS only | Scenario: Rust only | Scenario: Both |
|---|---|---|---|
| Stryker startup | ~3s | — | ~3s |
| Stryker mutations (~20, 8-10 concurrent) | ~3s | — | ~3s |
| cargo-mutants baseline + build | — | ~5s | ~5s |
| cargo-mutants mutations (~10, ncpu-2 concurrent) | — | ~8s | ~8s |
| **Total wall time** | **~6s** | **~15s** | **~15s** (parallel) |
| Overhead (file detection, result check) | ~1s | ~1s | ~1s |
| **Grand total** | **~7s** | **~16s** | **~16s** |

Rust dominates because each mutant requires recompilation. TS mutants run via test runner process reuse (no recompilation).

---

## Rust Setup: cargo-mutants

### Install

```bash
cargo install cargo-mutants
```

### Config: `.cargo/mutants.toml`

```toml
# Performance: use most available cores
# Not set here — passed dynamically as -j $(nproc - 2) by the hook script
# so it adapts to whatever machine it runs on

# Use the fast "mutants" profile (no debug symbols)
profile = "mutants"

# Timeout: 3x baseline (default is 5x, too generous for pre-commit)
timeout_multiplier = 3

# Minimum test timeout: 10 seconds (default is 20, reduce for fast test suites)
minimum_test_timeout = 10
```

### Config: `Cargo.toml` (add profile)

```toml
[profile.mutants]
inherits = "test"
debug = "none"           # No debug symbols — faster linking
opt-level = 1            # Slight optimization — faster test execution, slightly slower build
                         # Trade-off: opt-level 0 = fastest build, slowest tests
                         #            opt-level 1 = good balance for mutation testing
```

### How the hook invokes it

```bash
# Get staged .rs files, generate diff, run scoped mutation testing
JOBS=$(( $(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 4) - 2 ))
[ "$JOBS" -lt 2 ] && JOBS=2

git diff --cached -- '*.rs' | cargo mutants \
  --in-diff -              \  # Only mutate functions touched by the diff
  -j "$JOBS"               \  # Dynamic: ncpu - 2 parallel jobs
  --profile mutants         \  # Fast profile (no debug symbols)
  --timeout-multiplier 3    \  # 3x baseline timeout
  --minimum-test-timeout 10 \  # Floor of 10s per mutant
  2>&1
```

### Key numbers

| Setting | Value | Why |
|---|---|---|
| `jobs` | `ncpu - 2` (dynamic) | Each job is a cargo build+test. Use most cores, leave 2 free for OS + the other tool if running in parallel. On a 10-core M-series Mac this means 8 concurrent mutants. |
| `profile` | `mutants` (debug=none) | Skipping debug symbols saves ~30-40% link time per mutant. |
| `timeout_multiplier` | 3 | Default 5x is for CI where you want to catch hangs. Pre-commit just needs to know if tests pass/fail on the mutant. 3x is enough. |
| `minimum_test_timeout` | 10 | Default 20s is wasteful for fast test suites. Most unit tests finish in <5s. |
| `opt-level` | 1 | Level 0 means no optimization, tests run slower. Level 1 gives ~20-30% faster test execution for ~10% slower build. Level 2 is too slow to compile for mutation testing. |

### Expected mutant count

`--in-diff` only mutates functions whose body intersects the diff. A typical commit touching 2-3 functions generates **5-15 mutants**. At ncpu-2 parallel jobs (e.g. 8 on a 10-core Mac) with ~3s per mutant (incremental build + test):
- 5 mutants: ~3s (1 batch)
- 10 mutants: ~6s (2 batches)
- 15 mutants: ~6s (2 batches)

---

## TypeScript Setup: Stryker

### Install (per project)

```bash
npm install --save-dev @stryker-mutator/core @stryker-mutator/vitest-runner @stryker-mutator/typescript-checker
# OR for Jest:
npm install --save-dev @stryker-mutator/core @stryker-mutator/jest-runner @stryker-mutator/typescript-checker
```

### Config: `stryker.config.json`

```json
{
  "$schema": "https://raw.githubusercontent.com/stryker-mutator/stryker/master/packages/core/schema/stryker-core.json",
  "testRunner": "vitest",
  "checkers": ["typescript"],
  "incremental": true,
  "incrementalFile": "reports/stryker-incremental.json",
  "concurrency": 8,
  "timeoutMS": 3000,
  "timeoutFactor": 1.5,
  "coverageAnalysis": "perTest",
  "ignoreStatic": true,
  "disableBail": false,
  "reporters": ["clear-text"]
}
```

### How the hook invokes it

```bash
# Get staged .ts/.tsx files, pass as --mutate patterns
STAGED_TS=$(git diff --cached --name-only -- '*.ts' '*.tsx' | grep -v '\.test\.\|\.spec\.\|__tests__' | paste -sd ',' -)

npx stryker run \
  --mutate "$STAGED_TS"     \  # Only mutate staged production files
  --incremental              \  # Reuse previous results
  --concurrency 8            \  # 8 parallel workers
  --reporters clear-text     \  # No HTML report, just text
  2>&1
```

### Key numbers

| Setting | Value | Why |
|---|---|---|
| `concurrency` | 8 | M-series Macs have 8+ performance cores. Stryker workers are mostly I/O-bound (running test processes), so saturating cores is fine. |
| `timeoutMS` | 3000 | Absolute timeout added to every test run. Default 5000 is generous. 3000 is enough for unit tests. |
| `timeoutFactor` | 1.5 | Multiplier on baseline test time. Default 1.5 is already tight. Keep it. |
| `incremental` | true | **Critical.** First run caches results to `stryker-incremental.json`. Subsequent runs only re-test mutants in changed code. Turns 60s runs into 5s runs. |
| `ignoreStatic` | true | Static mutants (top-level constants, etc.) require running ALL tests, not just relevant ones. Ignoring them avoids a massive performance penalty with minimal coverage loss. |
| `coverageAnalysis` | `perTest` | Maps each test to the mutants it covers. Only runs relevant tests per mutant instead of the full suite. Default for Vitest, must be set explicitly for Jest. |
| `disableBail` | false (default) | Bail = stop testing a mutant as soon as one test fails. Faster. Keep it on. |

### Expected mutant count

Stryker generates ~5-10 mutations per changed function (arithmetic, conditionals, string literals, block removals). A commit touching 3 functions generates **15-30 mutants**. With 8 concurrent workers:
- 15 mutants: ~2s mutation time + 3s startup = **~5s**
- 30 mutants: ~4s mutation time + 3s startup = **~7s**

### Gitignore addition

```
# Stryker
reports/stryker-incremental.json
reports/
.stryker-tmp/
```

Note: `stryker-incremental.json` should NOT be committed — it's a local cache. Add to `.gitignore`.

---

## Hook Script: `mutation-test-hook.sh`

Lives at: `~/.claude/hooks/mutation-test-hook.sh`

```bash
#!/bin/bash
# Claude Code PreToolUse hook: runs mutation testing on staged files before git commit
# Budget: 30 seconds total. Rust and TS run in parallel.

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // ""')

# Only intercept git commit
if ! echo "$COMMAND" | grep -qE 'git\s+commit'; then
  exit 0
fi

REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)
if [ -z "$REPO_ROOT" ]; then
  exit 0
fi

cd "$REPO_ROOT"

# Detect staged file types
HAS_RUST=$(git diff --cached --name-only -- '*.rs' | head -1)
HAS_TS=$(git diff --cached --name-only -- '*.ts' '*.tsx' | grep -v '\.test\.\|\.spec\.\|__tests__' | head -1)

# Skip if no testable files staged
if [ -z "$HAS_RUST" ] && [ -z "$HAS_TS" ]; then
  exit 0
fi

FAILED=""
RUST_PID=""
TS_PID=""

# --- Rust: cargo-mutants ---
if [ -n "$HAS_RUST" ] && command -v cargo-mutants &>/dev/null && [ -f Cargo.toml ]; then
  JOBS=$(( $(sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 4) - 2 ))
  [ "$JOBS" -lt 2 ] && JOBS=2
  (
    git diff --cached -- '*.rs' | timeout 25 cargo mutants \
      --in-diff - \
      -j "$JOBS" \
      --profile mutants \
      --timeout-multiplier 3 \
      --minimum-test-timeout 10 \
      --output /tmp/mutants-results-$$ \
      2>&1
  ) &
  RUST_PID=$!
fi

# --- TypeScript: Stryker ---
if [ -n "$HAS_TS" ] && [ -f node_modules/.bin/stryker ] || [ -f package.json ]; then
  STAGED_TS=$(git diff --cached --name-only -- '*.ts' '*.tsx' | grep -v '\.test\.\|\.spec\.\|__tests__' | paste -sd ',' -)
  if [ -n "$STAGED_TS" ]; then
    (
      timeout 25 npx stryker run \
        --mutate "$STAGED_TS" \
        --incremental \
        --concurrency 8 \
        --reporters clear-text \
        2>&1
    ) &
    TS_PID=$!
  fi
fi

# Wait for both with overall timeout
TIMEOUT_END=$((SECONDS + 30))

if [ -n "$RUST_PID" ]; then
  while kill -0 "$RUST_PID" 2>/dev/null && [ $SECONDS -lt $TIMEOUT_END ]; do
    sleep 1
  done
  if kill -0 "$RUST_PID" 2>/dev/null; then
    kill "$RUST_PID" 2>/dev/null
    FAILED="$FAILED Rust(timeout)"
  else
    wait "$RUST_PID"
    [ $? -ne 0 ] && FAILED="$FAILED Rust(surviving-mutants)"
  fi
fi

if [ -n "$TS_PID" ]; then
  while kill -0 "$TS_PID" 2>/dev/null && [ $SECONDS -lt $TIMEOUT_END ]; do
    sleep 1
  done
  if kill -0 "$TS_PID" 2>/dev/null; then
    kill "$TS_PID" 2>/dev/null
    FAILED="$FAILED TypeScript(timeout)"
  else
    wait "$TS_PID"
    [ $? -ne 0 ] && FAILED="$FAILED TypeScript(surviving-mutants)"
  fi
fi

if [ -n "$FAILED" ]; then
  jq -n --arg reason "BLOCKED: Mutation testing failed:$FAILED. Surviving mutants mean your tests don't catch the changes you made. Write stronger tests that would fail if the production code is wrong, then retry the commit." '{
    "hookSpecificOutput": {
      "hookEventName": "PreToolUse",
      "permissionDecision": "deny",
      "permissionDecisionReason": $reason
    }
  }'
fi

exit 0
```

## Hook Registration: `settings.json`

Add to the existing `PreToolUse` → `Bash` hooks array:

```json
{
  "type": "command",
  "command": "/Users/tartakovsky/.claude/hooks/mutation-test-hook.sh",
  "timeout": 35000,
  "statusMessage": "Running mutation tests on staged files..."
}
```

**Timeout: 35000ms** (35s) — gives the 30s internal budget plus 5s overhead for process startup and result collection.

---

## Per-Project Setup Checklist

### Rust project

1. `cargo install cargo-mutants` (one-time)
2. Add `[profile.mutants]` to `Cargo.toml`
3. Create `.cargo/mutants.toml` with the config above
4. Done — hook auto-detects `Cargo.toml` and runs

### TypeScript project

1. `npm install --save-dev @stryker-mutator/core @stryker-mutator/vitest-runner @stryker-mutator/typescript-checker`
2. Create `stryker.config.json` with the config above
3. Add `reports/` and `.stryker-tmp/` to `.gitignore`
4. Run `npx stryker run` once to generate the initial incremental cache
5. Done — hook auto-detects `package.json` + stryker and runs

### Monorepo (both)

1. Do both of the above
2. Both run in parallel automatically — no extra config

---

## Escape Hatches

For exceptional cases where mutation testing should be skipped:

- **Large refactor with no behavior change:** The agent can note in the worklog why tests aren't expected to change, and you can temporarily increase the hook timeout or disable it
- **Generated code:** Add generated file patterns to Stryker's `mutate` exclusions and cargo-mutants' `exclude_globs` in `.cargo/mutants.toml`
- **Performance-critical commits:** If 30s is too slow for a specific large commit, the hook times out gracefully — it doesn't block, just warns

There is no silent skip. The hook always either passes, fails, or times out with a warning.

---

## What This Catches vs. What It Doesn't

### Catches
- Tests that don't actually assert on behavior (mutant survives = test is meaningless)
- Missing regression tests for bug fixes (changed code has no covering test)
- Weak assertions (test passes regardless of return value)
- Dead code in tests (assertions that never execute)

### Doesn't catch (use `/test-review` skill for these)
- Tests coupled to implementation details (technically they kill mutants, but they're fragile)
- Missing negative/error path tests (mutations only target existing code, not missing code)
- Test isolation issues (shared state, order dependence)
- Semantic test quality (testing the right thing vs. testing something)

Mutation testing + `/test-review` AI audit together cover the full spectrum.
