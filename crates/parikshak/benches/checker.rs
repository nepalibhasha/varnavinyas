use criterion::{Criterion, black_box, criterion_group, criterion_main};
use varnavinyas_parikshak::{check_text, check_word};

fn bench_check_word(c: &mut Criterion) {
    c.bench_function("check_word", |b| {
        b.iter(|| check_word(black_box("राजनैतिक")))
    });
}

fn bench_check_text_1k(c: &mut Criterion) {
    // Build a ~1000-word paragraph by repeating a sentence
    let sentence = "नेपाल एक सुन्दर देश हो। यहाँको प्राकृतिक सुन्दरता अतुलनीय छ। ";
    let paragraph = sentence.repeat(100); // ~1000 words
    c.bench_function("check_text_1k_words", |b| {
        b.iter(|| check_text(black_box(&paragraph)))
    });
}

criterion_group!(benches, bench_check_word, bench_check_text_1k,);
criterion_main!(benches);
