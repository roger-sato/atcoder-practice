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
        mut a:[usize;n]
    }
    let mut count = 0;
    a.sort();

    let mut right = n;
    for i in 0..n {
        right = cmp::max(right, i + 1);
        while right - 1 > i && a[right - 1] + a[i] >= 100000000 {
            right -= 1;
        }
        count += n - right
    }

    println!(
        "{}",
        ((n - 1) * a.iter().sum::<usize>()) - 100000000 * count
    );
}
