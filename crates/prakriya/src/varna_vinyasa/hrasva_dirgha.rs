mod a;
mod aa;
#[path = "hrasva_dirgha/helpers.rs"]
mod helpers;
mod i;
mod u;
mod uu;

// Academy source:
// docs/Notices-pages-77-99.md
// 3(क) नेपाली वर्णविन्यास -> pages 71-74 in the notice markdown/PDF transcription.
//
// Organization rule for this module family:
// - keep subsection code in Academy order: (अ), (आ), (इ), (ई), (उ), (ऊ)
// - keep numbered subrule comments beside the actual implementation site
// - keep broad fallback/helpers below specific numbered rules, in helper submodules

pub use a::{
    SPEC_DVI_TRI_HRASVA, SPEC_INITIAL_AAGANTUK_HRASVA, SPEC_INITIAL_ADJECTIVE_HRASVA,
    SPEC_INITIAL_AVYAYA_HRASVA, SPEC_INITIAL_NAME_HRASVA, SPEC_INITIAL_NUMBER_HRASVA,
    SPEC_INITIAL_ONOMATOPOEIC_HRASVA, SPEC_PREFIX_HRASVA, SPEC_PRONOUN, SPEC_SUFFIX_ELI,
    SPEC_SUFFIX_NU, SPEC_TADBHAV, rule_dvi_tri_hrasva, rule_initial_aagantuk_hrasva,
    rule_initial_adjective_hrasva, rule_initial_avyaya_hrasva, rule_initial_name_hrasva,
    rule_initial_number_hrasva, rule_initial_onomatopoeic_hrasva, rule_prefix_hrasva,
    rule_pronoun_vowel_length, rule_suffix_eli_hrasva, rule_suffix_nu_hrasva, rule_tadbhav_hrasva,
};
pub use aa::{
    SPEC_MEDIAL_AAGANTUK_NAME_HRASVA, SPEC_MEDIAL_ADJECTIVE_HRASVA, SPEC_MEDIAL_AVYAYA_HRASVA,
    SPEC_MEDIAL_DERIVED_NAME_HRASVA, SPEC_MEDIAL_ONOMATOPOEIC_HRASVA, SPEC_MEDIAL_PREFIX_HRASVA,
    SPEC_MEDIAL_SUFFIX_HRASVA, SPEC_MEDIAL_UNDERIVED_NAME_HRASVA, rule_medial_aagantuk_name_hrasva,
    rule_medial_adjective_hrasva, rule_medial_avyaya_hrasva, rule_medial_derived_name_hrasva,
    rule_medial_onomatopoeic_hrasva, rule_medial_prefix_hrasva, rule_medial_suffix_hrasva,
    rule_medial_underived_name_hrasva,
};
pub use i::{
    SPEC_FINAL_HRASVA_ENDINGS, SPEC_KINSHIP, rule_final_hrasva_endings, rule_kinship_tadbhav,
};
pub use u::{
    SPEC_SUFFIX_FAMILY_PRESERVES_DIRGHA, SPEC_SUFFIX_PRESERVES,
    rule_suffix_family_preserves_dirgha, rule_suffix_preserves_dirgha,
};
pub use uu::{
    SPEC_DIRGHA_ENDINGS, SPEC_KOSHA_BACKED, kosha_backed_dirgha_correction, rule_dirgha_endings,
};
