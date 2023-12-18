use core::iter;
use core::mem;
use core::ops::Range;
use core::slice;

#[cfg(not(feature = "no-std"))]
use std::vec;
#[cfg(feature = "no-std")]
use alloc::vec;

use crate::BitVec;
use crate::BITS;

impl BitVec {
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
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            bit_vec: self,
            idx: 0..self.len(),
        }
    }
}

/// An iterator for `BitVec`.
#[derive(Clone)]
pub struct Iter<'a> {
    bit_vec: &'a BitVec,
    idx: Range<usize>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        self.bit_vec.get(self.idx.next()?)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.idx.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<bool> {
        self.bit_vec.get(self.idx.next_back()?)
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl<'a> IntoIterator for &'a BitVec {
    type Item = bool;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl BitVec {
    #[inline]
    pub fn ones_from(&self, from: usize) -> Ones<'_> {
        let (byte, bit) = (from / BITS, from % BITS);
        let mask = (!0) << bit;
        let mut iter = self.as_bytes()[byte..].iter().copied();
        let (current, _) = usize_from_bytes(&mut iter);
        InnerOnes {
            base: byte * BITS,
            current: current & mask,
            iter,
        }
    }

    #[inline]
    pub fn into_ones(self) -> IntoOnes {
        let mut iter = self.into_bytes().into_iter();
        let (current, _) = usize_from_bytes(&mut iter);
        InnerOnes {
            base: 0,
            current,
            iter,
        }
    }
}

pub type Ones<'a> = InnerOnes<iter::Copied<slice::Iter<'a, u8>>>;
pub type IntoOnes = InnerOnes<vec::IntoIter<u8>>;

#[derive(Clone)]
pub struct InnerOnes<I> {
    base: usize,
    current: usize,
    iter: I,
}

impl<I: Iterator<Item = u8>> Iterator for InnerOnes<I> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        let mut c = self.current;
        while c == 0 {
            let (next, done) = usize_from_bytes(&mut self.iter);
            if done {
                return None;
            }
            self.base += BITS * mem::size_of::<usize>();
            c = next;
        }

        let lsb = c.trailing_zeros();
        self.current = c & (c - 1);
        Some(self.base + lsb as usize)
    }
}

/// Extract a `usize` from an iterator over bytes.
///
/// It helps our iterator performance to pull bits from a native word at a time.
#[inline]
fn usize_from_bytes(iter: &mut impl Iterator<Item = u8>) -> (usize, bool) {
    let mut n = 0;
    for i in 0..mem::size_of::<usize>() {
        match iter.next() {
            Some(b) => n |= usize::from(b) << (i * BITS),
            None => return (n, i == 0),
        }
    }
    (n, false)
}

#[test]
fn iter_ones_from() {
    let len = 5000;

    let zeros = BitVec::from_elem(len, false);
    assert_eq!(zeros.ones_from(0).next(), None);

    let ones = BitVec::from_elem(len, true);
    let mut iter = ones.ones_from(0);
    for i in 0..len {
        assert_eq!(iter.next(), Some(i));
        assert_eq!(ones.ones_from(i).next(), Some(i));
    }

    let mut halves = BitVec::from_elem(len * 2, false);
    for i in 0..len {
        halves.set(i * 2, true);
    }
    let mut iter = halves.ones_from(0);
    for i in 0..len {
        assert_eq!(iter.next(), Some(i * 2));
        assert_eq!(halves.ones_from(i * 2).next(), Some(i * 2));
        if i > 0 {
            assert_eq!(halves.ones_from(i * 2 - 1).next(), Some(i * 2));
        }
        assert_eq!(halves.ones_from(i).next(), Some(i + (i % 2)));
    }

    let mut sparse = BitVec::from_elem(len * 101, false);
    for i in 0..len {
        sparse.set(i * 101, true);
    }
    let mut iter = sparse.ones_from(0);
    for i in 0..len {
        let i101 = i * 101;
        assert_eq!(iter.next(), Some(i101));
        if i > 0 {
            for j in i101 - 100..=i101 {
                assert_eq!(sparse.ones_from(j).next(), Some(i101));
            }
        }
    }
}

#[test]
fn iter_into_ones() {
    let len = 5000;

    let zeros = BitVec::from_elem(len, false);
    assert_eq!(zeros.into_ones().next(), None);

    let ones = BitVec::from_elem(len, true);
    let mut iter = ones.into_ones();
    for i in 0..len {
        assert_eq!(iter.next(), Some(i));
    }

    let mut halves = BitVec::from_elem(len * 2, false);
    for i in 0..len {
        halves.set(i * 2, true);
    }
    let mut iter = halves.into_ones();
    for i in 0..len {
        assert_eq!(iter.next(), Some(i * 2));
    }

    let mut sparse = BitVec::from_elem(len * 101, false);
    for i in 0..len {
        sparse.set(i * 101, true);
    }
    let mut iter = sparse.into_ones();
    for i in 0..len {
        assert_eq!(iter.next(), Some(i * 101));
    }
}
