use super::{kosha_backed_dirgha_correction, rule_dirgha_endings};
use varnavinyas_shabda::{Origin, classify, decompose};

pub(crate) fn exact_headword_supported(word: &str) -> bool {
    let lex = varnavinyas_kosha::kosha();
    lex.lookup(word).is_some()
}

pub(crate) fn has_exact_headword_left_compound_tail(input: &str, tails: &[&str]) -> bool {
    let lex = varnavinyas_kosha::kosha();
    tails.iter().any(|tail| {
        let Some(left) = input.strip_suffix(tail) else {
            return false;
        };
        !left.is_empty() && lex.lookup(left).is_some()
    })
}

pub(super) mod final_classes {
    use super::*;
    pub(crate) fn final_dirgha_class_for(
        output: &str,
        vowel_label: &str,
    ) -> (&'static str, String) {
        if is_place_river_language_dirgha(output) {
            return (
                "3(क)(ऊ)-11",
                "स्थान, नदी र भाषा बुझाउने शब्दमा दीर्घ हुन्छ".to_string(),
            );
        }

        if is_number_final_dirgha(output) {
            return ("3(क)(ऊ)-9", "सङ्ख्यावाचक शब्दहरू अन्त्यमा दीर्घ हुन्छन्".to_string());
        }

        if is_vati_vi_suffix_dirgha(output) {
            return (
                "3(क)(ऊ)-2",
                "वती, वी प्रत्यय लागेर बनेका शब्दहरू दीर्घ हुन्छन्".to_string(),
            );
        }

        if is_profession_jati_thar_dirgha(output) {
            return (
                "3(क)(ऊ)-5",
                "पेसा, जाति र थर बुझाउने शब्दमा दीर्घ हुन्छ".to_string(),
            );
        }

        if is_adjective_final_dirgha(output) {
            return ("3(क)(ऊ)-8", "सबै ईकारान्त विशेषणहरू दीर्घ हुन्छन्".to_string());
        }

        if is_hi_final_dirgha(output) {
            return (
                "3(क)(ऊ)-12",
                "'चाहिँ'बाहेक 'ही' अन्त्यमा आउने शब्दमा दीर्घ हुन्छ".to_string(),
            );
        }

        if is_ari_tari_adverb_dirgha(output) {
            return (
                "3(क)(ऊ)-13",
                "अरी, तरी अन्त्यमा आउने अव्यय शब्दमा दीर्घ हुन्छ".to_string(),
            );
        }

        if output.ends_with("जी") || output.ends_with("ज्यू") {
            return ("3(क)(ऊ)-16", "मानबोधक वा आदरसूचक पदमा दीर्घ हुन्छ".to_string());
        }

        if output.ends_with("वानी")
            || output.ends_with("बी")
            || output.ends_with("री")
            || output.ends_with("दारी")
        {
            return (
                "3(क)(ऊ)-15",
                "विशेषणबाट बनेका भाववाची नामपदमा दीर्घ हुन्छ".to_string(),
            );
        }

        (
            "3(क)(ऊ)",
            format!("शब्दको अन्त्यमा दीर्घ {} आवश्यक (शब्दकोश प्रमाणित)", vowel_label),
        )
    }

    pub(crate) fn final_hrasva_class_for(output: &str) -> Option<(&'static str, String)> {
        if is_ps_final_dirgha_exception(output) {
            return None;
        }

        if is_location_inanimate_final_hrasva(output) {
            return Some((
                "3(क)(इ)-2",
                "स्थानवाचक र निर्जीव नामहरू अन्त्यमा ह्रस्व हुन्छन्".to_string(),
            ));
        }

        if has_final_hrasva_suffix_family(output) {
            return Some((
                "3(क)(इ)-3",
                "आइ, आईं, याइ, याइँ, आउ, नु प्रत्यय लागेका शब्दहरू अन्त्यमा ह्रस्व हुन्छन्".to_string(),
            ));
        }

        if has_final_hrasva_adjective_suffix(output) {
            return Some((
                "3(क)(इ)-4",
                "आरु, आलु, उ, एलु, तु प्रत्यय लागेका विशेषण शब्दहरू अन्त्यमा ह्रस्व हुन्छन्".to_string(),
            ));
        }

        if is_mula_avyaya_final_hrasva(output) {
            return Some(("3(क)(इ)-5", "सबै मूल अव्ययहरू अन्त्यमा ह्रस्व हुन्छन्".to_string()));
        }

        if is_vibhakti_final_hrasva(output) {
            return Some((
                "3(क)(इ)-6",
                "लाई, की, री, नीबाहेक अन्य विभक्तिहरू अन्त्यमा ह्रस्व हुन्छन्".to_string(),
            ));
        }

        if is_ti_avyaya_final_hrasva(output) {
            return Some((
                "3(क)(इ)-7",
                "अरी, तरी र ही प्रत्यय लागेका बाहेक 'ति' अन्त्यमा आउने अव्ययहरू ह्रस्व हुन्छन्".to_string(),
            ));
        }

        if has_ti_dhi_ni_ti_pi_final_hrasva(output) {
            return Some((
                "3(क)(इ)-9",
                "ति, धि, नि, टि, पि अन्त्यमा आउने सबै शब्दहरू ह्रस्व हुन्छन्".to_string(),
            ));
        }

        None
    }

