#![feature(test)]

mod lib;

use lib::{anagrams_for};

extern crate test;

use test::Bencher;


#[bench]
fn bench_anagrams_for(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    let w = "АБВ";
    let a = ["АБВ", "гва", "ЭЮЯ", "", "А", "АБВ"];
    b.iter(|| {
        anagrams_for(w, &a);
    })
}
