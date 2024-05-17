#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::ops::Index;

pub struct Scatter<'a, T> {
    slices: Vec<&'a [T]>,
}

impl<'a, T> Scatter<'a, T> {
    pub fn new(slices: &[&'a [T]]) -> Self {
        Scatter { slices: slices.to_vec() }
    }
}

impl<'a, T> Index<usize> for Scatter<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut index = index;
        for slice in &self.slices {
            if index < slice.len() {
                return &slice[index];
            }
            index -= slice.len();
        }
        panic!("index out of bounds");
    }
}

#[test]
fn test_scatter() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let c = [7, 8, 9];

    let scatter = Scatter::new(&[&a, &b, &c]);

    assert_eq!(scatter[0], 1);
    assert_eq!(scatter[5], 6);
}

#[test]
#[should_panic]
fn test_scanner_panic() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let c = [7, 8, 9];

    let scatter = Scatter::new(&[&a, &b, &c]);

    let _ = scatter[9];
}
