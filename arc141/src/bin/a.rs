#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        input! {
            n:String
        }
        let mut m = 11;
        for i in 2..=n.len() {
            if n.len() % i != 0 {
                continue;
            }
            let (a, _) = n.split_at(n.len() / i);
            let mut x = a
                .parse::<usize>()
                .unwrap()
                .to_string()
                .repeat(i)
                .parse::<usize>()
                .unwrap();

            if n.parse::<usize>().unwrap() < x {
                x = (a.parse::<usize>().unwrap() - 1)
                    .to_string()
                    .repeat(i)
                    .parse::<usize>()
                    .unwrap();
            }

            m = cmp::max(
                m,
                cmp::max(x, "9".repeat(n.len() - 1).parse::<usize>().unwrap()),
            )
        }
        println!("{}", m)
    }
}
