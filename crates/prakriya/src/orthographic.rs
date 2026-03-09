//! Backward-compatible facade for orthographic rule exports.
//!
//! New code should prefer the descriptive niyama modules:
//! - `varna_vinyasa::chandrabindu_shirbindu`
//! - `varna_vinyasa::ustai_ucharan_varnaharu`
//! - `varna_vinyasa::aadhi_vriddhi`
//! - `varna_vinyasa::halanta_ra_ajanta`
//! - `varna_vinyasa::panchham`

pub use crate::varna_vinyasa::aadhi_vriddhi::{SPEC_AADHI_VRIDDHI, rule_aadhi_vriddhi};
pub use crate::varna_vinyasa::chandrabindu_shirbindu::{SPEC_CHANDRABINDU, rule_chandrabindu};
pub use crate::varna_vinyasa::halanta_ra_ajanta::{SPEC_HALANTA, rule_halanta};
pub use crate::varna_vinyasa::panchham::{SPEC_PANCHHAM, rule_panchham_varna};
pub use crate::varna_vinyasa::ustai_ucharan_varnaharu::{
    SPEC_BA_VA, SPEC_GYA_GYAN, SPEC_KSHA_CHHYA, SPEC_RI_KRI, SPEC_SIBILANT, SPEC_YA_E, rule_ba_va,
    rule_gya_gyan, rule_ksha_chhya, rule_ri_kri, rule_sibilant, rule_ya_e,
};
