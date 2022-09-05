pub(crate) fn collect_example() {
    assert_eq!([Ok(()), Ok(()), Ok(())].into_iter().collect::<Result<_, ()>>(), Ok(()));
    assert_eq!([Ok(()), Err("first"), Err("second")].into_iter().collect::<Result<(), &str>>(), Err("first"));
    assert_eq!(["a", "b", "c"].into_iter().collect::<String>(), "abc");
}
