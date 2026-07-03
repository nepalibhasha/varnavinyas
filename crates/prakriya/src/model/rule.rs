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
                c if c.starts_with("3(ग)(अ)") => "श/ष/स प्रयोग नियम",
                c if c.starts_with("3(ग)(आ)") || c.starts_with("3(ग)-बव") => {
                    "ब/व प्रयोग नियम"
                }
                c if c.starts_with("3(ग)(ई)")
                    || c.starts_with("3(ग)-ऋ")
                    || c.starts_with("3(ग)-कृ") =>
                {
                    "ऋ/कृ प्रयोग नियम"
                }
                c if c.starts_with("3(ग)(इ)") || c == "3(छ)" => "य/ए भेद नियम",
                c if c.starts_with("3(ग)(उ)") || c.starts_with("3(छ)-क्ष") => {
                    "क्ष/छ भेद नियम"
                }
                c if c.starts_with("3(ग)(ऊ)") || c.starts_with("3(छ)-ज्ञ") => {
                    "ज्ञ/ग्य भेद नियम"
                }
                c if c.starts_with("3(ग)") => "उस्तै उच्चारण हुने वर्ण प्रयोग नियम",
                c if c.starts_with("3(घ)") => "पदयोग/पदवियोग नियम",
                c if c.starts_with("3(ङ)") => "हलन्त नियम",
                c if c.starts_with("3(इ)") => "य/ए भेद नियम",
                c if c.starts_with("3(उ)") => "क्ष/छ भेद नियम",
                "3(ई)" => "शुद्ध-अशुद्ध शब्द सूची",
                _ => "वर्णविन्यास नियम",
            },
            Rule::Vyakaran(code) => match *code {
                "section4-phrase-style" => "शैलीगत/प्रयोगगत सुझाव",
                "section4-phrase-style-inferred-ko-ka" => "शैलीगत/प्रयोगगत सुझाव (अनुमित)",
                "PS-Saisanik-7(क)-तिर्यक्" => {
                    "शैक्षणिक व्याकरण ७(क) — तिर्यक् रूपको प्रयोग"
                }
                "PS-Saisanik-7(ख)-तिर्यक्" => {
                    "शैक्षणिक व्याकरण ७(ख) — तिर्यक् रूपको प्रयोग"
                }
                "PS-Saisanik-7(ग)-तिर्यक्" => {
                    "शैक्षणिक व्याकरण ७(ग) — तिर्यक् रूपको प्रयोग"
                }
                _ => "व्याकरण नियम",
            },
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

#[cfg(test)]
mod tests {
    use super::Rule;

    #[test]
    fn active_section_codes_map_to_expected_descriptions() {
        let cases = [
            ("3(क)(अ)-1", "ह्रस्व/दीर्घ स्वर नियम"),
            ("3(ख)(आ)-1", "चन्द्रबिन्दु/शिरबिन्दु नियम"),
            ("3(ग)(अ)-1", "श/ष/स प्रयोग नियम"),
            ("3(ग)(आ)-1", "ब/व प्रयोग नियम"),
            ("3(ग)(ई)-ऋ-1", "ऋ/कृ प्रयोग नियम"),
            ("3(घ)", "पदयोग/पदवियोग नियम"),
            ("3(ङ)-पदान्त", "हलन्त नियम"),
            ("3(छ)", "य/ए भेद नियम"),
            ("3(छ)-क्ष", "क्ष/छ भेद नियम"),
            ("3(छ)-ज्ञ", "ज्ञ/ग्य भेद नियम"),
            ("3(ई)", "शुद्ध-अशुद्ध शब्द सूची"),
        ];

        for (code, expected) in cases {
            let rule = Rule::VarnaVinyasNiyam(code);
            assert_eq!(rule.description(), expected, "wrong description for {code}");
            assert_eq!(rule.to_string(), expected, "wrong display text for {code}");
        }
    }

    #[test]
    fn vyakaran_codes_map_to_expected_descriptions() {
        let cases = [
            ("section4-phrase-style", "शैलीगत/प्रयोगगत सुझाव"),
            (
                "section4-phrase-style-inferred-ko-ka",
                "शैलीगत/प्रयोगगत सुझाव (अनुमित)",
            ),
            (
                "PS-Saisanik-7(क)-तिर्यक्",
                "शैक्षणिक व्याकरण ७(क) — तिर्यक् रूपको प्रयोग",
            ),
            (
                "PS-Saisanik-7(ख)-तिर्यक्",
                "शैक्षणिक व्याकरण ७(ख) — तिर्यक् रूपको प्रयोग",
            ),
            (
                "PS-Saisanik-7(ग)-तिर्यक्",
                "शैक्षणिक व्याकरण ७(ग) — तिर्यक् रूपको प्रयोग",
            ),
        ];

        for (code, expected) in cases {
            let rule = Rule::Vyakaran(code);
            assert_eq!(rule.description(), expected, "wrong description for {code}");
            assert_eq!(rule.to_string(), expected, "wrong display text for {code}");
        }
    }
}
