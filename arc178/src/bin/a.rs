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
        m:usize,
        mut a:[usize;m]
    }
    let f = *a.iter().min().unwrap();
    let l = *a.iter().max().unwrap();
    if f == 1 || l == n {
        println!("{}", -1);
        return;
    }
    let mut ans: Vec<usize> = (1..=n).collect();
    a.sort();
    for &x in a.iter() {
        ans.swap(x - 1, x);
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
