#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::{iproduct, Itertools};
use petgraph::{algo::is_isomorphic, graph::UnGraph};
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }
    let mut min = 10000000;
    let mut count = 0;
    for i in 0..n {
        if p[i] <= min {
            count += 1;
            min = p[i]
        }
    }
    println!("{}", count);
}
