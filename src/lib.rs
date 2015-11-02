#![deny(unreachable_code, while_true)]
#![warn(missing_docs)]

use std::ops;
use std::ptr;
use std::cmp;
use std::fmt;

pub const BITS: usize = 32;
pub const MAX_BITS: usize = 8 * 1024;

pub const LIMBS: usize = MAX_BITS / BITS;

const BASE: u64 = 1 << BITS;

/// Big number implementation with constant buffer.
pub struct Int {
    limbs: [u32; LIMBS],
}

impl Int {
    /// Create new `Int` from raw data.
    pub fn from_raw_limbs(limbs: &[u32]) -> Self {
        assert!(limbs.len() <= LIMBS);
        let mut num = Int::default();

        unsafe { ptr::copy_nonoverlapping(limbs.as_ptr(), num.limbs.as_mut_ptr(), limbs.len()); }

        num
    }

    /// Set given bit
    pub fn set_bit(&mut self, bit: usize) {
        let div = bit / 32;
        let mo  = bit % 32;

        self.limbs[div] |= 1 << mo;
    }

    pub fn unset_bit(&mut self, bit: usize) {
        let div = bit / 32;
        let mo  = bit % 32;

        self.limbs[div] &= !(1 << mo);
    }

    pub fn max() -> Self {
        Self::from_raw_limbs(&[0xffffffff; LIMBS])
    }

    pub fn is_zero(&self) -> bool {
        let mut res = 0;
        for limb in &self.limbs[..] {
            res |= *limb
        }

        res == 0
    }
}

impl Clone for Int {
    fn clone(&self) -> Self {
        Int { limbs: self.limbs, }
    }
}

impl Copy for Int {}

impl fmt::Debug for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Int ({:?})", &self.limbs[..])
    }
}

impl Default for Int {
    fn default() -> Self {
        Int {
            limbs: [0; MAX_BITS / 32],
        }
    }
}

impl cmp::PartialEq for Int {
    fn eq(&self, other: &Self) -> bool {
        let mut val = 0;
        for (a, b) in self.limbs.iter().zip(other.limbs.iter()) {
            val |= a ^ b;
        }

        val == 0
    }
}

impl cmp::Eq for Int {}

impl cmp::PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        for (a, b) in self.limbs.iter().zip(other.limbs.iter()) {
            if a.partial_cmp(b).unwrap() != cmp::Ordering::Equal {
                return a.partial_cmp(b);
            }
        }

        Some(cmp::Ordering::Equal)
    }
}

impl cmp::Ord for Int {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

macro_rules! implement {
    (PartialEq for $name:ty) => {
        impl cmp::PartialEq<$name> for Int {
            fn eq(&self, other: &$name) -> bool {
                *self == Int::from(*other)
            }
        }
    };
    (From for $name:ty) => {
        impl From<$name> for Int {
            fn from(num: $name) -> Self {
                From::from(num as u32)
            }
        }
    }
}

implement!(PartialEq for u8);
implement!(PartialEq for u16);
implement!(PartialEq for u32);
implement!(PartialEq for u64);

implement!(From for u8);
implement!(From for u16);

impl From<u32> for Int {
    fn from(num: u32) -> Self {
        let mut tmp = Int::default();

        tmp.limbs[0] = num;

        tmp
    }
}

impl From<u64> for Int {
    fn from(num: u64) -> Self {
        let mut tmp: Int = From::from(num as u32);

        tmp.limbs[1] = (num >> 32) as u32;

        tmp
    }
}

fn add_with_carry(a: u32, b: u32, carry: u32) -> (u32, u32) {
    let tmp = a as u64 + b as u64 + carry as u64;

    (tmp as u32, (tmp >> BITS) as u32)
}

fn sub_with_carry(a: u32, b: u32, carry: u32) -> (u32, u32) {
    let tmp = BASE + a as u64 - b as u64 - carry as u64;

    ((tmp & 0xffffffff) as u32, ((tmp & BASE) >> BITS) as u32 ^ 1)
}

fn mul_with_carry(a: u32, b: u32, carry: u32) -> (u32, u32) {
    let tmp = a as u64 * b as u64 + carry as u64;

    (tmp as u32, (tmp >> BITS) as u32)
}

impl ops::Add for Int {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        let mut carry = 0;
        for (a, b) in self.limbs.iter_mut().zip(&other.limbs[..]) {
            let tmp = add_with_carry(*a, *b, carry);
            *a = tmp.0 as u32;
            carry = tmp.1;
        }

        self
    }
}

impl ops::Sub for Int {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        let mut borrow = 0;
        for (i, (a, b)) in self.limbs.iter_mut().zip(&other.limbs[..]).enumerate() {
            let diff = sub_with_carry(*a, *b, borrow);

            borrow = diff.1;
            *a = diff.0;
        }

        self
    }
}

impl<'a> ops::Mul for &'a Int {
    type Output = Int;

    fn mul(self, other: Self) -> Int {
        let mut tmp = Int::default();

        for (j, b) in other.limbs[..128].iter().enumerate() {
            let mut carry = 0;
            for (i, a) in self.limbs[..128].iter().enumerate() {
                let t = *a as u64 * *b as u64;
                let sum = add_with_carry(tmp.limbs[j + i], t as u32, carry);
                tmp.limbs[j + i] = sum.0;
                carry = (t >> 32) as u32 + sum.1;
            }
        }

        tmp
    }
}

impl ops::Mul for Int {
    type Output = Self;

    fn mul(self, other: Self) -> Self { &self * &other }
}
