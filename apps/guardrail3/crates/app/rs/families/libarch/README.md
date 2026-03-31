# RS-LIBARCH

Rust layered-library architecture checks for package-owned library roots.

This family enforces when a package library must escalate from a flat crate into
the layered workspace shape:

- root facade package at the package root
- `crates/api`
- `crates/core`
- optional `crates/infra`

It owns:

- escalation from flat library to layered workspace
- exact layered crate set once layered mode exists
- dependency direction between `api`, `core`, and `infra`
- root facade export policy

It does not replace:

- `RS-ARCH` for placement ownership
- `RS-ARCH` for workspace-membership exactness
- `RS-CODE` for generic facade/source quality
- `RS-DEPS` for dependency allowlist policy
