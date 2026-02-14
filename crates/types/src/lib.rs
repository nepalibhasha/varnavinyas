/// Word origin classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Origin {
    /// तत्सम — direct Sanskrit borrowing, retains original form.
    Tatsam,
    /// तद्भव — modified Sanskrit, follows Nepali phonology.
    Tadbhav,
    /// देशज — native Nepali word.
    Deshaj,
    /// आगन्तुक — foreign loanword (English, Arabic, Hindi, etc.).
    Aagantuk,
}
