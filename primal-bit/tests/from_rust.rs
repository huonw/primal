// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate primal_bit;
use primal_bit::BitVec;

fn from_bools(bools: &[bool]) -> BitVec {
    let mut bit_vec = BitVec::from_elem(bools.len(), false);
    for (i, b) in bools.iter().enumerate() {
        bit_vec.set(i, *b)
    }
    bit_vec
}
trait Methods {
    fn eq_vec(&self, v: &[bool]) -> bool;
    fn from_bytes(x: &[u8]) -> BitVec;
    fn all(&self) -> bool;
    fn none(&self) -> bool;
    fn any(&self) -> bool;
}
impl Methods for BitVec {
    fn eq_vec(&self, v: &[bool]) -> bool {
        self.len() == v.len() &&
            self.iter().zip(v).all(|(a, &b)| a == b)
    }
    fn from_bytes(bytes: &[u8]) -> BitVec {
        let len = bytes.len().checked_mul(8).expect("capacity overflow");
        let mut bit_vec = BitVec::from_elem(len, false);

        let mut i = 0;
        for &byte in bytes {
            for x in (0..8).rev() {
                bit_vec.set(i, byte & (1 << x) != 0);
                i += 1
            }
        }
        bit_vec
    }
    fn all(&self) -> bool {
        self.iter().all(|b| b)
    }
    fn none(&self) -> bool {
        self.iter().all(|b| !b)
    }
    fn any(&self) -> bool {
        !self.none()
    }

}

#[test]
fn test_to_str() {
    let zerolen = BitVec::new();
    assert_eq!(format!("{:?}", zerolen), "");

    let eightbits = BitVec::from_elem(8, false);
    assert_eq!(format!("{:?}", eightbits), "00000000")
}

#[test]
fn test_0_elements() {
    let act = BitVec::new();
    let exp = Vec::new();
    assert!(act.eq_vec(&exp));
    assert!(act.none() && act.all());
}

#[test]
fn test_1_element() {
    let mut act = BitVec::from_elem(1, false);
    assert!(act.eq_vec(&[false]));
    assert!(act.none() && !act.all());
    act = BitVec::from_elem(1, true);
    assert!(act.eq_vec(&[true]));
    assert!(!act.none() && act.all());
}

#[test]
fn test_2_elements() {
    let mut b = BitVec::from_elem(2, false);
    b.set(0, true);
    b.set(1, false);
    assert_eq!(format!("{:?}", b), "10");
    assert!(!b.none() && !b.all());
}

#[test]
fn test_10_elements() {
    let mut act;
    // all 0

    act = BitVec::from_elem(10, false);
    assert!((act.eq_vec(
                &[false, false, false, false, false, false, false, false, false, false])));
    assert!(act.none() && !act.all());
    // all 1

    act = BitVec::from_elem(10, true);
    assert!((act.eq_vec(&[true, true, true, true, true, true, true, true, true, true])));
    assert!(!act.none() && act.all());
    // mixed

    act = BitVec::from_elem(10, false);
    act.set(0, true);
    act.set(1, true);
    act.set(2, true);
    act.set(3, true);
    act.set(4, true);
    assert!((act.eq_vec(&[true, true, true, true, true, false, false, false, false, false])));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(10, false);
    act.set(5, true);
    act.set(6, true);
    act.set(7, true);
    act.set(8, true);
    act.set(9, true);
    assert!((act.eq_vec(&[false, false, false, false, false, true, true, true, true, true])));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(10, false);
    act.set(0, true);
    act.set(3, true);
    act.set(6, true);
    act.set(9, true);
    assert!((act.eq_vec(&[true, false, false, true, false, false, true, false, false, true])));
    assert!(!act.none() && !act.all());
}

