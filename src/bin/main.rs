#![allow(unused_imports)]
use ac_library::math;
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::collections::VecDeque;
use std::{cmp, f64::consts::PI, mem::swap, usize};

trait Transpose<'a, Elem, Iter, T>
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
    T: IntoIterator<Item = Iter>,
{
    fn transpose(self) -> Transposed<'a, Elem, Iter>;
}

impl<'a, Elem, Iter, T> Transpose<'a, Elem, Iter, T> for T
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
    T: IntoIterator<Item = Iter>,
{
    fn transpose(self) -> Transposed<'a, Elem, Iter> {
        Transposed {
            iters: self.into_iter().map(IntoIterator::into_iter).collect(),
        }
    }
}

struct Transposed<'a, Elem, Iter>
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
{
    iters: Vec<Iter::IntoIter>,
}

impl<'a, Elem, Iter> Iterator for Transposed<'a, Elem, Iter>
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
{
    type Item = Vec<&'a Elem>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iters.iter_mut().map(Iterator::next).collect()
    }
}

fn main() {
    input! {
        n:usize,
    }
    println!("{}", n)
}
