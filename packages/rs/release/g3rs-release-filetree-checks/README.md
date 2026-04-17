# g3rs-release-filetree-checks

Facade crate for the `release` family filetree lane.

This package validates release-related file presence at the workspace level:
- LICENSE material at the repo root
- `release-plz.toml`
- `cliff.toml`
- crate-local README files
- normalized input-failure reporting for unreadable filetree facts