    pub(crate) fn is_place_river_language_dirgha(output: &str) -> bool {
        matches!(
            output,
            "नेपाली"
                | "हिन्दी"
                | "मैथिली"
                | "भोजपुरी"
                | "फारसी"
                | "उर्दू"
                | "गुल्मी"
                | "मेची"
                | "कोसी"
                | "कर्णाली"
                | "महाकाली"
        )
    }

    pub(crate) fn is_location_inanimate_final_hrasva(output: &str) -> bool {
        matches!(
            output,
            "गाउँ"
                | "ठाउँ"
                | "मलेखु"
                | "साँखु"
                | "टेकु"
                | "घिउ"
                | "आलु"
                | "केराउ"
                | "घाउ"
                | "धनु"
        )
    }

    pub(crate) fn has_final_hrasva_suffix_family(output: &str) -> bool {
        output.ends_with("ाइ")
            || output.ends_with("ाइँ")
            || output.ends_with("याइ")
            || output.ends_with("याइँ")
            || output.ends_with("ाउ")
            || output.ends_with("नु")
    }

    pub(crate) fn has_final_hrasva_adjective_suffix(output: &str) -> bool {
        matches!(
            output,
            "सिकारु"
                | "दुधालु"
                | "बिखालु"
                | "यात्रु"
                | "घरेलु"
                | "पाल्तु"
        )
    }

    pub(crate) fn is_mula_avyaya_final_hrasva(output: &str) -> bool {
        matches!(
            output,
            "अगाडि"
                | "पछाडि"
                | "माथि"
                | "अनि"
                | "पनि"
                | "मुनि"
                | "भोलि"
                | "अस्ति"
                | "फेरि"
                | "चोटि"
                | "पर्सि"
                | "बर्सेनि"
        )
    }

    pub(crate) fn is_vibhakti_final_hrasva(output: &str) -> bool {
        matches!(output, "देखि" | "निम्ति" | "लागि")
    }

    pub(crate) fn is_ti_avyaya_final_hrasva(output: &str) -> bool {
        matches!(output, "त्यति" | "जति" | "कति" | "उति")
    }

    pub(crate) fn has_ti_dhi_ni_ti_pi_final_hrasva(output: &str) -> bool {
        output.ends_with("ति")
            || output.ends_with("धि")
            || output.ends_with("नि")
            || output.ends_with("टि")
            || output.ends_with("पि")
    }

