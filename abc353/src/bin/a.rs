#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        n:usize,
        h:[isize;n],
    }
    for i in 1..n {
        if h[i] > h[0] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", -1);
}
