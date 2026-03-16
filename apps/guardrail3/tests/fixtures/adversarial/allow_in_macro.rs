macro_rules! suppress {
    () => {
        #[allow(clippy::unwrap_used)]
        fn inner() {}
    };
}

suppress!();