    pub(crate) fn ps_final_dirgha_exception_for_hrasva(input: &str) -> Option<&'static str> {
        PS_FINAL_DIRGHA_EXCEPTIONS
            .iter()
            .find_map(|(hrasva, dirgha)| (*hrasva == input).then_some(*dirgha))
    }

    pub(crate) fn is_ps_final_dirgha_exception(input: &str) -> bool {
        PS_FINAL_DIRGHA_EXCEPTIONS
            .iter()
            .any(|(_, dirgha)| *dirgha == input)
    }

    pub(crate) fn is_known_correct_final_dirgha(input: &str) -> bool {
        if is_ps_final_dirgha_exception(input) {
            return true;
        }

        let hrasva = super::hrasva_helpers::replace_final_dirgha_with_hrasva(input);
        if hrasva == input {
            return false;
        }

        if let Some(p) = rule_dirgha_endings(&hrasva) {
            if p.output == input {
                return true;
            }
        }

        if let Some(p) = kosha_backed_dirgha_correction(&hrasva) {
            if p.output == input {
                return true;
            }
        }

        // If the hrasva form is a known vibhakti (e.g. लागि, देखि) and the dirgha
        // form also exists in the lexicon, it is a legitimate separate word — most
        // commonly an asamapaka (gerund) form of the same root (e.g. लागी = "having
        // applied").  Treat it as correctly spelled rather than a vibhakti misspelling.
        if is_vibhakti_final_hrasva(&hrasva) {
            let kosha = varnavinyas_kosha::kosha();
            if kosha.contains(input) {
                return true;
            }
        }

        false
    }

    const PS_FINAL_DIRGHA_EXCEPTIONS: &[(&str, &str)] = &[
        ("श्रेणि", "श्रेणी"),
        ("युवति", "युवती"),
        ("सूचि", "सूची"),
        ("अञ्जलि", "अञ्जली"),
        ("श्रद्धाञ्जलि", "श्रद्धाञ्जली"),
        ("आवलि", "आवली"),
        ("शब्दावलि", "शब्दावली"),
        ("औषधि", "औषधी"),
    ];

    pub(crate) fn is_profession_jati_thar_dirgha(output: &str) -> bool {
        matches!(
            output,
            "व्यापारी"
                | "हली"
                | "पादरी"
                | "खेती"
                | "ठकुरी"
                | "छेत्री"
                | "कामी"
                | "राई"
                | "जैसी"
                | "थारू"
                | "लिम्बू"
                | "अधिकारी"
                | "उप्रेती"
                | "सुवेदी"
                | "प्रसाईं"
                | "गिरी"
        )
    }

    pub(crate) fn is_adjective_final_dirgha(output: &str) -> bool {
        if is_vati_vi_suffix_dirgha(output)
            || is_profession_jati_thar_dirgha(output)
            || is_place_river_language_dirgha(output)
            || is_number_final_dirgha(output)
            || is_hi_final_dirgha(output)
            || is_ari_tari_adverb_dirgha(output)
        {
            return false;
        }

        let kosha = varnavinyas_kosha::kosha();
        let Some(entry) = kosha.lookup(output) else {
            return false;
        };

        super::hrasva_helpers::is_adjective_pos(entry.pos)
    }

    pub(crate) fn is_number_final_dirgha(output: &str) -> bool {
        matches!(output, "दुई" | "साठी" | "सत्तरी" | "असी")
    }

    pub(crate) fn is_vati_vi_suffix_dirgha(output: &str) -> bool {
        output.ends_with("वती") || output.ends_with("ावी") || output.ends_with("स्वी")
    }

    pub(crate) fn is_hi_final_dirgha(output: &str) -> bool {
        output.ends_with("ही") && output != "चाहिँ"
    }

    pub(crate) fn is_ari_tari_adverb_dirgha(output: &str) -> bool {
        matches!(
            output,
            "यसरी"
                | "त्यसरी"
                | "जसरी"
                | "सरासरी"
                | "कस्तरी"
                | "सुस्तरी"
                | "त्यस्तरी"
        )
    }
}

pub(super) mod hrasva_helpers {
    use super::*;
    pub(crate) fn has_tatsam_suffix(input: &str) -> bool {
        input.ends_with("ीकरण")
            || input.ends_with("ीकृत")
            || input.ends_with("ीकार")
            || input.ends_with("ीय")
            || input.ends_with("ीन")
    }

    /// शब्द ज्ञात स्त्रीलिङ्गी दीर्घ-अन्त्य ढाँचासँग मिल्छ कि छैन जाँच्ने।
    pub(crate) fn is_feminine_dirgha_pattern(input: &str) -> bool {
        // ी मा अन्त्य हुने सही स्त्रीलिङ्गी रूपहरू
        input.ends_with("नी")
            || input.ends_with("डी")
            || input.ends_with("ती")
            || input.ends_with("ली")
    }

    /// शब्द नातागोता वर्गमा पर्छ कि (वा त्यसमाथि प्रत्यय लागेको रूप हो कि) जाँच्ने।
    pub(crate) fn is_kinship_dirgha_pattern(input: &str) -> bool {
        static KINSHIP_BASES: &[&str] = &[
            "दिदी",
            "बहिनी",
            "भाउजू",
            "फुपू",
            "सासू",
            "जेठानी",
            "कान्छी",
            "बुहारी",
            "मितिनी",
        ];
        // ठीक उही रूप वा प्रत्यय-लागेका रूप (ले, मा, को, लाई आदि) जाँच्ने
        for base in KINSHIP_BASES {
            if input == *base || input.starts_with(base) {
                return true;
            }
        }
        false
    }

