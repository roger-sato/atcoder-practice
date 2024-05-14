#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        mut e: [usize; 6],
        n: usize,
        x: [usize; n]
    }

    let coin = [500, 100, 50, 10, 5, 1];
    e.reverse();

    if x.iter().all(|&xx| {
        let mut remaining = xx;
        for (i, &c) in coin.iter().enumerate() {
            let count = cmp::min(e[i], remaining / c);
            remaining -= count * c;
            e[i] -= count;
        }
        remaining == 0
    }) {
        println!("Yes");
    } else {
        println!("No");
    }
}
