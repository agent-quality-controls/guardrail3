Summary
- Removed tracked Python bytecode from `scripts/verify/__pycache__`.
- Added root ignore rules for Python bytecode so verifier runs no longer dirty the worktree.

Decisions made
- Removed the generated files from git instead of restoring them because `.pyc` files are interpreter output.
- Added generic `__pycache__/` and `*.py[cod]` ignore rules instead of only ignoring the current verifier path.
- Stopped the full `scripts/verify/all.sh` run after it spent several minutes in `layer3-rules.py`; verified the Python verifier layers relevant to bytecode execution directly.

Key files for context
- `.gitignore`
- `scripts/verify/_lib.py`
- `scripts/verify/layer5-regressions.py`

Verification
- `python3 scripts/verify/layer1-tree.py`
- `python3 scripts/verify/layer2-cli.py`
- `python3 scripts/verify/layer5-regressions.py`
- `git status --short --ignored scripts/verify/__pycache__ .gitignore` shows regenerated `__pycache__/` ignored.

Next steps
- Investigate `scripts/verify/all.sh` or `layer3-rules.py` runtime separately if full manifest verifier latency matters.