#[test]
fn test_31_elements() {
    let mut act;
    // all 0

    act = BitVec::from_elem(31, false);
    assert!(act.eq_vec(
            &[false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false]));
    assert!(act.none() && !act.all());
    // all 1

    act = BitVec::from_elem(31, true);
    assert!(act.eq_vec(
            &[true, true, true, true, true, true, true, true, true, true, true, true, true,
              true, true, true, true, true, true, true, true, true, true, true, true, true,
              true, true, true, true, true]));
    assert!(!act.none() && act.all());
    // mixed

    act = BitVec::from_elem(31, false);
    act.set(0, true);
    act.set(1, true);
    act.set(2, true);
    act.set(3, true);
    act.set(4, true);
    act.set(5, true);
    act.set(6, true);
    act.set(7, true);
    assert!(act.eq_vec(
            &[true, true, true, true, true, true, true, true, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false]));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(31, false);
    act.set(16, true);
    act.set(17, true);
    act.set(18, true);
    act.set(19, true);
    act.set(20, true);
    act.set(21, true);
    act.set(22, true);
    act.set(23, true);
    assert!(act.eq_vec(
            &[false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, true, true, true, true, true, true, true, true,
              false, false, false, false, false, false, false]));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(31, false);
    act.set(24, true);
    act.set(25, true);
    act.set(26, true);
    act.set(27, true);
    act.set(28, true);
    act.set(29, true);
    act.set(30, true);
    assert!(act.eq_vec(
            &[false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false,
              false, false, true, true, true, true, true, true, true]));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(31, false);
    act.set(3, true);
    act.set(17, true);
    act.set(30, true);
    assert!(act.eq_vec(
            &[false, false, false, true, false, false, false, false, false, false, false, false,
              false, false, false, false, false, true, false, false, false, false, false, false,
              false, false, false, false, false, false, true]));
    assert!(!act.none() && !act.all());
}

#[test]
fn test_32_elements() {
    let mut act;
    // all 0

    act = BitVec::from_elem(32, false);
    assert!(act.eq_vec(
            &[false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false]));
    assert!(act.none() && !act.all());
    // all 1

    act = BitVec::from_elem(32, true);
    assert!(act.eq_vec(
            &[true, true, true, true, true, true, true, true, true, true, true, true, true,
              true, true, true, true, true, true, true, true, true, true, true, true, true,
              true, true, true, true, true, true]));
    assert!(!act.none() && act.all());
    // mixed

    act = BitVec::from_elem(32, false);
    act.set(0, true);
    act.set(1, true);
    act.set(2, true);
    act.set(3, true);
    act.set(4, true);
    act.set(5, true);
    act.set(6, true);
    act.set(7, true);
    assert!(act.eq_vec(
            &[true, true, true, true, true, true, true, true, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false]));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(32, false);
    act.set(16, true);
    act.set(17, true);
    act.set(18, true);
    act.set(19, true);
    act.set(20, true);
    act.set(21, true);
    act.set(22, true);
    act.set(23, true);
    assert!(act.eq_vec(
            &[false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, true, true, true, true, true, true, true, true,
              false, false, false, false, false, false, false, false]));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(32, false);
    act.set(24, true);
    act.set(25, true);
    act.set(26, true);
    act.set(27, true);
    act.set(28, true);
    act.set(29, true);
    act.set(30, true);
    act.set(31, true);
    assert!(act.eq_vec(
            &[false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false,
              false, false, true, true, true, true, true, true, true, true]));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(32, false);
    act.set(3, true);
    act.set(17, true);
    act.set(30, true);
    act.set(31, true);
    assert!(act.eq_vec(
            &[false, false, false, true, false, false, false, false, false, false, false, false,
              false, false, false, false, false, true, false, false, false, false, false, false,
              false, false, false, false, false, false, true, true]));
    assert!(!act.none() && !act.all());
}

#[test]
fn test_33_elements() {
    let mut act;
    // all 0

    act = BitVec::from_elem(33, false);
    assert!(act.eq_vec(
            &[false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false]));
    assert!(act.none() && !act.all());
    // all 1

    act = BitVec::from_elem(33, true);
    assert!(act.eq_vec(
            &[true, true, true, true, true, true, true, true, true, true, true, true, true,
              true, true, true, true, true, true, true, true, true, true, true, true, true,
              true, true, true, true, true, true, true]));
    assert!(!act.none() && act.all());
    // mixed

    act = BitVec::from_elem(33, false);
    act.set(0, true);
    act.set(1, true);
    act.set(2, true);
    act.set(3, true);
    act.set(4, true);
    act.set(5, true);
    act.set(6, true);
    act.set(7, true);
    assert!(act.eq_vec(
            &[true, true, true, true, true, true, true, true, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false]));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(33, false);
    act.set(16, true);
    act.set(17, true);
    act.set(18, true);
    act.set(19, true);
    act.set(20, true);
    act.set(21, true);
    act.set(22, true);
    act.set(23, true);
    assert!(act.eq_vec(
            &[false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, true, true, true, true, true, true, true, true,
              false, false, false, false, false, false, false, false, false]));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(33, false);
    act.set(24, true);
    act.set(25, true);
    act.set(26, true);
    act.set(27, true);
    act.set(28, true);
    act.set(29, true);
    act.set(30, true);
    act.set(31, true);
    assert!(act.eq_vec(
            &[false, false, false, false, false, false, false, false, false, false, false,
              false, false, false, false, false, false, false, false, false, false, false,
              false, false, true, true, true, true, true, true, true, true, false]));
    assert!(!act.none() && !act.all());
    // mixed

    act = BitVec::from_elem(33, false);
    act.set(3, true);
    act.set(17, true);
    act.set(30, true);
    act.set(31, true);
    act.set(32, true);
    assert!(act.eq_vec(
            &[false, false, false, true, false, false, false, false, false, false, false, false,
              false, false, false, false, false, true, false, false, false, false, false, false,
              false, false, false, false, false, false, true, true, true]));
    assert!(!act.none() && !act.all());
}

