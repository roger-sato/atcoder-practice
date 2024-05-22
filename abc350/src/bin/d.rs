#![allow(unused_imports)]
use ac_library::dsu;
use ac_library::math;
use ac_library::Dsu;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        n:usize,
        m:usize,
        ee:[(usize,usize);m]
    }
    let mut uf = Dsu::new(n + 1);
    for &(u, v) in ee.iter() {
        uf.merge(u, v);
    }
    let mut ans = 0;
    for x in 1..=n {
        if uf.leader(x) == x {
            let s = uf.size(x);
            ans += (s) * (s - 1) / 2;
        }
    }

    println!("{}", ans - m);
}
