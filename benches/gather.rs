use divan::Bencher;

#[divan::bench(sample_count = 1000)]
pub fn bench_scalar(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            (
                (0..5).cycle().take(512).collect::<Vec<u32>>(),
                (0..512).collect::<Vec<u32>>(),
                Vec::with_capacity(512),
            )
        })
        .bench_values(|(indices, values, result)| {
            gather_scalar(&indices, &values, result);
        });
}

#[divan::bench(sample_count = 1000)]
pub fn bench_scalar_unrolled(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            (
                (0..5).cycle().take(512).collect::<Vec<u32>>(),
                (0..512).collect::<Vec<u32>>(),
                Vec::with_capacity(512),
            )
        })
        .bench_values(|(indices, values, result)| {
            gather_scalar_unrolled(&indices, &values, result);
        });
}

#[divan::bench(sample_count = 1000)]
pub fn bench_neon(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            (
                (0..5).cycle().take(512).collect::<Vec<u32>>(),
                (0..1024).collect::<Vec<u32>>(),
                Vec::with_capacity(512),
            )
        })
        .bench_values(|(indices, values, result)| {
            gather_neon(&indices, &values, result);
        });
}

// Do the gather operation literally.
fn gather_scalar(indices: &[u32], values: &[u32], mut result: Vec<u32>) {
    indices
        .into_iter()
        .for_each(|&idx| result.push(values[idx as usize]))
}

// Do the gather operation literally.
fn gather_scalar_unrolled(indices: &[u32], values: &[u32], mut result: Vec<u32>) {
    let uninit = result.spare_capacity_mut();
    let mut chunks = indices.chunks_exact(4);
    let mut offset = 0;
    for chunk in &mut chunks {
        uninit[offset].write(values[chunk[0] as usize]);
        uninit[offset + 1].write(values[chunk[1] as usize]);
        uninit[offset + 2].write(values[chunk[2] as usize]);
        uninit[offset + 3].write(values[chunk[3] as usize]);
        offset += 4;
    }

    for &idx in chunks.remainder() {
        uninit[offset].write(values[idx as usize]);
        offset += 1;
    }
    unsafe { result.set_len(indices.len()) };
}

fn gather_neon(indices: &[u32], values: &[u32], mut result: Vec<u32>) {
    use std::arch::aarch64::*;

    let uninit = result.spare_capacity_mut();

    // Read the indices 4 at a time using SIMD read instr.
    let mut output = 0;
    let mut chunks = indices.chunks_exact(4);
    for chunk in &mut chunks {
        // Vectorize the load
        let indices_vec = unsafe { vld1q_u32(chunk.as_ptr()) };
        let idx0 = unsafe { vgetq_lane_u32::<0>(indices_vec) };
        let idx1 = unsafe { vgetq_lane_u32::<1>(indices_vec) };
        let idx2 = unsafe { vgetq_lane_u32::<2>(indices_vec) };
        let idx3 = unsafe { vgetq_lane_u32::<3>(indices_vec) };

        uninit[output].write(values[idx0 as usize]);
        uninit[output + 1].write(values[idx1 as usize]);
        uninit[output + 2].write(values[idx2 as usize]);
        uninit[output + 3].write(values[idx3 as usize]);

        output += 4;
    }

    for &idx in chunks.remainder() {
        uninit[output].write(values[idx as usize]);
        output += 1;
    }

    unsafe {
        result.set_len(indices.len());
    }
}

pub fn main() {
    divan::main();
}
