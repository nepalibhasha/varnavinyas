use criterion::{Criterion, black_box, criterion_group, criterion_main};
use varnavinyas_kosha::kosha;

fn bench_kosha_contains_hit(c: &mut Criterion) {
    let k = kosha();
    c.bench_function("kosha_contains_hit", |b| {
        b.iter(|| k.contains(black_box("नेपाल")))
    });
}

fn bench_kosha_contains_miss(c: &mut Criterion) {
    let k = kosha();
    c.bench_function("kosha_contains_miss", |b| {
        b.iter(|| k.contains(black_box("ज्ञानप्रकाशमय")))
    });
}

criterion_group!(benches, bench_kosha_contains_hit, bench_kosha_contains_miss,);
criterion_main!(benches);
