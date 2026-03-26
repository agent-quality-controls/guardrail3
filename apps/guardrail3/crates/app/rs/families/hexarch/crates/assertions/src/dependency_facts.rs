pub fn assert_member_count<T: std::fmt::Debug>(members: &[T], expected: usize) {
    assert_eq!(members.len(), expected, "{members:#?}");
}

pub fn assert_no_cycles<T: std::fmt::Debug>(cycles: &[T]) {
    assert!(
        cycles.is_empty(),
        "expected no cycles in dependency facts, got: {cycles:#?}"
    );
}
