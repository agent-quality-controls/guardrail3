# Goal

Shrink `agent-quality-controls/guardrail3` by removing generated build and vendor artifacts from Git history.

# Current Facts

- Current `HEAD` has no tracked `target/` or `node_modules/` paths.
- Reachable history still contains generated artifacts.
- `git rev-list --all --objects` found:
  - `44,920` historical `target/` paths
  - `2,483` historical `node_modules/` paths
  - `47,829` generated paths matching the cleanup patterns
- `.git/lost-found` contains local unreachable recovery objects:
  - `42` commit objects
  - `7,422` non-commit objects
- The working tree has unrelated user-side changes:
  - `code-sessions` deleted
  - `resume` untracked

# Approach

1. Do not run history rewrite in the dirty working tree.
2. Create a temporary mirror clone from `https://github.com/agent-quality-controls/guardrail3.git`.
3. Install `git-filter-repo` if missing.
4. In the mirror clone, remove generated paths from every ref:
   - `*/target/**`
   - `*/node_modules/**`
   - `*/dist/**`
   - `*/build/**`
   - `*/coverage/**`
   - `*/.next/**`
   - `*/.astro/**`
   - `*/.turbo/**`
   - `*/.cache/**`
5. Verify rewritten mirror has no generated paths in reachable history.
6. Force-push all rewritten refs and tags to `origin`.
7. Update this local checkout by fetching the rewritten `origin/main`.
8. Preserve the user-side `resume`/`code-sessions` working-tree state while aligning the repo metadata.
9. Inspect `.git/lost-found` commit objects before deletion.
10. Delete `.git/lost-found` only after inspection shows it is local WIP recovery data, not active refs.

# Key Decisions

- Use a mirror clone to avoid mixing the rewrite with the current dirty checkout.
- Do not preserve historical `target/`, `node_modules/`, or distribution artifacts. They are generated files and should not be repository history.
- Delete `.git/lost-found` after inspection. It was local-only recovery data containing WIP commits from old sessions and anonymous blob objects. Keeping it would leave the checkout large even after the history rewrite.

# Verification

- `git rev-list --objects --all | rg '(^|/)(target|node_modules|dist|build|coverage|.next|.astro|.turbo|.cache)(/|$)'` returns no generated build/vendor paths after rewrite.
- `git count-objects -vH` shows reduced pack size in the rewritten mirror.
- `git ls-remote --heads origin main` matches the rewritten local main.
- Current working-tree user changes remain visible after the remote rewrite.
- `git count-objects -vH` shows the local checkout pack size around tens of MiB, not GiB.
