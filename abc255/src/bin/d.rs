#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        n:usize,
        q:usize,
        mut a: [usize;n],
        x: [usize;q]
    }
    a.sort();
    let mut s = vec![0];
    for x in a.iter() {
        s.push(x + s[s.len() - 1])
    }
    for xx in x {
        let i = a
            .binary_search_by(|x| match x.cmp(&xx) {
                Ordering::Equal => Ordering::Greater,
                ord => ord,
            })
            .unwrap_err();
        let left = i * xx - s[i];
        let right = s[n] - s[i] - (n - i) * xx;
        let ans = left + right;
        println!("{}", ans);
    }
}
