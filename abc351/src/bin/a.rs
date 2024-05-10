#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        a:[usize;9],
        b:[usize;8],
    }
    let sa: usize = a.iter().sum();
    let sb: usize = b.iter().sum();
    println!("{}", sa - sb + 1);
}
