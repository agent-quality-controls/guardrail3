use g3rs_arch_types::{
    G3RsArchFacadeSurface, G3RsArchSourceChecksInput, G3RsArchSourceCrate, G3RsArchSourceFile,
};

pub(crate) fn source_crate(rel_dir: &str) -> G3RsArchSourceCrate {
    G3RsArchSourceCrate {
        rel_dir: rel_dir.to_owned(),
        lib_rs_rel: Some(join_rel(rel_dir, "src/lib.rs")),
    }
}

pub(crate) fn input(
    crates: Vec<G3RsArchSourceCrate>,
    facade_surfaces: Vec<G3RsArchFacadeSurface>,
    source_files: Vec<G3RsArchSourceFile>,
) -> G3RsArchSourceChecksInput {
    G3RsArchSourceChecksInput {
        crates,
        facade_surfaces,
        source_files,
    }
}

fn join_rel(dir: &str, child: &str) -> String {
    if dir.is_empty() {
        child.to_owned()
    } else {
        format!("{dir}/{child}")
    }
}
