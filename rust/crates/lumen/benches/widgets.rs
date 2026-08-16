use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn bench_widgets(c: &mut Criterion) {
    c.bench_function("widgets", |b| b.iter(|| black_box(1 + 1)));
}
criterion_group!(benches, bench_widgets);
criterion_main!(benches);
