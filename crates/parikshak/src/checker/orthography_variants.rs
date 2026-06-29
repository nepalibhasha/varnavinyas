use crate::diagnostic::DiagnosticCategory;

pub(crate) struct AcceptedOrthographicVariant {
    pub(crate) common: &'static str,
    pub(crate) strict: &'static str,
    pub(crate) category: DiagnosticCategory,
    pub(crate) reason: &'static str,
    pub(crate) source_note: &'static str,
}

// This registry is deliberately small and reviewed. Do not add entries from
// corpus frequency alone; each entry must identify the strict source pressure
// and the reason the common form should remain non-blocking in editorial mode.
pub(crate) const ACCEPTED_ORTHOGRAPHIC_VARIANTS: &[AcceptedOrthographicVariant] = &[
    AcceptedOrthographicVariant {
        common: "संघीय",
        strict: "सङ्घीय",
        category: DiagnosticCategory::Chandrabindu,
        reason: "प्रचलित आधुनिक लेखन; कडा प्रज्ञा रूप सङ्घीय हो",
        source_note: "Notices shuddha/ashuddha table lists संघीय -> सङ्घीय; modern official/editorial use widely writes संघीय.",
    },
    AcceptedOrthographicVariant {
        common: "कांग्रेस",
        strict: "काङ्ग्रेस",
        category: DiagnosticCategory::Chandrabindu,
        reason: "आगन्तुक/राजनीतिक नामका रूपमा प्रचलित लेखन; कडा शब्दकोशीय रूप काङ्ग्रेस हो",
        source_note: "Dictionary data contains काङ्ग्रेस, but कांग्रेस is a widely used political/proper-name loan form.",
    },
    AcceptedOrthographicVariant {
        common: "संसद",
        strict: "संसद्",
        category: DiagnosticCategory::Halanta,
        reason: "प्रचलित आधुनिक लेखन; कडा तत्सम पदान्त रूप संसद् हो",
        source_note: "Both source documents prefer संसद्; lexicon word data and modern prose also attest संसद.",
    },
];
