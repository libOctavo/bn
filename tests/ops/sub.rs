use bn::Int;

#[test]
fn simple() {
    let a: Int = 2048u32.into();
    let b: Int = 1024u32.into();

    assert_eq!(a - b, b);
}

#[test]
fn zero() {
    let a: Int = 2048u32.into();

    assert_eq!(a - a, Int::default());
}
