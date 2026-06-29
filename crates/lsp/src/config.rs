use serde::Deserialize;
use varnavinyas_parikshak::{DiagnosticCategory, OrthographyMode, PunctuationMode};

/// LSP server configuration, synced from client settings.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Config {
    pub categories: EnabledCategories,
    #[serde(alias = "orthography_mode")]
    pub orthography_mode: OrthographyModeSetting,
    #[serde(alias = "punctuation_mode")]
    pub punctuation_mode: PunctuationModeSetting,
    #[serde(alias = "debug_include_noop_heuristics")]
    pub debug_include_noop_heuristics: bool,
}

/// How reviewed common-vs-strict orthographic variants should be classified.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum OrthographyModeSetting {
    #[default]
    AcademyStrict,
    CommonEditorial,
}

impl OrthographyModeSetting {
    pub fn to_core(self) -> OrthographyMode {
        match self {
            Self::AcademyStrict => OrthographyMode::AcademyStrict,
            Self::CommonEditorial => OrthographyMode::CommonEditorial,
        }
    }
}

/// How Section 5 punctuation diagnostics should be classified.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PunctuationModeSetting {
    #[default]
    Strict,
    NormalizedEditorial,
}

impl PunctuationModeSetting {
    pub fn to_core(self) -> PunctuationMode {
        match self {
            Self::Strict => PunctuationMode::Strict,
            Self::NormalizedEditorial => PunctuationMode::NormalizedEditorial,
        }
    }
}

/// Per-category enable/disable toggles. All default to true.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct EnabledCategories {
    pub hrasva_dirgha: bool,
    pub chandrabindu: bool,
    #[serde(alias = "shaShaSa")]
    pub sha_sha_s: bool,
    pub ba_va: bool,
    pub ri_kri: bool,
    pub halanta: bool,
    pub aadhi_vriddhi: bool,
    pub ya_e: bool,
    pub ksha_chhya: bool,
    pub gya_gyan: bool,
    pub sandhi: bool,
    pub punctuation: bool,
    pub shuddha_table: bool,
}

impl Default for EnabledCategories {
    fn default() -> Self {
        Self {
            hrasva_dirgha: true,
            chandrabindu: true,
            sha_sha_s: true,
            ba_va: true,
            ri_kri: true,
            halanta: true,
            aadhi_vriddhi: true,
            ya_e: true,
            ksha_chhya: true,
            gya_gyan: true,
            sandhi: true,
            punctuation: true,
            shuddha_table: true,
        }
    }
}

