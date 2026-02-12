use serde::Serialize;
use std::ffi::{CStr, CString, c_char};
use std::os::raw::c_int;

/// Word origin classification.
///
/// Returned by `varnavinyas_classify`. Integer-backed for C ABI safety.
#[repr(C)]
pub enum Origin {
    Tatsam = 0,
    Tadbhav = 1,
    Deshaj = 2,
    Aagantuk = 3,
}

/// Transliteration scheme constants for use with `varnavinyas_transliterate`.
///
/// Pass these as `c_int` values; the function validates the discriminant.
pub const SCHEME_DEVANAGARI: c_int = 0;
pub const SCHEME_IAST: c_int = 1;

#[derive(Serialize)]
struct CDiagnostic {
    span_start: u64,
    span_end: u64,
    incorrect: String,
    correction: String,
    rule: String,
    explanation: String,
    category: String,
    category_code: String,
    kind: String,
    confidence: f32,
}

/// Helper: convert a C string pointer to a Rust &str.
/// Returns None on null pointer or invalid UTF-8.
unsafe fn cstr_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

/// Helper: convert a Rust String to an owned C string pointer.
/// The caller must free the returned pointer with `varnavinyas_free_string`.
fn string_to_c(s: String) -> *mut c_char {
    CString::new(s)
        .map(|c| c.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Convert a `c_int` scheme value to the internal `Scheme` type.
/// Returns `None` for out-of-range values.
fn parse_scheme(value: c_int) -> Option<varnavinyas_lipi::Scheme> {
    match value {
        SCHEME_DEVANAGARI => Some(varnavinyas_lipi::Scheme::Devanagari),
        SCHEME_IAST => Some(varnavinyas_lipi::Scheme::Iast),
        _ => None,
    }
}

/// Check text for spelling and punctuation issues.
///
/// Returns a JSON array of diagnostics as a C string.
/// The caller must free the returned pointer with `varnavinyas_free_string`.
/// Returns NULL if `text` is null or not valid UTF-8.
///
/// # Safety
///
/// `text` must be a valid null-terminated C string or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn varnavinyas_check_text(text: *const c_char) -> *mut c_char {
    let Some(text) = (unsafe { cstr_to_str(text) }) else {
        return std::ptr::null_mut();
    };
    let diags = varnavinyas_parikshak::check_text(text);
    let c_diags: Vec<CDiagnostic> = diags
        .into_iter()
        .map(|d| CDiagnostic {
            span_start: d.span.0 as u64,
            span_end: d.span.1 as u64,
            incorrect: d.incorrect,
            correction: d.correction,
            rule: d.rule.to_string(),
            explanation: d.explanation,
            category: d.category.to_string(),
            category_code: d.category.as_code().to_string(),
            kind: d.kind.as_code().to_string(),
            confidence: d.confidence,
        })
        .collect();
    let json = serde_json::to_string(&c_diags).unwrap_or_else(|_| "[]".to_string());
    string_to_c(json)
}

/// Transliterate text between Devanagari and IAST.
///
/// `from` and `to` are scheme constants: `SCHEME_DEVANAGARI` (0) or `SCHEME_IAST` (1).
/// Returns the transliterated text as a C string.
/// The caller must free the returned pointer with `varnavinyas_free_string`.
/// Returns NULL on null input, invalid UTF-8, invalid scheme value, or transliteration error.
///
/// # Safety
///
/// `input` must be a valid null-terminated C string or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn varnavinyas_transliterate(
    input: *const c_char,
    from: c_int,
    to: c_int,
) -> *mut c_char {
    let Some(input) = (unsafe { cstr_to_str(input) }) else {
        return std::ptr::null_mut();
    };
    let (Some(from_scheme), Some(to_scheme)) = (parse_scheme(from), parse_scheme(to)) else {
        return std::ptr::null_mut();
    };
    match varnavinyas_lipi::transliterate(input, from_scheme, to_scheme) {
        Ok(result) => string_to_c(result),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Classify a word by its origin.
///
/// Returns `Origin::Deshaj` on null input or invalid UTF-8.
///
/// # Safety
///
/// `word` must be a valid null-terminated C string or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn varnavinyas_classify(word: *const c_char) -> Origin {
    let Some(word) = (unsafe { cstr_to_str(word) }) else {
        return Origin::Deshaj;
    };
    match varnavinyas_shabda::classify(word) {
        varnavinyas_shabda::Origin::Tatsam => Origin::Tatsam,
        varnavinyas_shabda::Origin::Tadbhav => Origin::Tadbhav,
        varnavinyas_shabda::Origin::Deshaj => Origin::Deshaj,
        varnavinyas_shabda::Origin::Aagantuk => Origin::Aagantuk,
    }
}

/// Free a string previously returned by a varnavinyas function.
///
/// Must be called on every non-NULL string returned by this library.
/// Passing NULL is a no-op.
///
/// # Safety
///
/// `ptr` must be a pointer previously returned by a varnavinyas function, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn varnavinyas_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(unsafe { CString::from_raw(ptr) });
    }
}

