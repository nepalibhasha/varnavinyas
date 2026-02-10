# Rust Guide for Varnavinyas

A Rust onboarding companion for building Varnavinyas. Covers everything you need to know to get started, organized by relevance to this project.

---

## 1. Installing Rust

### rustup (the Rust toolchain manager)

```bash
# Install rustup (macOS/Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version    # Rust compiler
cargo --version    # Package manager & build tool
rustup --version   # Toolchain manager
```

After installation, restart your terminal or run `source $HOME/.cargo/env`.

### VS Code Setup

1. Install [VS Code](https://code.visualstudio.com/)
2. Install the **rust-analyzer** extension (the official Rust language server)
3. Open the `varnavinyas/` folder in VS Code
4. rust-analyzer will automatically index the workspace

**Recommended VS Code settings** for Rust:
```json
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "editor.formatOnSave": true
}
```

### Project-specific toolchain

Varnavinyas pins its Rust version via `rust-toolchain.toml`. When you `cd` into the project directory, rustup automatically uses the pinned version. You don't need to manage versions manually.

---

## 2. Cargo Essentials

Cargo is Rust's build system and package manager. These are the commands you'll use daily.

### Basic Commands

```bash
cargo build                  # Compile the project (debug mode)
cargo build --release        # Compile with optimizations
cargo test                   # Run all tests
cargo run                    # Build and run (for binary crates)
cargo check                  # Type-check without building (faster)
```

### Workspace Commands

Varnavinyas is a **workspace** — multiple crates in one repository. Workspace commands:

```bash
cargo test --workspace           # Test ALL crates
cargo test -p varnavinyas-akshar # Test just the akshar crate
cargo test -p varnavinyas-akshar -- --nocapture  # See println! output
cargo build --workspace          # Build all crates
```

### Quality Commands

```bash
cargo fmt --all              # Auto-format all code
cargo fmt --all --check      # Check formatting (CI uses this)
cargo clippy --workspace     # Lint all code (clippy is your Rust mentor!)
cargo clippy --workspace -- -D warnings  # Treat warnings as errors
cargo doc --workspace --no-deps --open   # Build & open API docs
```

### Using Cargo Aliases

The project defines aliases in `.cargo/config.toml`:

```bash
cargo t     # → cargo test --workspace
cargo c     # → cargo clippy --workspace -- -D warnings
cargo f     # → cargo fmt --all
cargo d     # → cargo doc --workspace --no-deps --open
```

---

## 3. Key Concepts for This Project

### Ownership & Borrowing

Rust's most distinctive feature. Critical for string processing.

```rust
// Ownership: each value has exactly one owner
let s = String::from("नमस्ते");  // s owns this String
let s2 = s;                       // ownership MOVES to s2
// println!("{}", s);              // ERROR: s no longer valid

// Borrowing: temporarily lend access without taking ownership
fn count_chars(text: &str) -> usize {  // &str = borrowed reference
    text.chars().count()
}
let s = String::from("नमस्ते");
let n = count_chars(&s);  // borrow s (& = reference)
println!("{}", s);         // s still valid — we only borrowed it
```

**Why it matters for Varnavinyas**: We process Nepali text constantly. Functions like `classify(c: char)` and `split_aksharas(text: &str)` take borrowed references to avoid copying strings. Return types like `Vec<Akshara>` own their data.

### Enums & Pattern Matching

Used extensively in Varnavinyas for types like `CharType`, `Origin`, `Rule`, `Scheme`.

```rust
// Define an enum with variants
enum SvarType {
    Hrasva,
    Dirgha,
}

// Pattern match to handle each case
fn describe(sv: SvarType) -> &'static str {
    match sv {
        SvarType::Hrasva => "short vowel (ह्रस्व)",
        SvarType::Dirgha => "long vowel (दीर्घ)",
    }
}

// Enums can hold data
enum Rule {
    VarnaVinyasNiyam(&'static str),  // holds a section reference
    ShuddhaAshuddha(&'static str),  // holds a table entry
}
```

**`match` is exhaustive** — the compiler forces you to handle every variant. This prevents bugs where you forget a case.

### Traits

Traits are like interfaces. Key traits used in this project:

```rust
// Display — how to print a type
impl std::fmt::Display for SvarType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SvarType::Hrasva => write!(f, "ह्रस्व"),
            SvarType::Dirgha => write!(f, "दीर्घ"),
        }
    }
}

// From — convert between types
impl From<AksharError> for pyo3::PyErr {
    fn from(err: AksharError) -> pyo3::PyErr {
        pyo3::exceptions::PyValueError::new_err(err.to_string())
    }
}

// thiserror derives Error + Display automatically
#[derive(Debug, thiserror::Error)]
enum AksharError {
    #[error("invalid codepoint: U+{0:04X}")]
    InvalidCodepoint(u32),
}
```

### Modules & Visibility

How Rust organizes code into files:

```rust
// lib.rs — the crate root
mod devanagari;   // loads from devanagari.rs (or devanagari/mod.rs)
mod vowel;
mod consonant;

// Re-export public items
pub use devanagari::{classify, CharType, DevanagariChar};
pub use vowel::{SvarType, svar_type, hrasva_to_dirgha};
```

**Visibility rules**:
- `pub` — public to everyone
- `pub(crate)` — public within this crate only
- (no modifier) — private to the current module

### Testing

Tests live right next to the code they test:

```rust
// In vowel.rs
pub fn is_svar(c: char) -> bool {
    matches!(c, 'अ'..='औ')
}

#[cfg(test)]  // only compiled during testing
mod tests {
    use super::*;

    #[test]
    fn test_svar_vowels() {
        assert!(is_svar('अ'));
        assert!(is_svar('ई'));
        assert!(!is_svar('क'));
    }

    #[test]
    fn test_svar_not_consonant() {
        assert!(!is_svar('क'));
        assert!(!is_svar('A'));
    }
}
```

**Integration tests** go in `tests/` directory alongside `src/`:
```
crates/akshar/
├── src/
│   └── lib.rs
└── tests/
    └── classification.rs   # integration test
```

### Iterators & Closures

Essential for text processing:

```rust
// Iterate over characters in a string
let text = "नमस्ते";
for c in text.chars() {
    println!("{}: U+{:04X}", c, c as u32);
}

// Filter and collect
let vowels: Vec<char> = text.chars()
    .filter(|c| is_svar(*c))
    .collect();

// Map: transform each element
let codes: Vec<u32> = text.chars()
    .map(|c| c as u32)
    .collect();

// Chain operations
let devanagari_count = text.chars()
    .filter(|c| classify(*c).is_some())
    .count();
```

### String Types: `&str` vs `String`

The most important distinction for text processing:

```rust
// &str — a borrowed reference to string data (like a window/view)
// Lightweight, no allocation, can't be modified
let greeting: &str = "नमस्ते";

// String — an owned, heap-allocated, growable string
// Can be modified, costs an allocation
let mut name = String::from("नमस्ते");
name.push_str(" विश्व");

// &str from String (cheap — just borrows)
let borrowed: &str = &name;

// String from &str (costs allocation — copies data)
let owned: String = greeting.to_string();
```

**Rule of thumb**:
- Function parameters: use `&str` (accepts both `String` and `&str`)
- Return values: use `String` if the function creates new data
- Struct fields: use `String` (the struct owns its data)

---

## 4. Development Workflow

### cargo watch (auto-rebuild on save)

```bash
cargo install cargo-watch

# Re-run tests on every file save
cargo watch -x "test -p varnavinyas-akshar"

# Re-check on every save (faster than full build)
cargo watch -x check
```

### Using clippy as Your Rust Mentor

Clippy doesn't just find bugs — it teaches you idiomatic Rust. Run it often:

```bash
cargo clippy --workspace
```

Clippy will suggest:
- More idiomatic patterns (e.g., `if let` instead of `match` with one arm)
- Performance improvements (e.g., unnecessary clones)
- Common mistakes (e.g., comparing floats with `==`)

**Treat clippy warnings as learning opportunities**, not annoyances.

### Reading Compiler Errors

Rust's compiler errors are famously helpful. They often include:

1. **What went wrong** (clear error message)
2. **Where it went wrong** (precise line and column)
3. **Why it went wrong** (explanation)
4. **How to fix it** (suggestion with code)

Example:
```
error[E0382]: use of moved value: `s`
 --> src/main.rs:4:20
  |
2 |     let s = String::from("hello");
  |         - move occurs because `s` has type `String`
3 |     let s2 = s;
  |              - value moved here
4 |     println!("{}", s);
  |                    ^ value used here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s.clone();
  |               ++++++++
```

When you see an error you don't understand, read the full message — the compiler is teaching you.

---

## 5. Recommended Learning Path

Ordered by relevance to building Varnavinyas:

| Priority | Resource | Chapters/Topics | Why |
|----------|----------|----------------|-----|
| 1 | [The Rust Book](https://doc.rust-lang.org/book/) | Ch 1-10 | Core language: variables, types, ownership, structs, enums, modules |
| 2 | The Rust Book | Ch 13 | Iterators & closures — you'll use these for all text processing |
| 3 | The Rust Book | Ch 11 | Testing — you'll write many tests |
| 4 | The Rust Book | Ch 14 | Cargo & crates.io — publishing crates |
| 5 | The Rust Book | Ch 15 | Smart pointers — Box, Rc (useful for tree structures) |
| 6 | [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | Browse as needed | Quick reference for syntax patterns |
| 7 | [std library docs](https://doc.rust-lang.org/std/) | `str`, `String`, `char`, `Vec`, `HashMap` | The APIs you'll use most |

**Don't try to learn everything first**. Read chapters 1-6 of the Rust Book, then start building akshar. Come back to learn more as you need it.

---

## 6. Common Gotchas

### UTF-8 string indexing

Rust strings are UTF-8, not arrays of characters. You can't index by position:

```rust
let s = "नमस्ते";
// s[0]              // ERROR: Rust strings can't be indexed
s.chars().nth(0)     // OK: Some('न')
&s[0..3]             // OK but returns BYTES, not chars — fragile!
```

For Devanagari, always use `.chars()` iteration, never byte indexing.

### char vs grapheme cluster

A Rust `char` is a Unicode scalar value (4 bytes). But Devanagari "characters" as humans perceive them can span multiple `char`s:

```rust
// "की" might be:
// - one char: 'की' (precomposed, rare)
// - two chars: 'क' + 'ी' (common in NFC)
// Conjuncts span even more:
// "ज्ञ" = 'ज' + '्' + 'ञ' = 3 chars, 1 visual unit
```

This is why `split_aksharas()` exists — it groups chars into syllable units. The `unicode-segmentation` crate helps with grapheme cluster boundaries.

### Unicode normalization: NFC vs NFD

The same Devanagari text can be encoded differently:

```rust
// NFC (composed): "की" = U+0915 U+0940 (क + ी-matra)
// NFD (decomposed): different byte sequence, same visual

// ALWAYS normalize to NFC before comparing strings
let a = normalize("की");
let b = normalize("की");  // might be different encoding
assert_eq!(a, b);          // equal after normalization
```

### Orphan rule

You can't implement a foreign trait on a foreign type:

```rust
// This WON'T compile in your crate:
impl std::fmt::Display for Vec<String> { ... }

// Solution: wrap the foreign type in your own (newtype pattern)
struct Aksharas(Vec<Akshara>);
impl std::fmt::Display for Aksharas { ... }
```

### The borrow checker fights

When the borrow checker rejects your code:
1. **Don't fight it** — the compiler is usually right
2. **Clone if needed** — `.clone()` is fine for getting started; optimize later
3. **Restructure** — sometimes the fix is reorganizing which function owns what
4. **Ask clippy** — `cargo clippy` often suggests the fix

---

## 7. Quick Reference

### Useful `char` methods for Devanagari

```rust
let c = 'क';
c as u32                // → 0x0915 (Unicode codepoint)
char::from_u32(0x0915)  // → Some('क')
c.is_alphabetic()       // → true
c.len_utf8()            // → 3 (bytes in UTF-8)
```

### Useful `str` / `String` methods

```rust
let s = "नमस्ते दुनिया";
s.chars()                 // iterator over chars
s.chars().count()         // number of chars (not bytes!)
s.len()                   // byte length (NOT char count)
s.contains("नमस्ते")      // substring search
s.starts_with("नम")       // prefix check
s.split_whitespace()      // split on whitespace
s.trim()                  // remove leading/trailing whitespace
```

### Pattern matching cheat sheet

```rust
// Match on enum
match char_type {
    CharType::Svar => "vowel",
    CharType::Vyanjan => "consonant",
    _ => "other",                     // wildcard for remaining
}

// Match with guards
match c {
    'अ'..='औ' => "vowel range",
    'क'..='ह' => "consonant range",
    _ => "other",
}

// if let (when you only care about one variant)
if let Some(sv) = svar_type(c) {
    println!("vowel type: {:?}", sv);
}
```

---

*This guide covers what you need to start building. For deeper topics, consult [The Rust Book](https://doc.rust-lang.org/book/) and ask the compiler — it's the most helpful error reporter you'll ever encounter.*
