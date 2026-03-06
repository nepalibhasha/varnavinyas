//! Backward-compatible facade for orthographic rule exports.
//!
//! New code should prefer the descriptive niyama modules:
//! - `chandrabindu_shirbindu`
//! - `ustai_ucharan_varnaharu`
//! - `aadhi_vriddhi`
//! - `halanta_ra_ajanta`

pub use crate::aadhi_vriddhi::{SPEC_AADHI_VRIDDHI, rule_aadhi_vriddhi};
pub use crate::chandrabindu_shirbindu::{SPEC_CHANDRABINDU, rule_chandrabindu};
pub use crate::halanta_ra_ajanta::{SPEC_HALANTA, rule_halanta};
pub use crate::ustai_ucharan_varnaharu::{
    SPEC_BA_VA, SPEC_GYA_GYAN, SPEC_KSHA_CHHYA, SPEC_RI_KRI, SPEC_SIBILANT, SPEC_YA_E, rule_ba_va,
    rule_gya_gyan, rule_ksha_chhya, rule_ri_kri, rule_sibilant, rule_ya_e,
};
