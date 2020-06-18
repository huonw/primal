// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A very simple bit-vector that serves the needs of `primal`.

#![deny(unsafe_code)]
#![deny(warnings)]

use std::fmt;
use std::hash;
use std::ops::Index;

mod inner;
mod iter;

pub use crate::inner::BitVec;
pub use crate::iter::{Iter, IntoOnes, Ones};

const BITS: usize = 8;

impl Index<usize> for BitVec {
    type Output = bool;

    #[inline]
    fn index(&self, i: usize) -> &bool {
        if self.get(i).expect("index out of bounds") {
            &true
        } else {
            &false
        }
    }
}

impl BitVec {
    /// Count the number of ones in the entire `BitVec`.
    #[inline]
    pub fn count_ones(&self) -> usize {
        hamming::weight(self.as_bytes()) as usize
    }

    /// Count the number of ones for the bits up to but not including
    /// the `bit`th bit.
    #[inline]
    pub fn count_ones_before(&self, bit: usize) -> usize {
        assert!(bit <= self.len());
        let (byte, bit) = (bit / BITS, bit % BITS);
        let mask = (1 << bit) - 1;

        let bytes = self.as_bytes();

        hamming::weight(&bytes[..byte]) as usize
            + bytes.get(byte).map_or(0, |b| (b & mask).count_ones() as usize)
    }

    /// Find the index of the `n`th (0-indexed) set bit.
    pub fn find_nth_bit(&self, mut n: usize) -> Option<usize> {
        if n >= self.len() {
            return None;
        }
        n += 1;
        let mut bytes = self.as_bytes();

        let mut byte_idx = 0;
        while bytes.len() > 240 {
            let ix = bytes.len() / 2;
            let (first, second) = bytes.split_at(ix);

            let count = hamming::weight(first) as usize;
            if count >= n {
                bytes = first;
            } else {
                n -= count;
                bytes = second;
                byte_idx += ix;
            }
        }

        let i = bytes.iter().position(|&b| {
            let count = b.count_ones() as usize;
            if count >= n {
                true
            } else {
                n -= count;
                false
            }
        })?;

        // clear the bottom n-1 set bits, so that the lowest one
        // is the one we care about
        let mut b = bytes[i];
        for _ in 1..n {
            b &= b - 1;
        }
        assert!(b != 0);

        Some((byte_idx + i) * BITS + b.trailing_zeros() as usize)
    }

    /// Sets all bits to 1.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let before = 0b01100000;
    /// let after  = 0b11111111;
    ///
    /// let mut bv = BitVec::from_bytes(&[before]);
    /// bv.set_all();
    /// assert_eq!(bv, BitVec::from_bytes(&[after]));
    /// ```
    #[inline]
    pub fn set_all(&mut self) {
        for w in self.as_bytes_mut() { *w = !0; }
        self.fix_last_block();
    }

    /// Clears all bits in this vector.
    #[inline]
    pub fn clear(&mut self) {
        for w in self.as_bytes_mut() { *w = 0; }
    }

    /// Returns true if there are no bits in this vector
    #[inline]
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

impl Default for BitVec {
    #[inline]
    fn default() -> BitVec { BitVec::new() }
}

impl fmt::Debug for BitVec {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in self {
            write!(fmt, "{}", if bit { 1 } else { 0 })?;
        }
        Ok(())
    }
}

impl hash::Hash for BitVec {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.len());
        state.write(self.as_bytes());
    }
}

impl PartialEq for BitVec {
    #[inline]
    fn eq(&self, other: &BitVec) -> bool {
        self.len() == other.len() && self.as_bytes() == other.as_bytes()
    }
}

impl Eq for BitVec {}

#[cfg(test)]
mod tests {
    use super::BitVec;

    #[test]
    fn count_ones_before() {
        let len = 10000;

        let ones = BitVec::from_elem(len, true);
        let zeros = BitVec::from_elem(len, false);
        let mut halves = zeros.clone();
        for i in 0..len / 2 {
            halves.set(i * 2, true);
        }
        for i in 0..len + 1 {
            assert_eq!(ones.count_ones_before(i), i);
            assert_eq!(zeros.count_ones_before(i), 0);
            assert_eq!(halves.count_ones_before(i), (i + 1) / 2);
        }
    }

    #[test]
    fn find_nth_bit() {
        let len = 5000;

        let ones = BitVec::from_elem(len, true);
        let mut halves = BitVec::from_elem(len * 2, false);
        for i in 0..len {
            halves.set(i * 2, true);
        }
        for i in 0..len {
            assert_eq!(ones.find_nth_bit(i), Some(i));
            assert_eq!(halves.find_nth_bit(i), Some(i * 2));
        }
        assert_eq!(ones.find_nth_bit(len + 1), None);
        assert_eq!(halves.find_nth_bit(len + 1), None);

        assert_eq!(BitVec::new().find_nth_bit(0), None);
        assert_eq!(BitVec::from_elem(len, false).find_nth_bit(0), None);
    }

    #[test]
    fn get() {
        let len = 10000;

        let mut halves = BitVec::from_elem(len, false);
        for i in 0..len {
            assert_eq!(halves.get(i), Some(false));
        }
        for i in 0..len / 2 {
            halves.set(i * 2, true);
        }
        for i in 0..len {
            assert_eq!(halves.get(i), Some(i % 2 == 0));
        }
        assert_eq!(halves.get(len), None);
    }

    #[test]
    fn clone() {
        let len = 10000;

        let ones = BitVec::from_elem(len, true);
        let zeros = BitVec::from_elem(len, false);
        let mut halves = zeros.clone();
        for i in 0..len / 2 {
            halves.set(i * 2, true);
        }

        assert_eq!(ones.clone(), ones);
        assert_eq!(zeros.clone(), zeros);
        assert_eq!(halves.clone(), halves);

        let mut bv = BitVec::from_elem(len, false);
        bv.clone_from(&ones);
        assert_eq!(bv, ones);
        bv.clone_from(&zeros);
        assert_eq!(bv, zeros);
        bv.clone_from(&halves);
        assert_eq!(bv, halves);
    }

    #[test]
    fn len_is_empty() {
        let len = 10000;

        let ones = BitVec::from_elem(len, true);
        let zeros = BitVec::from_elem(len, false);
        let default = BitVec::default();

        assert_eq!(ones.len(), len);
        assert_eq!(zeros.len(), len);
        assert_eq!(default.len(), 0);

        assert!(!ones.is_empty());
        assert!(!zeros.is_empty());
        assert!(default.is_empty());
    }

    #[test]
    fn clear_set_all() {
        let len = 10000;

        let ones = BitVec::from_elem(len, true);
        let zeros = BitVec::from_elem(len, false);

        let mut bv = ones.clone();
        assert_eq!(bv, ones);
        bv.clear();
        assert_eq!(bv, zeros);
        bv.set_all();
        assert_eq!(bv, ones);
    }
}
