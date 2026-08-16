use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn bench_render(c: &mut Criterion) {
    c.bench_function("render", |b| b.iter(|| black_box(1 + 1)));
}
criterion_group!(benches, bench_render);
criterion_main!(benches);
