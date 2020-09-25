#![feature(test)]

mod lib;

use lib::{check, check_functional};

extern crate test;

use test::Bencher;


// function to benchmark must be annotated with `#[bench]`
#[bench]
fn bench_find_saddle_point(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    let i = "this is a test";
    b.iter(|| {
        check(i);
    })
}

#[bench]
fn bench_find_saddle_point_functional(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    let i = "this is a test";
    b.iter(|| {
        check_functional(i);
    })
}
