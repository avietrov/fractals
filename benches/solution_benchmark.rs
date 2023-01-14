use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use newton_factal::math::complex::Complex;
use newton_factal::math::polynomial::Polynomial;
use newton_factal::{newton_method_field, Field, newton_method_approximate};
use rand::prelude::*;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group_a = c.benchmark_group("solve point");
    for pol_order in 2..=7 {
        group_a.bench_with_input(BenchmarkId::from_parameter(pol_order), &pol_order, |b, &pol_order| {
            b.iter(|| {
                let mut rng = rand::thread_rng();
                let coef = (0..pol_order).map(|_| rng.gen()).collect();
                let pol = Polynomial::new(coef);
                let dpol = pol.derivative();
                let point = Complex { re: 4., im: 2. };
                newton_method_approximate(black_box(&pol), black_box(&dpol), black_box(&point), 100);
            })
        });
    }
    group_a.finish();

    let mut group_f = c.benchmark_group("solve field");
    group_f
        .measurement_time(Duration::from_secs(10));
    for grid in [16, 32, 64, 128, 256] {
        group_f.bench_with_input(BenchmarkId::from_parameter(grid), &grid, |b, &grid| {
            b.iter(|| {
                // y = 2x^5 - 3x^4 + 7x^3 + 5x + 3
                let pol = Polynomial::new(vec![3, 5, 0, 7, -3, 2]);
                let field = Field {
                    source: Complex { re: -5., im: -5. },
                    size: 10.0,
                    grid: grid,
                };
                newton_method_field(black_box(pol), black_box(field), 100);
            })
        });
    }
    group_f.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
