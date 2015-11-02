use test;

macro_rules! bench {
    ($name:ident -> ($a:expr, $b:expr)) => {
        #[bench]
        fn $name(bn: &mut test::Bencher) {
            let a = ::gen_int($a);
            let b = ::gen_int($b);

            bn.iter(|| test::black_box(a + b));

            bn.bytes = 32 * ($a + $b);
        }
    }
}

bench!(limbs_128_128 -> (128, 128));
bench!(limbs_256_256 -> (256, 256));
