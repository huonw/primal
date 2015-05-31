// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
// FIXME(Gankro): BitVec and BitSet are very tightly coupled. Ideally (for
// maintenance), they should be in separate files/modules, with BitSet only
// using BitVec's public API. This will be hard for performance though, because
// `BitVec` will not want to leak its internal representation while its internal
// representation as `u32`s must be assumed for best performance.

// FIXME(tbu-): `BitVec`'s methods shouldn't be `union`, `intersection`, but
// rather `or` and `and`.

// (1) Be careful, most things can overflow here because the amount of bits in
//     memory can overflow `usize`.
// (2) Make sure that the underlying vector has no excess length:
//     E. g. `nbits == 16`, `storage.len() == 2` would be excess length,
//     because the last word isn't used at all. This is important because some
//     methods rely on it (for *CORRECTNESS*).
// (3) Make sure that the unused bits in the last word are zeroed out, again
//     other methods rely on it for *CORRECTNESS*.
// (4) `BitSet` is tightly coupled with `BitVec`, so any changes you make in
// `BitVec` will need to be reflected in `BitSet`.

//! Collections implemented with bit vectors.
//!
//! # Examples
//!
//! This is a simple example of the [Sieve of Eratosthenes][sieve]
//! which calculates prime numbers up to a given limit.
//!
//! [sieve]: http://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
//!
//! ```ignore
//! use std::collections::{BitSet, BitVec};
//! use std::iter;
//!
//! let max_prime = 10000;
//!
//! // Store the primes as a BitSet
//! let primes = {
//!     // Assume all numbers are prime to begin, and then we
//!     // cross off non-primes progressively
//!     let mut bv = BitVec::from_elem(max_prime, true);
//!
//!     // Neither 0 nor 1 are prime
//!     bv.set(0, false);
//!     bv.set(1, false);
//!
//!     for i in iter::range_inclusive(2, (max_prime as f64).sqrt() as usize) {
//!         // if i is a prime
//!         if bv[i] {
//!             // Mark all multiples of i as non-prime (any multiples below i * i
//!             // will have been marked as non-prime previously)
//!             for j in (i * i..max_prime).step_by(i) { bv.set(j, false) }
//!         }
//!     }
//!     BitSet::from_bit_vec(bv)
//! };
//!
//! // Simple primality tests below our max bound
//! let print_primes = 20;
//! print!("The primes below {} are: ", print_primes);
//! for x in 0..print_primes {
//!     if primes.contains(&x) {
//!         print!("{} ", x);
//!     }
//! }
//! println!("");
//!
//! // We can manipulate the internal BitVec
//! let num_primes = primes.get_ref().iter().filter(|x| *x).count();
//! println!("There are {} primes below {}", num_primes, max_prime);
//! ```

use std::cmp;
use std::fmt;
use std::hash;
use std::iter::{repeat, FromIterator};
use std::ops::Index;
use std::slice;
use std::usize;

type MutBlocks<'a> = slice::IterMut<'a, u32>;

// Take two BitVec's, and return iterators of their words, where the shorter one
// has been padded with 0's
macro_rules! match_words {
    ($a: expr, $b: expr) => {{
        let a = &$a;
        let b = &$b;
        let a_len = a.storage.len();
        let b_len = b.storage.len();

        // have to uselessly pretend to pad the longer one for type matching
        if a_len < b_len {
            (a.storage.iter().cloned().enumerate().chain(iter::repeat(0).enumerate().take(b_len).skip(a_len)),
             b.storage.iter().cloned().enumerate().chain(iter::repeat(0).enumerate().take(0).skip(0)))
        } else {
            (a.storage.iter().cloned().enumerate().chain(iter::repeat(0).enumerate().take(0).skip(0)),
             b.storage.iter().cloned().enumerate().chain(iter::repeat(0).enumerate().take(a_len).skip(b_len)))
        }
    }}
}

static TRUE: bool = true;
static FALSE: bool = false;

/// The bitvector type.
///
/// # Examples
///
/// ```ignore
/// use std::collections::BitVec;
///
/// let mut bv = BitVec::from_elem(10, false);
///
/// // insert all primes less than 10
/// bv.set(2, true);
/// bv.set(3, true);
/// bv.set(5, true);
/// bv.set(7, true);
/// println!("{:?}", bv);
/// println!("total bits set to true: {}", bv.iter().filter(|x| *x).count());
///
/// // flip all values in bitvector, producing non-primes less than 10
/// bv.negate();
/// println!("{:?}", bv);
/// println!("total bits set to true: {}", bv.iter().filter(|x| *x).count());
///
/// // reset bitvector to empty
/// bv.clear();
/// println!("{:?}", bv);
/// println!("total bits set to true: {}", bv.iter().filter(|x| *x).count());
/// ```
pub struct BitVec {
    /// Internal representation of the bit vector
    storage: Vec<u32>,
    /// The number of valid bits in the internal representation
    nbits: usize
}

