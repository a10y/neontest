use divan::Bencher;
use multiversion::multiversion;

#[divan::bench]
fn scalar(bencher: Bencher) {
    let values = vec![f32::NAN; 1000];
    bencher.bench_local(|| nancount_scalar(&values));
}

#[divan::bench]
fn multiversion(bencher: Bencher) {
    let values = vec![f32::NAN; 1000];
    bencher.bench_local(|| nancount_vector(&values));
}

#[divan::bench]
fn neon(bencher: Bencher) {
    let values = vec![f32::NAN; 1000];
    bencher.bench_local(|| nancount_arch(&values));
}

fn nancount_scalar(values: &[f32]) -> usize {
    values.iter().filter(|f| f.is_nan()).count()
}

#[multiversion(targets("aarch64+neon"))]
fn nancount_vector(values: &[f32]) -> usize {
    values.iter().filter(|f| f.is_nan()).count()
}

#[inline(always)]
fn nancount_arch(values: &[f32]) -> usize {
    use std::arch::aarch64::*;

    let mut count = 0;
    let mut chunks = values.chunks_exact(4);
    for chunk in &mut chunks {
        // check if a block are NAN by checking if equal to self
        let v = unsafe { vld1q_f32(chunk.as_ptr()) };

        // Compare with self, false slots indicate NaN
        let cmp = unsafe { vceqq_f32(v, v) };

        // reinterpret the false positions as true
        let mask = unsafe { vmvnq_u32(cmp) };
        let ones = unsafe { vshrq_n_u32(mask, 31) };
        count += unsafe { vaddvq_u32(ones) };

        // // Doesn't work, we are adding with tons of overflow here...I think
        // count += (unsafe { vgetq_lane_u32::<0>(mask) } == 0) as u32;
        // count += (unsafe { vgetq_lane_u32::<1>(mask) } == 0) as u32;
        // count += (unsafe { vgetq_lane_u32::<2>(mask) } == 0) as u32;
        // count += (unsafe { vgetq_lane_u32::<3>(mask) } == 0) as u32;
    }

    count += chunks.remainder().iter().filter(|f| f.is_nan()).count() as u32;

    count as usize
}

// divan boilerplate
pub fn main() {
    assert!(std::arch::is_aarch64_feature_detected!("neon"));
    divan::main();
}
