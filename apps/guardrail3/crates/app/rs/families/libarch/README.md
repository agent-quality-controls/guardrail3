# RS-LIBARCH

Rust legacy layered-library checks for package-owned library roots.

This family is in retirement. The generic split-library architecture contract
now belongs in `RS-ARCH`.

`RS-LIBARCH` only retains the old layered-shape specifics that are still
temporarily present in the repo:

- exact layered crate set once layered mode exists
- dependency direction between `api`, `core`, and `infra`
- old layered facade/export policy tied to that shape

It no longer owns:

- escalation from flat library into split architecture
- split-root workspace-facade requirements
- generic split-root existence requirements

Those now belong in `RS-ARCH`.
