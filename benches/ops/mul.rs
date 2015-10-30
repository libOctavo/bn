use test;

macro_rules! bench {
    ($name:ident -> ($a:expr, $b:expr)) => {
        #[bench]
        fn $name(bn: &mut test::Bencher) {
            let a = ::gen_int($a);
            let b = ::gen_int($b);

            bn.iter(|| test::black_box(a * b));

            bn.bytes = 32 * ($a + $b);
        }
    }
}

bench!(limbs_1_1 -> (1, 1));
bench!(limbs_2_2 -> (2, 2));
bench!(limbs_3_3 -> (3, 3));
bench!(limbs_4_4 -> (4, 4));
bench!(limbs_5_5 -> (5, 5));
bench!(limbs_6_6 -> (6, 6));
bench!(limbs_7_7 -> (7, 7));
bench!(limbs_8_8 -> (8, 8));
bench!(limbs_9_9 -> (9, 9));
bench!(limbs_10_10 -> (10, 10));
bench!(limbs_11_11 -> (11, 11));
bench!(limbs_12_12 -> (12, 12));
bench!(limbs_13_13 -> (13, 13));
bench!(limbs_14_14 -> (14, 14));
bench!(limbs_15_15 -> (15, 15));
bench!(limbs_16_16 -> (16, 16));
bench!(limbs_24_24 -> (24, 24));
bench!(limbs_32_32 -> (32, 32));
bench!(limbs_64_64 -> (64, 64));
bench!(limbs_128_128 -> (128, 128));
bench!(limbs_256_256 -> (256, 256));
