pub(super) fn test(tc: crate::tcrate::TCrate) {
    let root = &tc.krate.index[&tc.krate.root];
    assert_eq!(
        root.docs.as_ref().unwrap(),
        ("This crate will not be tested with Jsonpath, but pure rust code.")
    );
}
