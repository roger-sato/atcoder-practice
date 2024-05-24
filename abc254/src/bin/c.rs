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
        k:usize,
        mut a:[usize;n]
    }
    let mut b: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..n {
        b[i % k].push(a[i]);
    }
    for x in b.iter_mut() {
        x.sort();
    }
    a.sort();
    let mut ans = true;

    for i in 0..n {
        if a[i] != b[i % k][i / k] {
            ans = false;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