/// Get the library version string.
///
/// The caller must free the returned pointer with `varnavinyas_free_string`.
#[unsafe(no_mangle)]
pub extern "C" fn varnavinyas_version() -> *mut c_char {
    string_to_c(env!("CARGO_PKG_VERSION").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn check_text_returns_valid_json() {
        let input = CString::new("नेपाल").unwrap();
        unsafe {
            let result = varnavinyas_check_text(input.as_ptr());
            assert!(!result.is_null());
            let s = CStr::from_ptr(result).to_str().unwrap();
            let parsed: serde_json::Value = serde_json::from_str(s).unwrap();
            assert!(parsed.is_array());
            varnavinyas_free_string(result);
        }
    }

    #[test]
    fn check_text_null_returns_null() {
        unsafe {
            let result = varnavinyas_check_text(std::ptr::null());
            assert!(result.is_null());
        }
    }

    #[test]
    fn transliterate_works() {
        let input = CString::new("नमस्ते").unwrap();
        unsafe {
            let result = varnavinyas_transliterate(input.as_ptr(), SCHEME_DEVANAGARI, SCHEME_IAST);
            assert!(!result.is_null());
            let s = CStr::from_ptr(result).to_str().unwrap();
            assert!(!s.is_empty());
            varnavinyas_free_string(result);
        }
    }

    #[test]
    fn transliterate_null_returns_null() {
        unsafe {
            let result =
                varnavinyas_transliterate(std::ptr::null(), SCHEME_DEVANAGARI, SCHEME_IAST);
            assert!(result.is_null());
        }
    }

    #[test]
    fn transliterate_invalid_scheme_returns_null() {
        let input = CString::new("नमस्ते").unwrap();
        unsafe {
            // Invalid from scheme
            let result = varnavinyas_transliterate(input.as_ptr(), 99, SCHEME_IAST);
            assert!(result.is_null());
            // Invalid to scheme
            let result = varnavinyas_transliterate(input.as_ptr(), SCHEME_DEVANAGARI, -1);
            assert!(result.is_null());
        }
    }

    #[test]
    fn classify_returns_valid_origin() {
        let input = CString::new("नेपाल").unwrap();
        unsafe {
            let origin = varnavinyas_classify(input.as_ptr());
            let val = origin as i32;
            assert!((0..=3).contains(&val));
        }
    }

    #[test]
    fn classify_null_returns_deshaj() {
        unsafe {
            let origin = varnavinyas_classify(std::ptr::null());
            assert_eq!(origin as i32, Origin::Deshaj as i32);
        }
    }

    #[test]
    fn free_string_null_is_noop() {
        unsafe {
            varnavinyas_free_string(std::ptr::null_mut());
        }
    }

    #[test]
    fn version_returns_valid_string() {
        let result = varnavinyas_version();
        assert!(!result.is_null());
        unsafe {
            let s = CStr::from_ptr(result).to_str().unwrap();
            assert!(s.contains("0.1.0"));
            varnavinyas_free_string(result);
        }
    }
}
