#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    if a == b {
        println!("-1");
        return;
    }
    let ans = vec![1, 2, 3];
    for &x in ans.iter() {
        if x != a && x != b {
            println!("{}", x)
        }
    }
}
