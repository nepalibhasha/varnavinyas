use serde::Deserialize;
use varnavinyas_parikshak::DiagnosticCategory;

/// LSP server configuration, synced from client settings.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub categories: EnabledCategories,
}

/// Per-category enable/disable toggles. All default to true.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct EnabledCategories {
    pub hrasva_dirgha: bool,
    pub chandrabindu: bool,
    pub sha_sha_s: bool,
    pub ri_kri: bool,
    pub halanta: bool,
    pub ya_e: bool,
    pub ksha_chhya: bool,
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
            ri_kri: true,
            halanta: true,
            ya_e: true,
            ksha_chhya: true,
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
            DiagnosticCategory::RiKri => self.ri_kri,
            DiagnosticCategory::Halanta => self.halanta,
            DiagnosticCategory::YaE => self.ya_e,
            DiagnosticCategory::KshaChhya => self.ksha_chhya,
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
            DiagnosticCategory::RiKri,
            DiagnosticCategory::Halanta,
            DiagnosticCategory::YaE,
            DiagnosticCategory::KshaChhya,
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
}
