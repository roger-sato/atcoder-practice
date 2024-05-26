#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::{cmp, f64::consts::PI, mem::swap, usize};

const MOD: usize = 1000000007;

fn main() {
    input! {
        q:usize,
    }
    let mut set: BTreeMap<usize, usize> = BTreeMap::new();
    for _ in 0..q {
        input! {
            x:u8
        }
        if x == 1 {
            input! {
                key:usize
            }
            let cnt = set.get(&key).unwrap_or(&0);
            set.insert(key, cnt + 1);
        } else if x == 2 {
            input! {
                key:usize,
                a:usize
            }
            let cnt = set.get(&key).unwrap_or(&0);
            if cnt <= &a {
                set.remove(&key);
            } else {
                set.insert(key, cnt - a);
            }
        } else {
            println!(
                "{}",
                set.last_key_value().unwrap().0 - set.first_key_value().unwrap().0
            );
        }
    }
}
