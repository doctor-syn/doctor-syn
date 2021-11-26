use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::{Distribution, Uniform};

mod doctor_syn {
    pub fn sin(x: f32) -> f32 {
        let x = x * (1.0 / (std::f32::consts::PI * 2.0));
        let x = x - x.round();
        (-f32::from_bits(1058770289u32))
            .mul_add(x * x, f32::from_bits(1081164794u32))
            .mul_add(x * x, -f32::from_bits(1097945697u32))
            .mul_add(x * x, f32::from_bits(1109932662u32))
            .mul_add(x * x, -f32::from_bits(1117350231u32))
            .mul_add(x * x, f32::from_bits(1117992419u32))
            .mul_add(x * x, -f32::from_bits(1109745127u32))
            .mul_add(x * x, f32::from_bits(1086918619u32))
            * x
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn f32_sin(input: &[f32], output: &mut [f32]) {
    input
        .iter()
        .zip(output.iter_mut())
        .for_each(|(i, o)| *o = f32::sin(*i));
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn our_sin(input: &[f32], output: &mut [f32]) {
    input
        .iter()
        .zip(output.iter_mut())
        .for_each(|(i, o)| *o = doctor_syn::sin(*i));
}

fn bench_sin(c: &mut Criterion) {
    let between = Uniform::from(-10.0..10.0);
    let mut rng = rand::thread_rng();
    let input: Vec<_> = (0..10240).map(|_| between.sample(&mut rng)).collect();
    let mut output = vec![0_f32; 64];

    unsafe {
        c.bench_function("std sin 64", |b| b.iter(|| f32_sin(&*input, &mut *output)));
        c.bench_function("our sin 64", |b| b.iter(|| our_sin(&*input, &mut *output)));
    }
}

criterion_group!(benches, bench_sin);
criterion_main!(benches);
