#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        n:usize,
        k:usize,
        p:[usize;n]
    }
    let mut index: Vec<usize> = Vec::new();
    index.resize(n, 0);

    for (i, &x) in p.iter().enumerate() {
        index[x - 1] = i;
    }
    let mut set = BTreeSet::new();
    for kin in 0..k {
        set.insert(index[kin]);
    }
    let mut ans = set.last().unwrap() - set.first().unwrap();
    for kk in 1..(n - k + 1) {
        set.remove(&index[kk - 1]);
        set.insert(index[kk + k - 1]);
        ans = cmp::min(ans, set.last().unwrap() - set.first().unwrap());
    }

    println!("{}", ans);
}
