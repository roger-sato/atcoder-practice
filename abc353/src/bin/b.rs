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
        mut a:[usize;n],
    }
    let mut ans = 0;
    let mut occupy = 0;
    a.reverse();
    while a.len() > 0 {
        ans += 1;
        while occupy + a.last().unwrap_or(&(k + 1)) <= k {
            occupy += a.pop().unwrap();
        }
        occupy = 0;
    }

    println!("{}", ans);
}
