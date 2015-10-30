use bn::Int;

#[test]
fn simple() {
    let num: Int = 1024u32.into();

    assert_eq!(num * num, 1024u64 * 1024u64);
}

#[test]
fn overflow() {
    let num: Int = 0xffffffffu32.into();

    assert_eq!(num * num, Into::<Int>::into(0xffffffffu64 * 0xffffffffu64));
}
