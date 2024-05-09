#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let mut ans = vec![];
    let mut i = 0;
    for x in s {
        while x != t[i] {
            i += 1
        }
        ans.push(i + 1);
        i += 1;
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
