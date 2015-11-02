use bn::Int;

#[test]
fn simple() {
    let num: Int = 1024u32.into();

    assert_eq!(num * num, 1024u64 * 1024u64);
}

#[test]
fn simple1() {
    let a: Int = 0u32.into();
    let b: Int = 1u32.into();

    assert_eq!(a * b, 0u32);
}

#[test]
fn overflow() {
    let num: Int = 0xffffffffu32.into();

    assert_eq!(num * num, Into::<Int>::into(0xffffffffu64 * 0xffffffffu64));
}

mod quickcheck {
    use bn::Int;
    use quickcheck::quickcheck;

    #[test]
    fn simple() {
        fn prop(a: u32, b: u32) -> bool {
            let (ai, bi): (Int, Int) = (a.into(), b.into());
            ai * bi == a as u64 * b as u64
        }

        quickcheck(prop as fn(u32, u32) -> bool);
    }
}