    pub(crate) fn is_pronoun_candidate(input: &str) -> bool {
        matches!(
            input,
            "तिमि"
                | "तीमी"
                | "तीमि"
                | "तिनि"
                | "तीनी"
                | "तीनि"
                | "यिनि"
                | "यीनी"
                | "यीनि"
                | "उनि"
                | "ऊनी"
                | "ऊनि"
                | "हामि"
        )
    }

    pub(crate) fn is_initial_hrasva_adjective(output: &str) -> bool {
        let lex = varnavinyas_kosha::kosha();
        if let Some(entry) = lex.lookup(output) {
            if is_initial_hrasva_avyaya(output) || is_initial_hrasva_onomatopoeic(output) {
                return false;
            }
            if is_adjective_pos(entry.pos) {
                return true;
            }
        }

        // Some Academy example adjectives are tagged as nouns in the kosha metadata.
        matches!(output, "चिल्लो" | "हुस्सु")
    }

    pub(crate) fn is_initial_hrasva_number(output: &str) -> bool {
        if output == "तीन" {
            return false;
        }

        let morphology = decompose(output);
        let base = if morphology.root.is_empty() {
            output
        } else {
            morphology.root.as_str()
        };

        matches!(
            output,
            "दुई" | "उन्नाइस"
                | "बिस"
                | "उनन्तिस"
                | "तिस"
                | "उनान्सय"
                | "त्रिचालिस"
                | "त्रिपन्न"
                | "त्रिसट्ठी"
                | "चौबिस"
                | "सत्ताइस"
        ) || matches!(
            base,
            "उन्नाइस"
                | "बिस"
                | "उनन्तिस"
                | "तिस"
                | "उनान्सय"
                | "त्रिचालिस"
                | "त्रिपन्न"
                | "त्रिसट्ठी"
                | "चौबिस"
                | "सत्ताइस"
        )
    }

    pub(crate) fn is_initial_hrasva_avyaya(output: &str) -> bool {
        let lex = varnavinyas_kosha::kosha();
        let Some(entry) = lex.lookup(output) else {
            return false;
        };

        entry.pos.contains("क्रि.वि.")
            || entry.pos.contains("क्रियाविशेषण")
            || entry.pos.contains("संयोजक")
            || entry.pos.contains("नामयोगी")
            || entry.pos.contains("ना.यो.")
    }

    pub(crate) fn is_initial_hrasva_onomatopoeic(output: &str) -> bool {
        let lex = varnavinyas_kosha::kosha();
        let Some(entry) = lex.lookup(output) else {
            return false;
        };

        entry.pos.contains("अ.मू.")
            || entry.pos.contains("अ. मू.")
            || entry.pos.contains("अमू.")
            || matches!(
                output,
                "किटिक्क"
                    | "पिटिक्क"
                    | "झिलिमिली"
                    | "टिलपिल"
                    | "मुसुक्क"
                    | "भुतुक्क"
                    | "कुपुकुपु"
                    | "टुलुटुलु"
            )
    }

    pub(crate) fn is_adjective_pos(pos: &str) -> bool {
        (pos.contains("विशेषण") && !pos.contains("क्रियाविशेषण"))
            || (pos.contains("वि.")
                && !pos.contains("क्रि.वि.")
                && !pos.contains("ना.वि.")
                && !pos.contains("वि.क्रि."))
    }

    pub(crate) fn has_medial_hrasva_suffix_family(output: &str) -> bool {
        matches!(
            output,
            "भौतिक"
                | "दैनिक"
                | "गायिका"
                | "लेखिका"
                | "कथित"
                | "अग्रिम"
                | "स्वर्णिम"
                | "गरिमा"
                | "वरिष्ठ"
                | "भावुक"
                | "भिक्षुक"
                | "परिचायिका"
                | "प्रतिकारी"
        )
    }

    pub(crate) fn is_name_pos(output: &str) -> bool {
        let lex = varnavinyas_kosha::kosha();
        let Some(entry) = lex.lookup(output) else {
            return false;
        };

        entry.pos.contains("ना.")
            || entry.pos.contains("नाम")
            || entry.pos.contains("ना. ")
            || entry.pos.contains("नाम ")
    }

    pub(crate) fn is_medial_hrasva_aagantuk_name(output: &str) -> bool {
        matches!(classify(output), Origin::Aagantuk) && is_name_pos(output)
    }

