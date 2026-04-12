use g3rs_toolchain_filetree_checks_types::G3RsToolchainFileTreeChecksInput;

pub(crate) fn input(
    toolchain_toml_rel_path: Option<&str>,
    legacy_toolchain_rel_path: Option<&str>,
) -> G3RsToolchainFileTreeChecksInput {
    G3RsToolchainFileTreeChecksInput {
        toolchain_toml_rel_path: toolchain_toml_rel_path.map(str::to_owned),
        legacy_toolchain_rel_path: legacy_toolchain_rel_path.map(str::to_owned),
    }
}
