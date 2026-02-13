/// A rule from an authoritative source.
/// Modeled after Vidyut's Rule enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rule {
    /// Nepal Academy Orthography Standard section reference.
    /// e.g., "3(क)" for hrasva/dirgha vowel rules.
    VarnaVinyasNiyam(&'static str),

    /// Nepal Academy Grammar reference.
    Vyakaran(&'static str),

    /// Specific word table entry from Section 4.
    ShuddhaAshuddha(&'static str),

    /// Punctuation rule from Section 5.
    ChihnaNiyam(&'static str),
}

impl Rule {
    /// Get the rule code.
    pub fn code(&self) -> &'static str {
        match self {
            Rule::VarnaVinyasNiyam(s) => s,
            Rule::Vyakaran(s) => s,
            Rule::ShuddhaAshuddha(s) => s,
            Rule::ChihnaNiyam(s) => s,
        }
    }

    /// Get the source name.
    pub fn source_name(&self) -> &'static str {
        match self {
            Rule::VarnaVinyasNiyam(_) => "वर्णविन्यास नियम",
            Rule::Vyakaran(_) => "व्याकरण",
            Rule::ShuddhaAshuddha(_) => "शुद्ध-अशुद्ध तालिका",
            Rule::ChihnaNiyam(_) => "चिह्न नियम",
        }
    }

    /// Human-readable description of the rule.
    pub fn description(&self) -> &'static str {
        match self {
            Rule::VarnaVinyasNiyam(code) => match *code {
                c if c.starts_with("3(क)") => "ह्रस्व/दीर्घ स्वर नियम",
                c if c.starts_with("3(ख)") => "चन्द्रबिन्दु/शिरबिन्दु नियम",
                c if c.starts_with("3(ग)") => "श/ष/स प्रयोग नियम",
                c if c.starts_with("3(घ)") => "ऋ/कृ प्रयोग नियम",
                c if c.starts_with("3(ङ)") => "हलन्त नियम",
                c if c.starts_with("3(इ)") => "य/ए भेद नियम",
                c if c.starts_with("3(उ)") => "क्ष/छ भेद नियम",
                "3(ई)" => "शुद्ध-अशुद्ध शब्द सूची",
                _ => "वर्णविन्यास नियम",
            },
            Rule::Vyakaran(_) => "व्याकरण नियम",
            Rule::ShuddhaAshuddha(_) => "शुद्ध-अशुद्ध शब्द सूची",
            Rule::ChihnaNiyam(_) => "विराम चिह्न नियम",
        }
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
