# .npmrc

## Location

**Where pnpm reads:** Project-level `.npmrc` next to root `package.json`. During `pnpm install` from root, per-package `.npmrc` files are NOT read (pnpm limitation). User-level `~/.npmrc` merges underneath.

**In steady-parent:** `.npmrc` at root (37 lines, 13 key=value settings with comments).

One instance. No per-app. No walk-up relevant for workspace installs.

## Contents (verified)

Every key=value in steady-parent matches guardrail3's canonical exactly:
```
strict-peer-dependencies=true
disallow-workspace-cycles=true
save-workspace-protocol=rolling
engine-strict=true
package-manager-strict-version=true
strict-dep-builds=true
verify-deps-before-run=error
minimum-release-age=1440
block-exotic-subdeps=true
trust-policy=warn          ← has inline comment about undici-types
save-prefix=               ← empty string (no ^ or ~)
public-hoist-pattern=      ← empty string
shamefully-hoist=false
```

**User might also have:** `registry=https://npm.private-registry.com`, `//registry.npmjs.org/:_authToken=${NPM_TOKEN}`, `auto-install-peers=true`, `node-linker=hoisted`, etc.

## Category: Merge-managed

All 13 guardrail keys must be present with correct values. User's extra keys (registry, auth, etc.) preserved.

## Algorithm

```
1. Read file line by line
2. For each guardrail key:
   - If key present: check value matches guardrail value
     - If matches: LEAVE (preserve comment)
     - If different: LEAVE but validate warns "npmrc setting relaxed: X is Y, baseline is Z"
   - If key missing: APPEND at end
3. User keys not in guardrail list: LEAVE
4. Comments: LEAVE (line-based merge preserves them naturally)
5. Write back
```

**Same warn-not-force model as clippy thresholds.** If the user has `shamefully-hoist=true`, we warn but don't force false. They may have a legitimate reason (some packages don't work without hoisting).

## Override mechanism

None needed. User edits .npmrc directly. guardrail3 leaves their keys alone.

## Edge cases

1. **Keys with empty values:** `save-prefix=` and `public-hoist-pattern=` have empty values. The parser must handle `key=` (no value after `=`) correctly.
2. **Comment-only lines:** Lines starting with `#` or `;` are comments. Preserve them.
3. **Auth tokens:** Lines like `//registry.npmjs.org/:_authToken=${NPM_TOKEN}` are NOT key=value pairs. They're scoped registry auth. Must be preserved and not confused with guardrail keys.
4. **Duplicate keys:** If .npmrc has the same key twice, pnpm uses the last one. Our merge should not create duplicates — check before appending.

## Parser

Line-based. Split on first `=`. Key is before `=`, value is after. Comments are lines starting with `#` or `;`. No external crate needed — simple string operations.
