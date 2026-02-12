# Benchmarks

Baseline recorded on 2026-02-11. Apple Silicon (local dev machine), `cargo bench` with default criterion settings.

## Results

| Benchmark | Median | Description |
|-----------|--------|-------------|
| `kosha_contains_hit` | 145 ns | FST lookup for a word that exists |
| `kosha_contains_miss` | 251 ns | FST lookup for a word that doesn't exist |
| `derive_correction_table` | 320 ns | Derive a word resolved by correction table |
| `check_word` | 418 ns | Full pipeline: derive + kosha for one word |
| `derive_correct_word` | 3.38 us | Derive a word that needs no correction |
| `derive_hrasva_dirgha` | 3.90 us | Derive a word resolved by hrasva/dirgha rules |
| `check_text_1k_words` | 2.96 ms | Full pipeline for ~1000-word paragraph |

## How to Run

```bash
cargo bench --workspace           # all benchmarks
cargo bench -p varnavinyas-kosha  # single crate
```

HTML reports are generated in `target/criterion/`.
