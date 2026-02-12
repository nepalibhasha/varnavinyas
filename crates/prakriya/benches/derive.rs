use criterion::{Criterion, black_box, criterion_group, criterion_main};
use varnavinyas_prakriya::derive;

fn bench_derive_correct_word(c: &mut Criterion) {
    c.bench_function("derive_correct_word", |b| {
        b.iter(|| derive(black_box("नेपाल")))
    });
}

fn bench_derive_correction_table(c: &mut Criterion) {
    c.bench_function("derive_correction_table", |b| {
        b.iter(|| derive(black_box("राजनैतिक")))
    });
}

fn bench_derive_hrasva_dirgha(c: &mut Criterion) {
    c.bench_function("derive_hrasva_dirgha", |b| {
        b.iter(|| derive(black_box("दुरी")))
    });
}

criterion_group!(
    benches,
    bench_derive_correct_word,
    bench_derive_correction_table,
    bench_derive_hrasva_dirgha,
);
criterion_main!(benches);
