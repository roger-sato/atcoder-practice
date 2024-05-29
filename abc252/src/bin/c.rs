#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::{self, BTreeMap, VecDeque};
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        n:usize,
        s:[String;n]
    }
    let mut ans = std::usize::MAX;
    for y in "0123456789".chars() {
        let mut m = collections::BTreeMap::new();
        for x in s.iter() {
            let f = x.find(y).unwrap();
            *m.entry(f).or_insert(0) += 1;
        }
        let mut lans = 0;
        for (&v, &x) in m.iter() {
            lans = std::cmp::max(lans, v + 10 * (x - 1));
        }
        ans = std::cmp::min(ans, lans);
    }
    println!("{ans}");
}