// FIXME(Gankro): NopeNopeNopeNopeNope (wait for IndexGet to be a thing)
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

/// Computes how many blocks are needed to store that many bits
fn blocks_for_bits(bits: usize) -> usize {
    // If we want 17 ints, dividing by 32 will produce 0. So we add 1 to make sure we
    // reserve enough. But if we want exactly a multiple of 32, this will actually allocate
    // one too many. So we need to check if that's the case. We can do that by computing if
    // bitwise AND by `32 - 1` is 0. But LLVM should be able to optimize the semantically
    // superior modulo operator on a power of two to this.
    //
    // Note that we can technically avoid this branch with the expression
    // `(nbits + u32::BITS - 1) / 32::BITS`, but if nbits is almost usize::MAX this will overflow.
    if bits % 32 == 0 {
        bits / 32
    } else {
        bits / 32 + 1
    }
}

/// Computes the bitmask for the final word of the vector
fn mask_for_bits(bits: usize) -> u32 {
    // Note especially that a perfect multiple of u32::BITS should mask all 1s.
    !0 >> (32 - bits % 32) % 32
}

impl BitVec {
    /// Applies the given operation to the blocks of self and other, and sets
    /// self to be the result. This relies on the caller not to corrupt the
    /// last word.
    #[inline]
    fn process<F>(&mut self, other: &BitVec, mut op: F) -> bool where F: FnMut(u32, u32) -> u32 {
        assert_eq!(self.len(), other.len());
        // This could theoretically be a `debug_assert!`.
        assert_eq!(self.storage.len(), other.storage.len());
        let mut changed_bits = 0;
        for (a, b) in self.blocks_mut().zip(other.storage.iter().cloned()) {
            let w = op(*a, b);
            changed_bits |= *a ^ w;
            *a = w;
        }
        changed_bits != 0
    }

    /// Iterator over mutable refs to  the underlying blocks of data.
    fn blocks_mut(&mut self) -> MutBlocks {
        // (2)
        self.storage.iter_mut()
    }

