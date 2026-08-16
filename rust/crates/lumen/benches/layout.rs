use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn bench_flex(c: &mut Criterion) {
    c.bench_function("flex", |b| b.iter(|| black_box(1 + 1)));
}
criterion_group!(benches, bench_flex);
criterion_main!(benches);
