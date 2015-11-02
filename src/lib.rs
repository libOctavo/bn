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
    len: usize
}

impl Int {
    /// Create new `Int` from raw data.
    pub fn from_raw_limbs(limbs: &[u32]) -> Self {
        assert!(limbs.len() <= LIMBS);
        let mut num = Int::default();

        unsafe { ptr::copy_nonoverlapping(limbs.as_ptr(), num.limbs.as_mut_ptr(), limbs.len()); }

        num.len = limbs.len();

        num
    }

    /// Set given bit
    pub fn set_bit(&mut self, bit: usize) {
        let div = bit / 32;
        let mo  = bit % 32;

        self.limbs[div] |= 1 << mo;
        self.len = cmp::max(div, self.len);
    }

    pub fn unset_bit(&mut self, bit: usize) {
        let div = bit / 32;
        let mo  = bit % 32;

        self.limbs[div] &= !(1 << mo);
        self.len = cmp::max(div, self.len);
    }

    pub fn max() -> Self {
        Self::from_raw_limbs(&[0xffffffff; LIMBS])
    }

    pub fn is_zero(&self) -> bool {
        let mut res = 0;
        for limb in &self.limbs[..self.len] {
            res |= *limb
        }

        res == 0
    }
}

impl Clone for Int {
    fn clone(&self) -> Self {
        Int { limbs: self.limbs, len: self.len }
    }
}

impl Copy for Int {}

impl fmt::Debug for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Int ({:?}, len: {})", &self.limbs[..], self.len)
    }
}

impl Default for Int {
    fn default() -> Self {
        Int {
            limbs: [0; MAX_BITS / 32],
            len: 0
        }
    }
}

impl cmp::PartialEq for Int {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len { return false; }

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
        if self.len != other.len { return self.len.partial_cmp(&other.len); }

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
        if num != 0 { tmp.len = 1 };

        tmp
    }
}

impl From<u64> for Int {
    fn from(num: u64) -> Self {
        let mut tmp: Int = From::from(num as u32);

        tmp.limbs[1] = (num >> 32) as u32;
        if tmp.limbs[1] != 0 {
            tmp.len = 2;
        }

        tmp
    }
}

impl ops::Add for Int {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.len = cmp::max(self.len, other.len);

        let mut carry = 0;
        for (a, b) in self.limbs[..self.len].iter_mut().zip(&other.limbs[..self.len]) {
            let tmp = *a as u64 + *b as u64 + carry;
            carry = tmp >> 32;
            *a = tmp as u32;
        }
        if carry != 0 && self.len < LIMBS {
            self.limbs[self.len] = carry as u32;
            self.len += 1;
        }

        self
    }
}

impl ops::Sub for Int {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self.len = cmp::max(self.len, other.len);

        let mut len = 0;
        let mut borrow = 0;
        for (i, (a, b)) in self.limbs[..self.len].iter_mut().zip(&other.limbs[..self.len]).enumerate() {
            let diff = BASE + (*a as u64) - (*b as u64) - (borrow as u64);

            borrow = ((diff & BASE) >> BITS) ^ 1;

            *a = (diff & 0xffffffff) as u32;

            if *a != 0 { len = i + 1; }
        }

        self.len = len;

        self
    }
}

impl ops::Mul for Int {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self.len = cmp::max(self.len, other.len) + 1;

        let mut len = 0;
        let mut carry = 0;
        for (i, (a, b)) in self.limbs[..self.len].iter_mut().zip(&other.limbs[..self.len]).enumerate() {
            let tmp = (*a as u64 * *b as u64) + carry;
            carry = tmp >> 32;
            *a = tmp as u32;

            if *a != 0 { len = i + 1 }
        }

        self.len = len;

        self
    }
}
