#!/usr/bin/env python3
"""Layer 4: forbidden Cargo deps.

Each [[forbidden_dep]] declares: from a `from` glob (matching package
directories) to a `to_glob` (matching dependency package names),
the dependency must NOT exist.

Approach: for each package matching `from`, parse its Cargo.toml,
read `[dependencies]`, `[dev-dependencies]`, and `[build-dependencies]`,
check no key matches `to_glob`.
"""

from __future__ import annotations

import fnmatch
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _lib import REPO_ROOT, load_manifest, section


def matching_cargo_tomls(from_glob: str) -> list[Path]:
    # `from_glob` looks like `packages/rs/hooks/g3rs-hooks-source-checks/**`.
    # Strip the trailing `/**` if any.
    base = from_glob.rstrip("/")
    if base.endswith("/**"):
        base = base[: -len("/**")]
    base_path = REPO_ROOT / base
    if not base_path.exists():
        return []
    out: list[Path] = []
    for cargo in base_path.rglob("Cargo.toml"):
        if "/target/" in str(cargo) or "/.cargo-target/" in str(cargo):
            continue
        out.append(cargo)
    return out


def deps_in(cargo_toml: Path) -> list[str]:
    text = cargo_toml.read_text()
    deps: list[str] = []
    section_re = re.compile(
        r"^\[(?:dev-|build-)?dependencies(?:\.[\w\-]+)?\]\s*$",
        re.MULTILINE,
    )
    starts = [m.start() for m in section_re.finditer(text)]
    for i, start in enumerate(starts):
        end = starts[i + 1] if i + 1 < len(starts) else len(text)
        body = text[start:end]
        for line in body.splitlines()[1:]:
            line = line.strip()
            if not line or line.startswith("#") or line.startswith("["):
                continue
            m = re.match(r"^([\w\-]+)\s*=", line)
            if m:
                deps.append(m.group(1))
    return deps


def main() -> int:
    manifest = load_manifest()
    failures: list[str] = []

    for entry in section(manifest, "forbidden_dep"):
        from_glob = entry["from"]
        to_glob = entry["to_glob"]
        for cargo in matching_cargo_tomls(from_glob):
            for dep in deps_in(cargo):
                if fnmatch.fnmatch(dep, to_glob):
                    rel = cargo.relative_to(REPO_ROOT)
                    failures.append(
                        f"forbidden dep: {rel} -> {dep} (matches {to_glob})"
                    )

    if failures:
        print("layer:4-deps status:FAIL")
        for f in failures:
            print(f"  {f}")
        return 1
    print("layer:4-deps status:PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
