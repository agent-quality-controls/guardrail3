pub fn assert_root_dirs_exclude<I>(roots: I, rel_dir: &str)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let root_dirs = roots
        .into_iter()
        .map(|root| root.as_ref().to_owned())
        .collect::<Vec<_>>();
    assert!(
        root_dirs.iter().all(|root| root != rel_dir),
        "unexpected root dir `{rel_dir}` in facts roots: {root_dirs:#?}"
    );
}
