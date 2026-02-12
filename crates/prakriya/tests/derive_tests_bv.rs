use varnavinyas_prakriya::derive;

// O7.2: B/V distinction (Tatsam uses व)
#[test]
fn o7_bv_vidya() {
    let p = derive("बिद्या");
    assert_eq!(p.output, "विद्या");
}

#[test]
fn o7_bv_vidvaan() {
    // Checks both B/V and Halanta!
    let p = derive("बिद्वान");
    assert_eq!(p.output, "विद्वान्");
}

#[test]
fn o7_bv_bidesh() {
    let p = derive("बिदेश");
    assert_eq!(p.output, "विदेश");
}
