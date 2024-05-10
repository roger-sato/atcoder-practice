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
        ab:[(isize,isize);n]
    }
    let mut mi = 0;
    let mut mx = 0;
    for (i, (a, b)) in ab.iter().enumerate() {
        if b - a > mx {
            mx = b - a;
            mi = i
        }
    }
    let ans: isize = ab.iter().map(|&x| x.0).sum();

    println!("{}", ans - ab[mi].0 + ab[mi].1);
}
