/// शब्दउत्पत्ति वर्गीकरण।
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Origin {
    /// तत्सम — direct Sanskrit borrowing (मूल रूप कायम).
    Tatsam,
    /// तद्भव — नेपाली ध्वनि-नियमअनुसार रूपान्तरित संस्कृत शब्द।
    Tadbhav,
    /// देशज — मूल नेपाली शब्द।
    Deshaj,
    /// आगन्तुक — बाह्य भाषाबाट आएका शब्द (जस्तै: English, Arabic, Hindi)।
    Aagantuk,
}

impl Origin {
    /// उत्पत्ति वर्गको मानक नेपाली नाम।
    pub const fn nepali_label(self) -> &'static str {
        match self {
            Origin::Tatsam => "तत्सम",
            Origin::Tadbhav => "तद्भव",
            Origin::Deshaj => "देशज",
            Origin::Aagantuk => "आगन्तुक",
        }
    }

    /// उत्पत्ति वर्गको मानक transliterated नाम।
    pub const fn transliterated_label(self) -> &'static str {
        match self {
            Origin::Tatsam => "tatsam",
            Origin::Tadbhav => "tadbhav",
            Origin::Deshaj => "deshaj",
            Origin::Aagantuk => "aagantuk",
        }
    }

    /// द्विभाषी लेबल: नेपाली शब्द + transliteration।
    pub const fn bilingual_label(self) -> &'static str {
        match self {
            Origin::Tatsam => "तत्सम (tatsam)",
            Origin::Tadbhav => "तद्भव (tadbhav)",
            Origin::Deshaj => "देशज (deshaj)",
            Origin::Aagantuk => "आगन्तुक (aagantuk)",
        }
    }
}
