# Summary

Rewrote the repository history in a temporary mirror clone to remove generated build and vendor artifacts from every pushed branch, then force-pushed the cleaned refs to `agent-quality-controls/guardrail3`.

# Decisions Made

- Used a temporary mirror clone instead of rewriting the dirty working tree, because the checkout already had user-side `code-sessions`/`resume` changes.
- Removed generated paths from history:
  - `*/target/**`
  - `*/node_modules/**`
  - `*/dist/**`
  - `*/build/**`
  - `*/.next/**`
  - `*/.astro/**`
  - `*/.turbo/**`
  - `*/.cache/**`
- Did not remove broad `coverage` paths, because this repo has real source and behavior files under `coverage` directories.
- Deleted local `.git/lost-found` after inspecting it. It contained old local WIP recovery commits and anonymous objects, not active refs.
- Deleted stale local-only agent branches after the remote rewrite, because they kept the polluted history reachable locally.

# Key Files

- `.plans/2026-05-15-120407-clean-generated-history.md`

# Verification

- `git rev-list --objects --all | rg '(^|/)(target|node_modules|dist|build|.next|.astro|.turbo|.cache)(/|$)'` returns zero matches.
- Mirror clone pack size changed from `1.69 GiB` to `19.36 MiB`.
- Local checkout `.git` size changed to about `20 MiB`.
- Local `git count-objects -vH` reports one pack at `17.95 MiB`.
- `origin/main` points at rewritten commit `78ca8ea6f2155dadd597429c5420582815773d4a`.

# Next Steps

- Reclone existing old checkouts instead of pulling across the rewritten history.
- Keep generated artifact paths out of future commits with existing hooks and `.gitignore`.