    pub(crate) fn is_medial_derived_name(output: &str) -> bool {
        let lex = varnavinyas_kosha::kosha();
        let Some(entry) = lex.lookup(output) else {
            return false;
        };
        if has_specific_hrasva_prefix_structure(output)
            || !is_name_pos(output)
            || matches!(classify(output), Origin::Aagantuk)
            || is_kinship_dirgha_pattern(output)
            || is_adjective_pos(entry.pos)
            || is_initial_hrasva_avyaya(output)
            || is_initial_hrasva_onomatopoeic(output)
        {
            return false;
        }

        let morphology = decompose(output);
        entry.pos.contains('+')
            || !morphology.prefixes.is_empty()
            || !morphology.suffixes.is_empty()
    }

    pub(crate) fn is_medial_underived_name(output: &str) -> bool {
        let lex = varnavinyas_kosha::kosha();
        let Some(entry) = lex.lookup(output) else {
            return false;
        };
        if has_specific_hrasva_prefix_structure(output)
            || !is_name_pos(output)
            || matches!(classify(output), Origin::Aagantuk)
            || is_kinship_dirgha_pattern(output)
            || is_adjective_pos(entry.pos)
            || is_initial_hrasva_avyaya(output)
            || is_initial_hrasva_onomatopoeic(output)
        {
            return false;
        }

        let morphology = decompose(output);
        !entry.pos.contains('+') && morphology.prefixes.is_empty() && morphology.suffixes.is_empty()
    }

    pub(crate) fn is_medial_hrasva_adjective(output: &str) -> bool {
        let lex = varnavinyas_kosha::kosha();
        let Some(entry) = lex.lookup(output) else {
            return false;
        };
        is_adjective_pos(entry.pos)
    }

    pub(crate) fn is_medial_hrasva_avyaya(output: &str) -> bool {
        let lex = varnavinyas_kosha::kosha();
        let Some(entry) = lex.lookup(output) else {
            return false;
        };
        entry.pos.contains("क्रि.वि.")
            || entry.pos.contains("क्रियाविशेषण")
            || entry.pos.contains("संयोजक")
            || entry.pos.contains("नामयोगी")
            || entry.pos.contains("ना.यो.")
    }

    pub(crate) fn medial_dirgha_to_hrasva_candidates(input: &str) -> Vec<String> {
        let chars: Vec<char> = input.chars().collect();
        if chars.len() < 4 {
            return Vec::new();
        }

        let mut out = Vec::new();
        // Academy 3(क)(आ): "medial" here excludes the first syllable.
        // At the codepoint level, index 1 is often the first consonant's vowel sign,
        // so start from index 2 to avoid stealing initial-hrasva cases like
        // सूमार्ग/कीसान into medial rules.
        for i in 2..chars.len() - 1 {
            let replacement = match chars[i] {
                'ी' => Some('ि'),
                'ू' => Some('ु'),
                'ई' => Some('इ'),
                'ऊ' => Some('उ'),
                _ => None,
            };
            let Some(rep) = replacement else {
                continue;
            };
            let mut candidate = chars.clone();
            candidate[i] = rep;
            out.push(candidate.into_iter().collect());
        }
        out
    }

    pub(crate) fn has_specific_hrasva_prefix_structure(output: &str) -> bool {
        const PREFIXES: &[&str] = &[
            "नि",
            "दु",
            "वि",
            "उत",
            "उप",
            "कु",
            "सु",
            "अनु",
            "अभि",
            "अति",
            "अधि",
            "प्रति",
            "परि",
        ];
        let morphology = decompose(output);
        morphology
            .prefixes
            .iter()
            .any(|prefix| PREFIXES.contains(&prefix.as_str()))
    }

    pub(crate) fn is_initial_underived_name_candidate(output: &str) -> bool {
        let morphology = decompose(output);
        morphology.prefixes.is_empty()
            && morphology.suffixes.is_empty()
            && !matches!(morphology.origin, Origin::Aagantuk | Origin::Tatsam)
            && !is_initial_hrasva_adjective(output)
            && !is_initial_hrasva_number(output)
            && !is_initial_hrasva_avyaya(output)
            && !is_initial_hrasva_onomatopoeic(output)
    }

    pub(crate) fn is_initial_aagantuk_name_candidate(output: &str) -> bool {
        matches!(classify(output), Origin::Aagantuk) && !is_initial_hrasva_onomatopoeic(output)
    }