impl EnabledCategories {
    /// Check if a given diagnostic category is enabled.
    pub fn is_enabled(&self, category: DiagnosticCategory) -> bool {
        match category {
            DiagnosticCategory::HrasvaDirgha => self.hrasva_dirgha,
            DiagnosticCategory::Chandrabindu => self.chandrabindu,
            DiagnosticCategory::ShaShaS => self.sha_sha_s,
            DiagnosticCategory::BaVa => self.ba_va,
            DiagnosticCategory::RiKri => self.ri_kri,
            DiagnosticCategory::Halanta => self.halanta,
            DiagnosticCategory::AadhiVriddhi => self.aadhi_vriddhi,
            DiagnosticCategory::YaE => self.ya_e,
            DiagnosticCategory::KshaChhya => self.ksha_chhya,
            DiagnosticCategory::GyaGyan => self.gya_gyan,
            DiagnosticCategory::Sandhi => self.sandhi,
            DiagnosticCategory::Punctuation => self.punctuation,
            DiagnosticCategory::ShuddhaTable => self.shuddha_table,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_all_enabled() {
        let config = Config::default();
        for cat in [
            DiagnosticCategory::HrasvaDirgha,
            DiagnosticCategory::Chandrabindu,
            DiagnosticCategory::ShaShaS,
            DiagnosticCategory::BaVa,
            DiagnosticCategory::RiKri,
            DiagnosticCategory::Halanta,
            DiagnosticCategory::AadhiVriddhi,
            DiagnosticCategory::YaE,
            DiagnosticCategory::KshaChhya,
            DiagnosticCategory::GyaGyan,
            DiagnosticCategory::Sandhi,
            DiagnosticCategory::Punctuation,
            DiagnosticCategory::ShuddhaTable,
        ] {
            assert!(
                config.categories.is_enabled(cat),
                "category {cat:?} should be enabled by default"
            );
        }
    }

    #[test]
    fn disable_single_category() {
        let mut config = Config::default();
        config.categories.hrasva_dirgha = false;
        assert!(
            !config
                .categories
                .is_enabled(DiagnosticCategory::HrasvaDirgha)
        );
        assert!(
            config
                .categories
                .is_enabled(DiagnosticCategory::Chandrabindu)
        );
    }

    #[test]
    fn parses_sha_sha_s_camel_case_setting() {
        let config: Config = serde_json::from_value(serde_json::json!({
            "categories": {
                "shaShaS": false
            }
        }))
        .expect("config should parse");

        assert!(!config.categories.is_enabled(DiagnosticCategory::ShaShaS));
    }

    #[test]
    fn parses_all_stable_category_settings_independently() {
        let config: Config = serde_json::from_value(serde_json::json!({
            "categories": {
                "baVa": false,
                "aadhiVriddhi": false,
                "gyaGyan": false
            }
        }))
        .expect("config should parse");

        assert!(!config.categories.is_enabled(DiagnosticCategory::BaVa));
        assert!(
            !config
                .categories
                .is_enabled(DiagnosticCategory::AadhiVriddhi)
        );
        assert!(!config.categories.is_enabled(DiagnosticCategory::GyaGyan));
        assert!(config.categories.is_enabled(DiagnosticCategory::ShaShaS));
        assert!(
            config
                .categories
                .is_enabled(DiagnosticCategory::ShuddhaTable)
        );
        assert!(config.categories.is_enabled(DiagnosticCategory::KshaChhya));
    }

    #[test]
    fn accepts_legacy_sha_sha_sa_setting_alias() {
        let config: Config = serde_json::from_value(serde_json::json!({
            "categories": {
                "shaShaSa": false
            }
        }))
        .expect("config should parse");

        assert!(!config.categories.is_enabled(DiagnosticCategory::ShaShaS));
    }

    #[test]
    fn default_punctuation_mode_is_strict() {
        let config = Config::default();
        assert_eq!(config.punctuation_mode, PunctuationModeSetting::Strict);
    }

    #[test]
    fn default_orthography_mode_is_academy_strict() {
        let config = Config::default();
        assert_eq!(
            config.orthography_mode,
            OrthographyModeSetting::AcademyStrict
        );
    }

    #[test]
    fn parses_vscode_top_level_runtime_settings() {
        let config: Config = serde_json::from_value(serde_json::json!({
            "orthographyMode": "common-editorial",
            "punctuationMode": "normalized-editorial",
            "debugIncludeNoopHeuristics": true
        }))
        .expect("config should parse");

        assert_eq!(
            config.orthography_mode,
            OrthographyModeSetting::CommonEditorial
        );
        assert_eq!(
            config.punctuation_mode,
            PunctuationModeSetting::NormalizedEditorial
        );
        assert!(config.debug_include_noop_heuristics);
    }

    #[test]
    fn accepts_legacy_snake_case_runtime_settings() {
        let config: Config = serde_json::from_value(serde_json::json!({
            "orthography_mode": "common-editorial",
            "punctuation_mode": "normalized-editorial",
            "debug_include_noop_heuristics": true
        }))
        .expect("config should parse");

        assert_eq!(
            config.orthography_mode,
            OrthographyModeSetting::CommonEditorial
        );
        assert_eq!(
            config.punctuation_mode,
            PunctuationModeSetting::NormalizedEditorial
        );
        assert!(config.debug_include_noop_heuristics);
    }

    #[test]
    fn default_debug_noop_heuristics_is_off() {
        let config = Config::default();
        assert!(!config.debug_include_noop_heuristics);
    }
}
