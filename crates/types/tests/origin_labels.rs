use varnavinyas_types::Origin;

#[test]
fn origin_labels_are_bilingual_and_stable() {
    assert_eq!(Origin::Tatsam.nepali_label(), "तत्सम");
    assert_eq!(Origin::Tadbhav.nepali_label(), "तद्भव");
    assert_eq!(Origin::Deshaj.nepali_label(), "देशज");
    assert_eq!(Origin::Aagantuk.nepali_label(), "आगन्तुक");

    assert_eq!(Origin::Tatsam.transliterated_label(), "tatsam");
    assert_eq!(Origin::Tadbhav.transliterated_label(), "tadbhav");
    assert_eq!(Origin::Deshaj.transliterated_label(), "deshaj");
    assert_eq!(Origin::Aagantuk.transliterated_label(), "aagantuk");

    assert_eq!(Origin::Tatsam.bilingual_label(), "तत्सम (tatsam)");
    assert_eq!(Origin::Tadbhav.bilingual_label(), "तद्भव (tadbhav)");
    assert_eq!(Origin::Deshaj.bilingual_label(), "देशज (deshaj)");
    assert_eq!(Origin::Aagantuk.bilingual_label(), "आगन्तुक (aagantuk)");
}
