use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ray_tracer::canvas::Canvas;

fn ppm_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("arb");
    group.sample_size(40);
    let canvas = black_box(Canvas::new(80, 50));
    group.bench_function("PPM algorithm", |b| b.iter(|| canvas.to_ppm()));
    group.finish();
}

criterion_group!(benches, ppm_benchmark);
criterion_main!(benches);
