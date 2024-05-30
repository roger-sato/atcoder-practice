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
        q:usize,
        mut x:[usize;q]
    }
    let mut ans: Vec<usize> = (0..n).collect();
    let mut index: Vec<usize> = (0..n).collect();
    for i in 0..q {
        let x = x[i] - 1;
        let j = index[x];
        let nj = if j + 1 == n { j - 1 } else { j + 1 };
        let v = ans[nj];
        index[x] = nj;
        index[v] = j;
        ans.swap(j, nj)
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