    /// An operation might screw up the unused bits in the last block of the
    /// `BitVec`. As per (3), it's assumed to be all 0s. This method fixes it up.
    fn fix_last_block(&mut self) {
        let extra_bits = self.len() % 32;
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

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.storage.as_mut_ptr() as *mut _,
                                           (self.nbits + 7) / 8)
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.storage.as_ptr() as *const _,
                                       (self.nbits + 7) / 8)
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
        let nblocks = blocks_for_bits(nbits);
        let mut bit_vec = BitVec {
            storage: repeat(if bit { !0 } else { 0 }).take(nblocks).collect(),
            nbits: nbits
        };
        bit_vec.fix_last_block();
        bit_vec
    }

    /// Constructs a new, empty `BitVec` with the specified capacity.
    ///
    /// The bitvector will be able to hold at least `capacity` bits without
    /// reallocating. If `capacity` is 0, it will not allocate.
    ///
    /// It is important to note that this function does not specify the
    /// *length* of the returned bitvector, but only the *capacity*.
    pub fn with_capacity(nbits: usize) -> BitVec {
        BitVec {
            storage: Vec::with_capacity(blocks_for_bits(nbits)),
            nbits: 0,
        }
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
        let w = i / 32;
        let b = i % 32;
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
        let w = i / 32;
        let b = i % 32;
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

    /// Flips all bits.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let before = 0b01100000;
    /// let after  = 0b10011111;
    ///
    /// let mut bv = BitVec::from_bytes(&[before]);
    /// bv.negate();
    /// assert_eq!(bv, BitVec::from_bytes(&[after]));
    /// ```
    #[inline]
    pub fn negate(&mut self) {
        for w in &mut self.storage { *w = !*w; }
        self.fix_last_block();
    }

    /// Calculates the union of two bitvectors. This acts like the bitwise `or`
    /// function.
    ///
    /// Sets `self` to the union of `self` and `other`. Both bitvectors must be
    /// the same length. Returns `true` if `self` changed.
    ///
    /// # Panics
    ///
    /// Panics if the bitvectors are of different lengths.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let a   = 0b01100100;
    /// let b   = 0b01011010;
    /// let res = 0b01111110;
    ///
    /// let mut a = BitVec::from_bytes(&[a]);
    /// let b = BitVec::from_bytes(&[b]);
    ///
    /// assert!(a.union(&b));
    /// assert_eq!(a, BitVec::from_bytes(&[res]));
    /// ```
    #[inline]
    pub fn union(&mut self, other: &BitVec) -> bool {
        self.process(other, |w1, w2| w1 | w2)
    }

    /// Calculates the intersection of two bitvectors. This acts like the
    /// bitwise `and` function.
    ///
    /// Sets `self` to the intersection of `self` and `other`. Both bitvectors
    /// must be the same length. Returns `true` if `self` changed.
    ///
    /// # Panics
    ///
    /// Panics if the bitvectors are of different lengths.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let a   = 0b01100100;
    /// let b   = 0b01011010;
    /// let res = 0b01000000;
    ///
    /// let mut a = BitVec::from_bytes(&[a]);
    /// let b = BitVec::from_bytes(&[b]);
    ///
    /// assert!(a.intersect(&b));
    /// assert_eq!(a, BitVec::from_bytes(&[res]));
    /// ```
    #[inline]
    pub fn intersect(&mut self, other: &BitVec) -> bool {
        self.process(other, |w1, w2| w1 & w2)
    }

    /// Calculates the difference between two bitvectors.
    ///
    /// Sets each element of `self` to the value of that element minus the
    /// element of `other` at the same index. Both bitvectors must be the same
    /// length. Returns `true` if `self` changed.
    ///
    /// # Panics
    ///
    /// Panics if the bitvectors are of different length.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let a   = 0b01100100;
    /// let b   = 0b01011010;
    /// let a_b = 0b00100100; // a - b
    /// let b_a = 0b00011010; // b - a
    ///
    /// let mut bva = BitVec::from_bytes(&[a]);
    /// let bvb = BitVec::from_bytes(&[b]);
    ///
    /// assert!(bva.difference(&bvb));
    /// assert_eq!(bva, BitVec::from_bytes(&[a_b]));
    ///
    /// let bva = BitVec::from_bytes(&[a]);
    /// let mut bvb = BitVec::from_bytes(&[b]);
    ///
    /// assert!(bvb.difference(&bva));
    /// assert_eq!(bvb, BitVec::from_bytes(&[b_a]));
    /// ```
    #[inline]
    pub fn difference(&mut self, other: &BitVec) -> bool {
        self.process(other, |w1, w2| w1 & !w2)
    }

    /// Returns `true` if all bits are 1.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_elem(5, true);
    /// assert_eq!(bv.all(), true);
    ///
    /// bv.set(1, false);
    /// assert_eq!(bv.all(), false);
    /// ```
    pub fn all(&self) -> bool {
        let mut last_word = !0;
        // Check that every block but the last is all-ones...
        self.storage.iter().cloned().all(|elem| {
            let tmp = last_word;
            last_word = elem;
            tmp == !0
        // and then check the last one has enough ones
        }) && (last_word == mask_for_bits(self.nbits))
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

    /// Returns `true` if all bits are 0.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_elem(10, false);
    /// assert_eq!(bv.none(), true);
    ///
    /// bv.set(3, true);
    /// assert_eq!(bv.none(), false);
    /// ```
    pub fn none(&self) -> bool {
        self.storage.iter().cloned().all(|w| w == 0)
    }

    /// Returns `true` if any bit is 1.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_elem(10, false);
    /// assert_eq!(bv.any(), false);
    ///
    /// bv.set(3, true);
    /// assert_eq!(bv.any(), true);
    /// ```
    #[inline]
    pub fn any(&self) -> bool {
        !self.none()
    }

    /// Shortens a `BitVec`, dropping excess elements.
    ///
    /// If `len` is greater than the vector's current length, this has no
    /// effect.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_bytes(&[0b01001011]);
    /// bv.truncate(2);
    /// assert!(bv.eq_vec(&[false, true]));
    /// ```
    pub fn truncate(&mut self, len: usize) {
        if len < self.len() {
            self.nbits = len;
            // This fixes (2).
            self.storage.truncate(blocks_for_bits(len));
            self.fix_last_block();
        }
    }

    /// Reserves capacity for at least `additional` more bits to be inserted in the given
    /// `BitVec`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_elem(3, false);
    /// bv.reserve(10);
    /// assert_eq!(bv.len(), 3);
    /// assert!(bv.capacity() >= 13);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        let desired_cap = self.len().checked_add(additional).expect("capacity overflow");
        let storage_len = self.storage.len();
        if desired_cap > self.capacity() {
            self.storage.reserve(blocks_for_bits(desired_cap) - storage_len);
        }
    }

    /// Reserves the minimum capacity for exactly `additional` more bits to be inserted in the
    /// given `BitVec`. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer `reserve` if future
    /// insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_elem(3, false);
    /// bv.reserve(10);
    /// assert_eq!(bv.len(), 3);
    /// assert!(bv.capacity() >= 13);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        let desired_cap = self.len().checked_add(additional).expect("capacity overflow");
        let storage_len = self.storage.len();
        if desired_cap > self.capacity() {
            self.storage.reserve_exact(blocks_for_bits(desired_cap) - storage_len);
        }
    }

    /// Returns the capacity in bits for this bit vector. Inserting any
    /// element less than this amount will not trigger a resizing.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::new();
    /// bv.reserve(10);
    /// assert!(bv.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.storage.capacity().checked_mul(32).unwrap_or(usize::MAX)
    }

    /// Grows the `BitVec` in-place, adding `n` copies of `value` to the `BitVec`.
    ///
    /// # Panics
    ///
    /// Panics if the new len overflows a `usize`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_bytes(&[0b01001011]);
    /// bv.grow(2, true);
    /// assert_eq!(bv.len(), 10);
    /// assert_eq!(bv.to_bytes(), [0b01001011, 0b11000000]);
    /// ```
    pub fn grow(&mut self, n: usize, value: bool) {
        // Note: we just bulk set all the bits in the last word in this fn in multiple places
        // which is technically wrong if not all of these bits are to be used. However, at the end
        // of this fn we call `fix_last_block` at the end of this fn, which should fix this.

        let new_nbits = self.nbits.checked_add(n).expect("capacity overflow");
        let new_nblocks = blocks_for_bits(new_nbits);
        let full_value = if value { !0 } else { 0 };

        // Correct the old tail word, setting or clearing formerly unused bits
        let num_cur_blocks = blocks_for_bits(self.nbits);
        if self.nbits % 32 > 0 {
            let mask = mask_for_bits(self.nbits);
            if value {
                self.storage[num_cur_blocks - 1] |= !mask;
            } else {
                // Extra bits are already zero by invariant.
            }
        }

        // Fill in words after the old tail word
        let stop_idx = cmp::min(self.storage.len(), new_nblocks);
        for idx in num_cur_blocks..stop_idx {
            self.storage[idx] = full_value;
        }

        // Allocate new words, if needed
        if new_nblocks > self.storage.len() {
            let to_add = new_nblocks - self.storage.len();
            self.storage.extend(repeat(full_value).take(to_add));
        }

        // Adjust internal bit count
        self.nbits = new_nbits;

        self.fix_last_block();
    }

    /// Removes the last bit from the BitVec, and returns it. Returns None if the BitVec is empty.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::from_bytes(&[0b01001001]);
    /// assert_eq!(bv.pop(), Some(true));
    /// assert_eq!(bv.pop(), Some(false));
    /// assert_eq!(bv.len(), 6);
    /// ```
    pub fn pop(&mut self) -> Option<bool> {
        if self.is_empty() {
            None
        } else {
            let i = self.nbits - 1;
            let ret = self[i];
            // (3)
            self.set(i, false);
            self.nbits = i;
            if self.nbits % 32 == 0 {
                // (2)
                self.storage.pop();
            }
            Some(ret)
        }
    }

    /// Pushes a `bool` onto the end.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    ///
    /// let mut bv = BitVec::new();
    /// bv.push(true);
    /// bv.push(false);
    /// assert!(bv.eq_vec(&[true, false]));
    /// ```
    pub fn push(&mut self, elem: bool) {
        if self.nbits % 32 == 0 {
            self.storage.push(0);
        }
        let insert_pos = self.nbits;
        self.nbits = self.nbits.checked_add(1).expect("Capacity overflow");
        self.set(insert_pos, elem);
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

    pub fn push_all(&mut self, bits: &BitVec, nbits: usize) {
        assert!(self.len() % 32 == 0);
        let iter = bits.storage.iter().cloned();
        let count = if nbits >= bits.len() {
            self.storage.extend(iter);
            bits.len()
        } else {
            self.storage.extend(iter.take(blocks_for_bits(nbits)));
            nbits
        };
        self.nbits = self.nbits.checked_add(count).expect("capacity overflow");
    }
}

impl Default for BitVec {
    #[inline]
    fn default() -> BitVec { BitVec::new() }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<I: IntoIterator<Item=bool>>(iter: I) -> BitVec {
        let mut ret = BitVec::new();
        ret.extend(iter);
        ret
    }
}

impl Extend<bool> for BitVec {
    #[inline]
    fn extend<I: IntoIterator<Item=bool>>(&mut self, iterable: I) {
        let iterator = iterable.into_iter();
        let (min, _) = iterator.size_hint();
        self.reserve(min);
        for element in iterator {
            self.push(element)
        }
    }
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
            try!(write!(fmt, "{}", if bit { 1 } else { 0 }));
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
