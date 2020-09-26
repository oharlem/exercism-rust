#![feature(test)]

mod lib;

use lib::{encode};

extern crate test;

use test::Bencher;


#[bench]
fn bench_find_saddle_point_functional(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    let i = 18_446_744_073_709_551_615;
    b.iter(|| {
        encode(i);
    })
}
