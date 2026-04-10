# Goal

Prove the extracted hook lanes actually surface misconfigured hooks and trigger the expected Rust family failures end to end.

# Approach

1. Find the current end-to-end validation entrypoint or existing integration harness for hooks and Rust families.
2. Identify the narrowest place to add an integration test if the proof does not already exist.
3. Add tests first for:
   - misconfigured hook surfaces hook-family failures
   - hook trigger logic over Rust config/source changes surfaces the expected downstream family failures
4. Run the new tests and let them fail if behavior is missing.
5. Fix the implementation at the integration boundary only if the failing tests prove a bug.
6. Re-run targeted workspaces and keep the worktree clean.

# Key decisions

- Prefer the existing validation/integration harness over inventing a new one.
- If no real integration harness exists yet for packages, add the smallest integration test around the current wired path.
- Do not broaden scope into hook config or file-tree lanes.

# Files to modify

- app or package integration test harness files once identified
- .worklogs/*
