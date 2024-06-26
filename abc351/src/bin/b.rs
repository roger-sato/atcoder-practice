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
        a:[Chars;n],
        b:[Chars;n]
    }
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                println!("{}", [i + 1, j + 1].map(|x| x.to_string()).join(" "));
                return;
            }
        }
    }
}
