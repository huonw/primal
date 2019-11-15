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

#![deny(warnings)]

extern crate hamming;

use std::cmp::{self, Ordering};
use std::fmt;
use std::hash;
use std::ops::Index;

const BITS: usize = 64;

static TRUE: bool = true;
static FALSE: bool = false;

/// The bitvector type.
pub struct BitVec {
    /// Internal representation of the bit vector
    storage: Vec<u64>,
    /// The number of valid bits in the internal representation
    nbits: usize
}

impl Index<usize> for BitVec {
    type Output = bool;

    #[inline]
    fn index(&self, i: usize) -> &bool {
        if self.get(i).expect("index out of bounds") {
            &TRUE
        } else {
            &FALSE
        }
    }
}

impl BitVec {
    /// An operation might screw up the unused bits in the last block of the
    /// `BitVec`. As per (3), it's assumed to be all 0s. This method fixes it up.
    fn fix_last_block(&mut self) {
        let extra_bits = self.len() % BITS;
        if extra_bits > 0 {
            let mask = (1 << extra_bits) - 1;
            let storage_len = self.storage.len();
            self.storage[storage_len - 1] &= mask;
        }
    }

    /// Creates an empty `BitVec`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    /// let mut bv = BitVec::new();
    /// ```
    pub fn new() -> BitVec {
        BitVec { storage: Vec::new(), nbits: 0 }
    }

    pub fn from_u64s(data: Vec<u64>, bits: usize) -> BitVec {
        assert!(bits <= data.len() * 64);
        let mut ret = BitVec { storage: data, nbits: bits };
        ret.fix_last_block();
        ret
    }
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.storage.as_mut_ptr() as *mut _,
                                           (self.nbits + 7) / 8)
        }
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.storage.as_ptr() as *const _,
                                       (self.nbits + 7) / 8)
        }
    }

    #[inline]
    pub fn as_u64s(&self) -> &[u64] {
        &self.storage
    }

    /// Count the number of ones for the bits up to but not including
    /// the `bit`th bit.
    pub fn count_ones_before(&self, bit: usize) -> usize {
        assert!(bit <= self.nbits);
        let (byte, bit) = (bit / 8, bit % 8);
        let mask = (1 << bit) - 1;

        let bytes = self.as_bytes();

        hamming::weight(&bytes[..byte]) as usize
            + bytes.get(byte).map_or(0, |b| (b & mask).count_ones() as usize)
    }

    /// Find the index of the `n`th (0-indexed) set bit.
    pub fn find_nth_bit(&self, mut n: usize) -> Option<usize> {
        n += 1;
        let all_bytes = self.as_bytes();
        let mut bytes = all_bytes;

        while bytes.len() > 240 {
            let ix = bytes.len() / 2;
            let (first, second) = bytes.split_at(ix);

            let count = hamming::weight(first) as usize;
            match count.cmp(&n) {
                Ordering::Equal | Ordering::Greater => {
                    bytes = first;
                }
                Ordering::Less => {
                    n -= count;
                    bytes = second;
                }
            }
        }

        let mut byte_idx = bytes.as_ptr() as usize - all_bytes.as_ptr() as usize;

        let mut b = 0;
        for &b_ in bytes {
            let count = b_.count_ones() as usize;
            if count >= n {
                b = b_;
                break
            }

            byte_idx += 1;
            n -= count
        }
        if b == 0 {
            None
        } else {
            // clear the bottom n-1 set bits, so that the lowest one
            // is the one we care about
            for _ in 1..n {
                b = b & (b - 1);
            }
            assert!(b != 0);

            Some(byte_idx * 8 + b.trailing_zeros() as usize)
        }
    }

    /// Creates a `BitVec` that holds `nbits` elements, setting each element
    /// to `bit`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_elem(10, false);
    /// assert_eq!(bv.len(), 10);
    /// for x in bv.iter() {
    ///     assert_eq!(x, false);
    /// }
    /// ```
    pub fn from_elem(nbits: usize, bit: bool) -> BitVec {
        let nblocks = nbits.checked_add(BITS - 1).expect("capacity overflow") / BITS;
        let out_vec = vec![if bit { !0 } else { 0 }; nblocks];

        let mut bit_vec = BitVec {
            storage: out_vec,
            nbits: nbits,
        };

        bit_vec.fix_last_block();
        bit_vec
    }

    /// Retrieves the value at index `i`, or `None` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let bv = BitVec::from_bytes(&[0b01100000]);
    /// assert_eq!(bv.get(0), Some(false));
    /// assert_eq!(bv.get(1), Some(true));
    /// assert_eq!(bv.get(100), None);
    ///
    /// // Can also use array indexing
    /// assert_eq!(bv[1], true);
    /// ```
    #[inline]
    pub fn get(&self, i: usize) -> Option<bool> {
        if i >= self.nbits {
            return None;
        }
        let w = i / BITS;
        let b = i % BITS;
        self.storage.get(w).map(|&block|
            (block & (1 << b)) != 0
        )
    }

    /// Sets the value of a bit at an index `i`.
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_elem(5, false);
    /// bv.set(3, true);
    /// assert_eq!(bv[3], true);
    /// ```
    #[inline]
    pub fn set(&mut self, i: usize, x: bool) {
        assert!(i < self.nbits);
        unsafe {
            self.set_unchecked(i, x)
        }
    }

    #[inline]
    pub unsafe fn set_unchecked(&mut self, i: usize, x: bool) {
        //
        let w = i / BITS;
        let b = i % BITS;
        let flag = 1 << b;
        let ptr = self.storage.get_unchecked_mut(w);
        let val = if x { *ptr | flag } else { *ptr & !flag };
        *ptr = val;
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
        for w in &mut self.storage { *w = !0; }
        self.fix_last_block();
    }

    /// Returns an iterator over the elements of the vector in order.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let bv = BitVec::from_bytes(&[0b01110100, 0b10010010]);
    /// assert_eq!(bv.iter().filter(|x| *x).count(), 7);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter { bit_vec: self, next_idx: 0, end_idx: self.nbits }
    }

    /// Returns the total number of bits in this vector
    #[inline]
    pub fn len(&self) -> usize { self.nbits }

    /// Returns true if there are no bits in this vector
    #[inline]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Clears all bits in this vector.
    #[inline]
    pub fn clear(&mut self) {
        for w in &mut self.storage { *w = 0; }
    }
}

