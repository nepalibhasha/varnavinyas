use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};

mod ba_va;
mod gya_gyan;
mod ksha_chhya;
mod ri_kri;
mod sibilant;
mod ya_e;

pub use ba_va::rule_ba_va;
pub use gya_gyan::rule_gya_gyan;
pub use ksha_chhya::rule_ksha_chhya;
pub use ri_kri::rule_ri_kri;
pub use sibilant::rule_sibilant;
pub use ya_e::rule_ya_e;

// Section 3(ग) source context:
// docs/Notices-pages-77-99.md (pp. 76-80),
// "(ग) उस्तै उच्चारण हुने वर्णहरू (श/ष/स, ब/व, य/ए, ऋ/रि, क्ष/छ्य, क्षे/छे)..."
//
// Implementation policy for this module:
// - follow Academy subrule intent first;
// - keep transformations conservative with kosha plausibility checks;
// - prefer "no suggestion" over speculative replacement.
//
// Mapping note:
// - This module implements the word-level parts of 3(ग).
// - Academy subsection order is: (अ) श/ष/स, (आ) ब/व/ओ, (इ) ए/य,
//   (ई) ऋ/रि, (उ) क्ष/छ्य/छे, (ऊ) ज्ञ/ग्या/ग्याँ.
// - Some function bodies still reflect historical growth order, so comments and
//   registry ordering are used to preserve auditability against the notice text.
// - Broad lexical inventories in the document are represented as guard logic
//   rather than exhaustive in-code wordlists.

// (अ) 'श, ष, स' को प्रयोग
// Document subrules:
// - 'श' (तत्सम): patterns around sibilant clusters / चवर्ग / ऋ,र / विसर्ग
// - 'ष' (तत्सम): retroflex and इ/उ + क/प environments
// - 'स' (all origins): especially तद्भव/आगन्तुक normalization
//
// Current implementation focus:
// - subrule-8 and subrule-9 style normalization for तद्भव/आगन्तुक.
// - leaves tatsam "श/ष/स" distinctions unchanged at this layer.
pub const SPEC_SIBILANT: RuleSpec = RuleSpec {
    id: "ortho-sibilant",
    category: RuleCategory::ShaShaS,
    kind: DiagnosticKind::Error,
    priority: 310,
    citation: Rule::VarnaVinyasNiyam("3(ग)(अ)"),
    examples: &[("रजिष्टर", "रजिस्टर")],
};

// (आ) 'ब', 'व' र 'ओ' को प्रयोग
// Document subrules:
// - 'ब' environments and lexical classes
// - 'व' environments and lexical classes
// - dedicated 'ओ' usage classes
//
// Current implementation focus:
// - conservative b<->v single-position swap with kosha validation.
// - 'ओ' logic is currently not a dedicated generalized rule in this module.
pub const SPEC_BA_VA: RuleSpec = RuleSpec {
    id: "ortho-ba-va",
    category: RuleCategory::ShaShaS,
    kind: DiagnosticKind::Error,
    priority: 315,
    citation: Rule::VarnaVinyasNiyam("3(ग)(आ)"),
    examples: &[("बिदेश", "विदेश"), ("बिज्ञान", "विज्ञान")],
};

// (इ) 'ए' र 'य' को प्रयोग
// Document subrules include multiple morphological buckets (verb forms,
// participles, pronouns, tatsam/loan classes).
//
// Current implementation focus:
// - conservative word-initial ए<->य alternation with kosha validation.
pub const SPEC_YA_E: RuleSpec = RuleSpec {
    id: "ortho-ya-e",
    category: RuleCategory::YaE,
    kind: DiagnosticKind::Error,
    priority: 350,
    citation: Rule::VarnaVinyasNiyam("3(ग)(इ)"),
    examples: &[("एथार्थ", "यथार्थ"), ("यकता", "एकता")],
};

// (ई) 'ऋ' र 'रि' को प्रयोग
// Document subrules:
// - ऋ/kṛ-family forms are tatsam-only;
// - तद्भव/आगन्तुक generally keep "रि".
//
// Current implementation focus:
// - tatsam-only रि->ऋ and क्रि->कृ corrections, guarded by kosha.
pub const SPEC_RI_KRI: RuleSpec = RuleSpec {
    id: "ortho-ri-kri",
    category: RuleCategory::RiKri,
    kind: DiagnosticKind::Error,
    priority: 320,
    citation: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
    examples: &[("रिषि", "ऋषि"), ("क्रिति", "कृति")],
};

// (उ) क्ष/क्षे/क्ष्य र छ/छे/छ्य को प्रयोग
// Document subrules:
// - tatsam: क्ष/क्षे/क्ष्य;
// - broader usage: छ/छे/छ्य.
//
// Current implementation focus:
// - bidirectional candidate generation among listed grapheme families
//   with kosha attestation guard.
pub const SPEC_KSHA_CHHYA: RuleSpec = RuleSpec {
    id: "ortho-ksha-chhya",
    category: RuleCategory::KshaChhya,
    kind: DiagnosticKind::Error,
    priority: 360,
    citation: Rule::VarnaVinyasNiyam("3(ग)(उ)"),
    examples: &[("लछ्य", "लक्ष्य"), ("छेत्र", "क्षेत्र")],
};

// (ऊ) 'ज्ञ', 'ग्याँ' र 'ग्या' को प्रयोग
// Document subrules:
// - ज्ञ: tatsam-only class;
// - ग्याँ/ग्या: mostly Nepali/loan usage classes.
//
// Current implementation focus:
// - normalize ग्याँ/ग्या -> ज्ञा only when target is kosha-attested.
pub const SPEC_GYA_GYAN: RuleSpec = RuleSpec {
    id: "ortho-gya-gyan",
    category: RuleCategory::GyaGyan,
    kind: DiagnosticKind::Error,
    priority: 365,
    citation: Rule::VarnaVinyasNiyam("3(ग)(ऊ)"),
    examples: &[("अग्यान", "अज्ञान"), ("प्रग्या", "प्रज्ञा")],
};
