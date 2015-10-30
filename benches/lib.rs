#![feature(test)]

extern crate bn;
extern crate rand;
extern crate test;

use rand::Rng;

pub fn gen_int(limbs: usize) -> bn::Int {
    let mut rng = rand::thread_rng();
    let vec: Vec<_> = (0..limbs).map(|_| rng.gen::<u32>() & 0x7fffffff).collect();

    bn::Int::from_raw_limbs(&vec)
}

mod ops;
