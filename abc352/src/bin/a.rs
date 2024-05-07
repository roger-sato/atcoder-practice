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
        mut x:usize,
        mut y:usize,
        z:usize,
    }
    if x > y {
        swap(&mut x, &mut y)
    }

    println!("{}", if x <= z && z <= y { "Yes" } else { "No" });
}
