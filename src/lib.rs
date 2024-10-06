#![feature(portable_simd)]
mod multi_sfmt;

use crate::multi_sfmt::MultiSFMT;
use multiversion::multiversion;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use smallvec::SmallVec;
use std::simd::{u64x8, Simd};

#[inline]
fn encode_needles(needles: Vec<u64>) -> u64 {
    assert!(needles.len() == 12 && needles.iter().all(|&needle| needle <= 16));

    let mut needle = 0;
    for i in 0..needles.len() {
        needle |= needles[i] << (5 * i);
    }
    needle
}

#[inline]
fn find_needles(sfmt: &mut MultiSFMT) -> u64x8 {
    let mut needles = Simd::splat(0);
    for i in 0..12 {
        needles |= sfmt.next_needle() << Simd::splat(i * 5);
    }
    needles
}

#[multiversion(targets = "simd")]
fn find_seed_simd(seed_hi: u32, needles: u64, offset: u32) -> SmallVec<[u32; 1]> {
    let mut results = SmallVec::new();
    let mut sfmt = MultiSFMT::default();

    let seed_begin = seed_hi << 16;
    let seed_end = (seed_hi + 1) << 16;
    for s in (seed_begin..seed_end).step_by(8) {
        let seed = Simd::from_array([s, s | 1, s | 2, s | 3, s | 4, s | 5, s | 6, s | 7]);
        sfmt.init(seed);
        sfmt.advance(offset as usize);

        let f = find_needles(&mut sfmt);
        for i in 0..8 {
            if f[i] == needles {
                results.push(s | (i as u32))
            }
        }
    }
    results
}

pub fn find_seed(
    seed_hi_range: (u32, u32), // right-open
    needles: Vec<u64>,
    offset: u32,
    notify_progress: impl Fn(&[u32], u32) -> () + Sync,
) -> Vec<u32> {
    let (seed_hi_begin, seed_hi_end) = seed_hi_range;
    assert!(seed_hi_begin < seed_hi_end && seed_hi_end <= 0x10000);

    assert!(needles.len() == 12 && needles.iter().all(|&needle| needle <= 16));

    let needles = encode_needles(needles);

    let length = seed_hi_end - seed_hi_begin;

    (seed_hi_begin..seed_hi_end)
        .into_par_iter()
        .flat_map_iter(|seed_hi| {
            let hits = find_seed_simd(seed_hi, needles, offset);
            notify_progress(&hits, length);
            hits
        })
        .collect()
}
