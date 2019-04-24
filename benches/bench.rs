#![feature(test)]

extern crate cichlid;
extern crate test;

use test::{Bencher,black_box};
use cichlid::{ColorRGB, prelude::*};


fn rand_change(seed: &mut u64) -> u64 {
    *seed ^= *seed >> 12;
    *seed ^= *seed << 25;
    *seed ^= *seed >> 27;
    seed.wrapping_mul(2685_8216_5773_6338_717)
}

fn create_rand_rgb_vec(rng: &mut u64, amt: usize) -> Vec<ColorRGB> {
    (0..).take(amt)
         .map(|_| ColorRGB::from_color_code(rand_change(rng) as u32))
         .collect()
}

#[bench]
fn bench_fade_large_batch(b: &mut Bencher) {
    let f = |slice: &mut [ColorRGB], fade: u8| {slice.fade_to_black(fade)};
    bench_fade_large(b, f);
}

#[bench]
fn bench_fade_large_scale(b: &mut Bencher) {
    let f = |slice: &mut [ColorRGB], fade: u8| {slice.iter_mut().for_each(|p| p.scale(fade))};
    bench_fade_large(b, f);
}

#[bench]
fn bench_fade_small_batch(b: &mut Bencher) {
    let f = |slice: &mut [ColorRGB], fade: u8| {slice.fade_to_black(fade)};
    bench_fade_small(b, f);
}

#[bench]
fn bench_fade_small_scale(b: &mut Bencher) {
    let f = |slice: &mut [ColorRGB], fade: u8| {slice.iter_mut().for_each(|p| p.scale(fade))};
    bench_fade_small(b, f);
}

fn bench_fade_large<F: FnMut(&mut [ColorRGB], u8)>(b: &mut Bencher, f: F) {
    let mut seed = 105554620937;
    let mut strips: Vec<Vec<ColorRGB>> = (10..)
        .step_by(19)
        .take(24)
        .map(|amt| create_rand_rgb_vec(&mut seed, amt))
        .collect();
    inner_bench_fade_over_vec(b, &mut strips, f);
}

fn bench_fade_small<F: FnMut(&mut [ColorRGB], u8)>(b: &mut Bencher, f: F) {
    let mut seed = 500184610019991;
    let mut strips: Vec<Vec<ColorRGB>> = (2..)
        .take(31)
        .map(|amt| create_rand_rgb_vec(&mut seed, (amt % 8) + (amt + 1 / 2)))
        .collect();
    inner_bench_fade_over_vec(b, &mut strips, f);
}


fn inner_bench_fade_over_vec<F>(b: &mut Bencher, strips: &mut Vec<Vec<ColorRGB>>, mut f: F)
    where F: FnMut(&mut [ColorRGB], u8) {
    b.iter(|| {
        let mut t = strips.clone();
        for fade in 0..=255 {
            for s in t.iter_mut() {
                let slice: &mut [ColorRGB] = s.as_mut();
                black_box(f(slice, 255 - fade));
            }
        }
    })
}