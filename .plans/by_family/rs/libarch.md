# RS-LIBARCH

Status: planned, not implemented.

Implementation root:

- none yet

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/rs/libarch.md` as the detailed design ledger until code and a family README exist

Current state:

- library/package architecture family is still a design lane, not a live runtime family
- `RS-ARCH` already models package ownership and `libarch` enablement, but the package-structure family itself does not exist yet

Historical/supplemental references:

- `.plans/todo/checks/rs/libarch.md`
- `arch` and `hexarch` docs where package/app ownership boundaries are already described

Next planning focus:

- define the initial family README once code exists
- keep package architecture separate from generic Cargo policy and from repo-global `arch`
