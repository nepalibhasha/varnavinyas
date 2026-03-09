mod aa;
mod helpers;

// Academy source:
// docs/Notices-pages-77-99.md
// 3(ख) चन्द्रविन्दु (ँ), शिरविन्दु (ं) र पञ्चम वर्ण ... को प्रयोगसम्बन्धी नियम
//
// Organization note:
// - 3(ख)(अ) panchham/shirbindu logic lives in `structural.rs` (`rule_panchham_varna`)
// - this module family covers the 3(ख)(आ) chandrabindu rules and tatsam shirobindu normalization

pub use aa::{SPEC_CHANDRABINDU, rule_chandrabindu};
