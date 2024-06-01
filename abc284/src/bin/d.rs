#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        q:usize,
    }
    for _ in 0..q {
        input! {
            n:usize,
        }
        for i in 2..((n as f64).powf(1.0 / 3.0).ceil() as usize + 100000) {
            if n % i == 0 {
                if (n / i) % i == 0 {
                    println!("{} {}", i, n / (i * i));
                    break;
                } else {
                    println!("{} {}", ((n / i) as f64).sqrt() as usize, i);
                    break;
                }
            }
        }
    }
}
