#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        ss: Chars,
    }

    let s: Vec<char> = ss
        .iter()
        .rev()
        .map(|&x| if x == '0' { 'A' } else { 'B' })
        .collect();

    let mut state = 'A';
    let mut ans = Vec::new();
    let mut sum = 0;

    for (i, &x) in s.iter().enumerate() {
        if x != state {
            ans.push((state, n - i));
            sum += n - i;
            state = if state == 'A' { 'B' } else { 'A' };
        }
    }

    println!("{}", sum);
    for &(c, x) in ans.iter() {
        print!("{}", c.to_string().repeat(x));
    }
}
