#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        a:i64,
        m:i64,
        mut l:i64,
        mut r:i64,
    }
    l -= a;
    r -= a;
    let li = (l - 1).div_euclid(m);
    let ri = r.div_euclid(m);
    println!("{}", ri - li);
}
