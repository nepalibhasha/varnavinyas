use crate::aadhi_vriddhi;
use crate::chandrabindu_shirbindu;
use crate::halanta_ra_ajanta;
use crate::hrasva_dirgha;
use crate::rule_spec::PatternRule;
use crate::structural;
use crate::ustai_ucharan_varnaharu;

/// Rules organized by Academy Niyama sections.
///
/// This is a registry-level organization layer. Rule bodies may still live in
/// legacy modules; migration can happen incrementally without changing behavior.
pub fn section3_rules() -> Vec<PatternRule> {
    vec![
        // 3(क) ह्रस्वदीर्घ वर्ण र मात्रा (इ, ई, ि, ी, उ, ऊ, ु, ू) को प्रयोगसम्बन्धी नियम
        PatternRule {
            spec: hrasva_dirgha::SPEC_SUFFIX_NU,
            apply: hrasva_dirgha::rule_suffix_nu_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_SUFFIX_ELI,
            apply: hrasva_dirgha::rule_suffix_eli_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_SUFFIX_PRESERVES,
            apply: hrasva_dirgha::rule_suffix_preserves_dirgha,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_TADBHAV,
            apply: hrasva_dirgha::rule_tadbhav_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_DIRGHA_ENDINGS,
            apply: hrasva_dirgha::rule_dirgha_endings,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_KINSHIP,
            apply: hrasva_dirgha::rule_kinship_tadbhav,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_KOSHA_BACKED,
            apply: hrasva_dirgha::kosha_backed_dirgha_correction,
        },
        PatternRule {
            spec: aadhi_vriddhi::SPEC_AADHI_VRIDDHI,
            apply: aadhi_vriddhi::rule_aadhi_vriddhi,
        },
        // 3(ख) चन्द्रविन्दु (ँ), शिरविन्दु (ं) र पञ्चम वर्ण (ङ, ञ, ण, न, म) को प्रयोगसम्बन्धी नियम
        PatternRule {
            spec: chandrabindu_shirbindu::SPEC_CHANDRABINDU,
            apply: chandrabindu_shirbindu::rule_chandrabindu,
        },
        PatternRule {
            spec: structural::SPEC_PANCHHAM,
            apply: structural::rule_panchham_varna,
        },
        // 3(ग) उस्तै उच्चारण हुने वर्णहरू (श/ष/स, ऋ/रि, ब/व, य/ए, क्ष/छ्य, क्षे/छे) आदिको प्रयोगसम्बन्धी नियम
        PatternRule {
            spec: ustai_ucharan_varnaharu::SPEC_SIBILANT,
            apply: ustai_ucharan_varnaharu::rule_sibilant,
        },
        PatternRule {
            spec: ustai_ucharan_varnaharu::SPEC_RI_KRI,
            apply: ustai_ucharan_varnaharu::rule_ri_kri,
        },
        PatternRule {
            spec: ustai_ucharan_varnaharu::SPEC_BA_VA,
            apply: ustai_ucharan_varnaharu::rule_ba_va,
        },
        PatternRule {
            spec: ustai_ucharan_varnaharu::SPEC_YA_E,
            apply: ustai_ucharan_varnaharu::rule_ya_e,
        },
        PatternRule {
            spec: ustai_ucharan_varnaharu::SPEC_KSHA_CHHYA,
            apply: ustai_ucharan_varnaharu::rule_ksha_chhya,
        },
        PatternRule {
            spec: ustai_ucharan_varnaharu::SPEC_GYA_GYAN,
            apply: ustai_ucharan_varnaharu::rule_gya_gyan,
        },
        // 3(घ) पदयोग र पदवियोगसम्बन्धी नियम
        // Note: currently implemented at text-level in parikshak (not prakriya single-word rules).
        // 3(ङ) हलन्त र अजन्त प्रयोगसम्बन्धी नियम
        PatternRule {
            spec: halanta_ra_ajanta::SPEC_HALANTA,
            apply: halanta_ra_ajanta::rule_halanta,
        },
        // 3(च) लिपिगत विशिष्टता र अन्य केही ध्यान दिनुपर्ने कुराहरू
        // Note: not yet modeled as dedicated pattern rules.
    ]
}

pub fn non_section3_rules() -> Vec<PatternRule> {
    vec![
        // Section 4-style structural rules
        PatternRule {
            spec: structural::SPEC_SHRI,
            apply: structural::rule_shri_correction,
        },
        PatternRule {
            spec: structural::SPEC_REDUNDANT_SUFFIX,
            apply: structural::rule_redundant_suffix,
        },
    ]
}