    /// शब्दको पहिलो अक्षर/मात्रामा आएको दीर्घ ई/ऊलाई ह्रस्वमा फिर्ता गर्छ।
    /// 3(क)(अ)-3/-4 मा शब्दादिको दीर्घ-लेखन जाँच्दा यही helper प्रयोग गरिन्छ।
    pub(crate) fn initial_dirgha_to_hrasva(input: &str) -> Option<String> {
        let mut output = String::with_capacity(input.len());
        let mut replaced = false;
        let mut saw_initial_vowel = false;
        let mut consonants_before_vowel = 0usize;

        for ch in input.chars() {
            if !saw_initial_vowel {
                match ch {
                    'ई' => {
                        output.push('इ');
                        replaced = true;
                        saw_initial_vowel = true;
                        continue;
                    }
                    'ऊ' => {
                        output.push('उ');
                        replaced = true;
                        saw_initial_vowel = true;
                        continue;
                    }
                    'ी' => {
                        if consonants_before_vowel > 1 {
                            return None;
                        }
                        output.push('ि');
                        replaced = true;
                        saw_initial_vowel = true;
                        continue;
                    }
                    'ू' => {
                        if consonants_before_vowel > 1 {
                            return None;
                        }
                        output.push('ु');
                        replaced = true;
                        saw_initial_vowel = true;
                        continue;
                    }
                    // पहिलो अक्षरमै अर्को स्वर आयो भने 3(क)(अ)-3/-4 लागू हुँदैन।
                    'अ' | 'आ' | 'इ' | 'उ' | 'ए' | 'ऐ' | 'ओ' | 'औ' | 'ऋ' | 'ा' | 'ि' | 'ु' | 'े'
                    | 'ै' | 'ो' | 'ौ' | 'ृ' => return None,
                    _ => {
                        if varnavinyas_akshar::is_vyanjan(ch) {
                            consonants_before_vowel += 1;
                        }
                    }
                }
            }
            output.push(ch);
        }

        replaced.then_some(output)
    }

    /// शब्दको पहिलो अक्षर/मात्रामा आएको ह्रस्व इ/उलाई दीर्घमा बदल्छ।
    /// 3(क)(ई)-1 मा संस्कृतबाट जस्ताको तस्तै आएका शब्द जाँच्दा यही helper प्रयोग हुन्छ।
    pub(crate) fn initial_hrasva_to_dirgha(input: &str) -> Option<String> {
        let mut output = String::with_capacity(input.len());
        let mut replaced = false;
        let mut saw_initial_vowel = false;
        let mut consonants_before_vowel = 0usize;

        for ch in input.chars() {
            if !saw_initial_vowel {
                match ch {
                    'इ' => {
                        output.push('ई');
                        replaced = true;
                        saw_initial_vowel = true;
                        continue;
                    }
                    'उ' => {
                        output.push('ऊ');
                        replaced = true;
                        saw_initial_vowel = true;
                        continue;
                    }
                    'ि' => {
                        if consonants_before_vowel > 1 {
                            return None;
                        }
                        output.push('ी');
                        replaced = true;
                        saw_initial_vowel = true;
                        continue;
                    }
                    'ु' => {
                        if consonants_before_vowel > 1 {
                            return None;
                        }
                        output.push('ू');
                        replaced = true;
                        saw_initial_vowel = true;
                        continue;
                    }
                    'अ' | 'आ' | 'ई' | 'ऊ' | 'ए' | 'ऐ' | 'ओ' | 'औ' | 'ऋ' | 'ा' | 'ी' | 'ू' | 'े'
                    | 'ै' | 'ो' | 'ौ' | 'ृ' => return None,
                    _ => {
                        if varnavinyas_akshar::is_vyanjan(ch) {
                            consonants_before_vowel += 1;
                        }
                    }
                }
            }
            output.push(ch);
        }

        replaced.then_some(output)
    }

    pub(crate) fn replace_final_dirgha_with_hrasva(s: &str) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        if let Some(last) = chars.last_mut() {
            *last = match *last {
                'ई' => 'इ',
                'ऊ' => 'उ',
                'ी' => 'ि',
                'ू' => 'ु',
                other => other,
            };
        }
        chars.into_iter().collect()
    }

    pub(crate) fn pronoun_wrong_dirgha_start(correct: &str) -> Option<String> {
        if let Some(rest) = correct.strip_prefix("ति") {
            return Some(format!("ती{rest}"));
        }
        if let Some(rest) = correct.strip_prefix("यि") {
            return Some(format!("यी{rest}"));
        }
        if let Some(rest) = correct.strip_prefix('उ') {
            return Some(format!("ऊ{rest}"));
        }
        None
    }
}
