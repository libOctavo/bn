use bn::Int;

#[test]
fn simple() {
    let num: Int = 1024u32.into();

    assert_eq!(num + num, 2048u32);
}

#[test]
fn overflow() {
    let num: Int = 0xffffffffu32.into();

    assert_eq!(num + num, Into::<Int>::into(2 * 0xffffffffu64));
}

#[test]
fn different_sizes() {
    let a_n = 0x00000000ffffffffu64;
    let b_n = 0x0000000f00000000u64;

    let a: Int = a_n.into();
    let b: Int = b_n.into();

    assert_eq!(a + b, a_n + b_n);
}