#[test]
fn test_equal_differing_sizes() {
    let v0 = BitVec::from_elem(10, false);
    let v1 = BitVec::from_elem(11, false);
    assert!(v0 != v1);
}

#[test]
fn test_equal_greatly_differing_sizes() {
    let v0 = BitVec::from_elem(10, false);
    let v1 = BitVec::from_elem(110, false);
    assert!(v0 != v1);
}

#[test]
fn test_equal_sneaky_small() {
    let mut a = BitVec::from_elem(1, false);
    a.set(0, true);

    let mut b = BitVec::from_elem(1, true);
    b.set(0, true);

    assert_eq!(a, b);
}

#[test]
fn test_equal_sneaky_big() {
    let mut a = BitVec::from_elem(100, false);
    for i in 0..100 {
        a.set(i, true);
    }

    let mut b = BitVec::from_elem(100, true);
    for i in 0..100 {
        b.set(i, true);
    }

    assert_eq!(a, b);
}

#[test]
fn test_from_bytes() {
    let bit_vec = BitVec::from_bytes(&[0b10110110, 0b00000000, 0b11111111]);
    let str = concat!("10110110", "00000000", "11111111");
    assert_eq!(format!("{:?}", bit_vec), str);
}

#[test]
fn test_to_bools() {
    let bools = vec![false, false, true, false, false, true, true, false];
    assert_eq!(BitVec::from_bytes(&[0b00100110]).iter().collect::<Vec<bool>>(), bools);
}

#[test]
fn test_bit_vec_iterator() {
    let bools = vec![true, false, true, true];
    let bit_vec: BitVec = from_bools(&bools);

    assert_eq!(bit_vec.iter().collect::<Vec<bool>>(), bools);

    let long: Vec<_> = (0..10000).map(|i| i % 2 == 0).collect();
    let bit_vec: BitVec = from_bools(&long);
    assert_eq!(bit_vec.iter().collect::<Vec<bool>>(), long)
}

#[test]
fn test_small_clear() {
    let mut b = BitVec::from_elem(14, true);
    assert!(!b.none() && b.all());
    b.clear();
    assert!(b.none() && !b.all());
}

#[test]
fn test_big_clear() {
    let mut b = BitVec::from_elem(140, true);
    assert!(!b.none() && b.all());
    b.clear();
    assert!(b.none() && !b.all());
}

#[test]
fn test_small_bit_vec_tests() {
    let v = BitVec::from_bytes(&[0]);
    assert!(!v.all());
    assert!(!v.any());
    assert!(v.none());

    let v = BitVec::from_bytes(&[0b00010100]);
    assert!(!v.all());
    assert!(v.any());
    assert!(!v.none());

    let v = BitVec::from_bytes(&[0xFF]);
    assert!(v.all());
    assert!(v.any());
    assert!(!v.none());
}

#[test]
fn test_big_bit_vec_tests() {
    let v = BitVec::from_bytes(&[ // 88 bits
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0]);
    assert!(!v.all());
    assert!(!v.any());
    assert!(v.none());

    let v = BitVec::from_bytes(&[ // 88 bits
        0, 0, 0b00010100, 0,
        0, 0, 0, 0b00110100,
        0, 0, 0]);
    assert!(!v.all());
    assert!(v.any());
    assert!(!v.none());

    let v = BitVec::from_bytes(&[ // 88 bits
        0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF]);
    assert!(v.all());
    assert!(v.any());
    assert!(!v.none());
}
