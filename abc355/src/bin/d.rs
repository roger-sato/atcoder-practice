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
        x:[(usize,usize);n]
    }
    let mut ll: Vec<usize> = x.iter().map(|s| s.0).collect();
    let mut rr: Vec<usize> = x.iter().map(|s| s.1).collect();

    ll.sort();
    rr.sort();
    let mut ans = 0;
    let mut j = 0;
    for i in 0..n {
        while rr[j] < ll[i] {
            j += 1
        }
        ans += j;
    }
    println!("{}", n * (n - 1) / 2 - ans);
}
