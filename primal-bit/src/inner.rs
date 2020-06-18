//! Internals that unsafely rely on consistency between `BitVec` fields,
//! encapsulated in a module to reduce scope.
#![allow(unsafe_code)]

use crate::BITS;

/// The bitvector type.
pub struct BitVec {
    /// Internal representation of the bit vector
    storage: Vec<u8>,
    /// The number of valid bits in the internal representation
    nbits: usize,
}

impl BitVec {
    /// Creates an empty `BitVec`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::collections::BitVec;
    /// let mut bv = BitVec::new();
    /// ```
    #[inline]
    pub fn new() -> BitVec {
        BitVec {
            storage: Vec::new(),
            nbits: 0,
        }
    }

    /// Creates a `BitVec` from the given bytes.
    #[inline]
    pub fn from_bytes(data: Vec<u8>, nbits: usize) -> BitVec {
        let nbytes = nbits / BITS + usize::from(nbits % BITS != 0);
        assert_eq!(nbytes, data.len());
        let mut ret = BitVec {
            storage: data,
            nbits,
        };
        ret.fix_last_block();
        ret
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
        let nbytes = nbits / BITS + usize::from(nbits % BITS != 0);
        let out_vec = vec![if bit { !0 } else { 0 }; nbytes];

        let mut bit_vec = BitVec {
            storage: out_vec,
            nbits,
        };

        if bit {
            bit_vec.fix_last_block();
        }
        bit_vec
    }

    /// An operation might screw up the unused bits in the last block of the
    /// `BitVec`. As per (3), it's assumed to be all 0s. This method fixes it up.
    pub(crate) fn fix_last_block(&mut self) {
        let extra_bits = self.nbits % BITS;
        if extra_bits > 0 {
            let mask = (1 << extra_bits) - 1;
            let last = self.storage.last_mut().unwrap();
            *last &= mask;
        }
    }

    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.storage
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.storage
    }

    #[inline]
    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.storage
    }

    /// Returns the total number of bits in this vector
    #[inline]
    pub fn len(&self) -> usize {
        self.nbits
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
        let mask = 1 << b;
        unsafe {
            let block = *self.storage.get_unchecked(w);
            Some(block & mask != 0)
        }
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
        assert!(i < self.nbits, "index out of bounds");
        let w = i / BITS;
        let b = i % BITS;
        let mask = 1 << b;
        let bit = u8::from(x) << b;
        unsafe {
            let ptr = self.storage.get_unchecked_mut(w);
            *ptr = (*ptr & !mask) | bit;
        }
    }
}

impl Clone for BitVec {
    #[inline]
    fn clone(&self) -> BitVec {
        BitVec {
            storage: self.storage.clone(),
            nbits: self.nbits,
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &BitVec) {
        self.nbits = 0; // safe default while storage is modified
        self.storage.clone_from(&source.storage);
        self.nbits = source.nbits;
    }
}
