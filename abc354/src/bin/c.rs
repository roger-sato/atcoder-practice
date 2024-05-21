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
        mut x:[(usize,usize);n]
    }
    let mut xx: Vec<(usize, &(usize, usize))> = x.iter().enumerate().collect();
    xx.sort_by_key(|&(_, (_, x))| x);
    let mut ans: Vec<(usize, usize)> = vec![];

    for &(i, &(a, _)) in xx.iter() {
        if ans.last().unwrap_or(&(0 as usize, 0 as usize)).1 < a {
            ans.push((i + 1, a))
        }
    }
    ans.sort_by_key(|x| x.0);

    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|x| x.0.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
