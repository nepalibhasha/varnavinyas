use crate::model::rule_spec::PatternRule;

pub mod aadhi_vriddhi;
pub mod chandrabindu_shirbindu;
pub mod halanta_ra_ajanta;
pub mod hrasva_dirgha;
pub mod panchham;
pub mod ustai_ucharan_varnaharu;

pub fn varna_vinyasa_rules() -> Vec<PatternRule> {
    let mut rules = Vec::new();
    rules.extend(ka_rules());
    rules.extend(kha_rules());
    rules.extend(ga_rules());
    rules.extend(nga_rules());
    rules
}

fn ka_rules() -> Vec<PatternRule> {
    vec![
        // 3(क) ह्रस्वदीर्घ वर्ण र मात्रा (इ, ई, ि, ी, उ, ऊ, ु, ू) को प्रयोगसम्बन्धी नियम
        PatternRule {
            spec: hrasva_dirgha::SPEC_PREFIX_HRASVA,
            apply: hrasva_dirgha::rule_prefix_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_DVI_TRI_HRASVA,
            apply: hrasva_dirgha::rule_dvi_tri_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_INITIAL_NAME_HRASVA,
            apply: hrasva_dirgha::rule_initial_name_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_INITIAL_AAGANTUK_HRASVA,
            apply: hrasva_dirgha::rule_initial_aagantuk_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_SUFFIX_NU,
            apply: hrasva_dirgha::rule_suffix_nu_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_SUFFIX_ELI,
            apply: hrasva_dirgha::rule_suffix_eli_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_PRONOUN,
            apply: hrasva_dirgha::rule_pronoun_vowel_length,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_INITIAL_ADJECTIVE_HRASVA,
            apply: hrasva_dirgha::rule_initial_adjective_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_INITIAL_NUMBER_HRASVA,
            apply: hrasva_dirgha::rule_initial_number_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_PS_IYA_HRASVA_EXCEPTIONS,
            apply: hrasva_dirgha::rule_ps_iya_hrasva_exceptions,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_INITIAL_AVYAYA_HRASVA,
            apply: hrasva_dirgha::rule_initial_avyaya_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_INITIAL_ONOMATOPOEIC_HRASVA,
            apply: hrasva_dirgha::rule_initial_onomatopoeic_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_TADBHAV,
            apply: hrasva_dirgha::rule_tadbhav_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_MEDIAL_PREFIX_HRASVA,
            apply: hrasva_dirgha::rule_medial_prefix_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_MEDIAL_SUFFIX_HRASVA,
            apply: hrasva_dirgha::rule_medial_suffix_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_MEDIAL_DERIVED_NAME_HRASVA,
            apply: hrasva_dirgha::rule_medial_derived_name_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_MEDIAL_UNDERIVED_NAME_HRASVA,
            apply: hrasva_dirgha::rule_medial_underived_name_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_MEDIAL_AAGANTUK_NAME_HRASVA,
            apply: hrasva_dirgha::rule_medial_aagantuk_name_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_MEDIAL_ADJECTIVE_HRASVA,
            apply: hrasva_dirgha::rule_medial_adjective_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_MEDIAL_AVYAYA_HRASVA,
            apply: hrasva_dirgha::rule_medial_avyaya_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_MEDIAL_ONOMATOPOEIC_HRASVA,
            apply: hrasva_dirgha::rule_medial_onomatopoeic_hrasva,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_KINSHIP,
            apply: hrasva_dirgha::rule_kinship_tadbhav,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_FINAL_HRASVA_ENDINGS,
            apply: hrasva_dirgha::rule_final_hrasva_endings,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_INITIAL_TATSAM_DIRGHA,
            apply: hrasva_dirgha::rule_initial_tatsam_dirgha,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_SU_PREFIX_PRESERVES_DIRGHA,
            apply: hrasva_dirgha::rule_su_prefix_preserves_dirgha,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_SUFFIX_PRESERVES,
            apply: hrasva_dirgha::rule_suffix_preserves_dirgha,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_SUFFIX_FAMILY_PRESERVES_DIRGHA,
            apply: hrasva_dirgha::rule_suffix_family_preserves_dirgha,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_FINAL_II_SUFFIX_DIRGHA,
            apply: hrasva_dirgha::rule_final_ii_suffix_dirgha,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_FINAL_VATI_VI_DIRGHA,
            apply: hrasva_dirgha::rule_final_vati_vi_dirgha,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_FINAL_ADJECTIVE_DIRGHA,
            apply: hrasva_dirgha::rule_final_adjective_dirgha,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_DIRGHA_ENDINGS,
            apply: hrasva_dirgha::rule_dirgha_endings,
        },
        PatternRule {
            spec: hrasva_dirgha::SPEC_KOSHA_BACKED,
            apply: hrasva_dirgha::kosha_backed_dirgha_correction,
        },
        PatternRule {
            spec: aadhi_vriddhi::SPEC_AADHI_VRIDDHI,
            apply: aadhi_vriddhi::rule_aadhi_vriddhi,
        },
    ]
}

fn kha_rules() -> Vec<PatternRule> {
    vec![
        // 3(ख) चन्द्रविन्दु (ँ), शिरविन्दु (ं) र पञ्चम वर्ण (ङ, ञ, ण, न, म) को प्रयोगसम्बन्धी नियम
        PatternRule {
            spec: chandrabindu_shirbindu::SPEC_CHANDRABINDU,
            apply: chandrabindu_shirbindu::rule_chandrabindu,
        },
        PatternRule {
            spec: panchham::SPEC_PANCHHAM,
            apply: panchham::rule_panchham_varna,
        },
    ]
}

fn ga_rules() -> Vec<PatternRule> {
    vec![
        // 3(ग) उस्तै उच्चारण हुने वर्णहरू (श/ष/स, ऋ/रि, ब/व, य/ए, क्ष/छ्य, क्षे/छे) आदिको प्रयोगसम्बन्धी नियम
        PatternRule {
            spec: ustai_ucharan_varnaharu::SPEC_SIBILANT,
            apply: ustai_ucharan_varnaharu::rule_sibilant,
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
            spec: ustai_ucharan_varnaharu::SPEC_RI_KRI,
            apply: ustai_ucharan_varnaharu::rule_ri_kri,
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
    ]
}

fn nga_rules() -> Vec<PatternRule> {
    vec![
        // 3(ङ) हलन्त र अजन्त प्रयोगसम्बन्धी नियम
        PatternRule {
            spec: halanta_ra_ajanta::SPEC_HALANTA,
            apply: halanta_ra_ajanta::rule_halanta,
        },
        // 3(च) लिपिगत विशिष्टता र अन्य केही ध्यान दिनुपर्ने कुराहरू
        // Note: not yet modeled as dedicated pattern rules.
    ]
}