impl Default for BitVec {
    #[inline]
    fn default() -> BitVec { BitVec::new() }
}
impl Clone for BitVec {
    #[inline]
    fn clone(&self) -> BitVec {
        BitVec { storage: self.storage.clone(), nbits: self.nbits }
    }

    #[inline]
    fn clone_from(&mut self, source: &BitVec) {
        self.nbits = source.nbits;
        self.storage.clone_from(&source.storage);
    }
}

impl fmt::Debug for BitVec {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for bit in self {
            write!(fmt, "{}", if bit { 1 } else { 0 })?;
        }
        Ok(())
    }
}

impl hash::Hash for BitVec {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.nbits.hash(state);
        for elem in self.storage.iter().cloned() {
            elem.hash(state);
        }
    }
}

impl cmp::PartialEq for BitVec {
    #[inline]
    fn eq(&self, other: &BitVec) -> bool {
        if self.nbits != other.nbits {
            return false;
        }
        self.storage.iter().cloned().zip(other.storage.iter().cloned()).all(|(w1, w2)| w1 == w2)
    }
}

impl cmp::Eq for BitVec {}

/// An iterator for `BitVec`.
#[derive(Clone)]
pub struct Iter<'a> {
    bit_vec: &'a BitVec,
    next_idx: usize,
    end_idx: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        if self.next_idx != self.end_idx {
            let idx = self.next_idx;
            self.next_idx += 1;
            Some(self.bit_vec[idx])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.end_idx - self.next_idx;
        (rem, Some(rem))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<bool> {
        if self.next_idx != self.end_idx {
            self.end_idx -= 1;
            Some(self.bit_vec[self.end_idx])
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl<'a> IntoIterator for &'a BitVec {
    type Item = bool;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

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
}
