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
        a:usize,
        b:usize
    }
    let ans = sum_up(n, 1) - sum_up(n, a) - sum_up(n, b) + sum_up(n, a * b / gcd(a, b));

    println!("{}", ans);
}

fn sum_up(n: usize, d: usize) -> usize {
    let nd: usize = (n as f64 / d as f64).floor() as usize;
    nd * (d + nd * d) / 2
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
