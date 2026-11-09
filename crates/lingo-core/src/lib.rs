//! Provider-neutral language routing for call and local text translation.
//! A route with target=None or target=source is passthrough (original audio).

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "zh")]
    TraditionalChinese,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "es")]
    Spanish,
}

impl Language {
    pub const fn code(self) -> &'static str {
        match self {
            Self::TraditionalChinese => "zh-TW",
            Self::Japanese => "ja-JP",
            Self::English => "en-US",
            Self::Korean => "ko-KR",
            Self::French => "fr-FR",
            Self::Spanish => "es-ES",
        }
    }

    pub const fn output_code(self) -> &'static str {
        match self {
            Self::TraditionalChinese => "zh",
            Self::Japanese => "ja",
            Self::English => "en",
            Self::Korean => "ko",
            Self::French => "fr",
            Self::Spanish => "es",
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::TraditionalChinese => "Traditional Chinese",
            Self::Japanese => "Japanese",
            Self::English => "English",
            Self::Korean => "Korean",
            Self::French => "French",
            Self::Spanish => "Spanish",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Direction {
    pub source: Language,
    pub target: Language,
}

impl Direction {
    pub fn new(source: Language, target: Language) -> Result<Self, &'static str> {
        if source == target {
            return Err("source and target languages must differ");
        }
        Ok(Self { source, target })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CallRoutes {
    pub outbound: Direction,
    pub inbound: Direction,
}

impl Default for CallRoutes {
    fn default() -> Self {
        Self {
            outbound: Direction::new(Language::TraditionalChinese, Language::Japanese)
                .expect("languages differ"),
            inbound: Direction::new(Language::Japanese, Language::TraditionalChinese)
                .expect("languages differ"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDirection {
    pub source: Language,
    /// None => forward original audio, with no translation provider call.
    pub target: Option<Language>,
}

impl AudioDirection {
    pub fn needs_translation(self) -> bool {
        match self.target {
            Some(target) => target != self.source,
            None => false,
        }
    }

    pub fn output_language(self) -> Option<Language> {
        if self.needs_translation() {
            self.target
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallLanguageSettings {
    pub outbound: AudioDirection,
    pub inbound: AudioDirection,
}

impl CallLanguageSettings {
    pub fn needs_translation(self) -> bool {
        self.outbound.needs_translation() || self.inbound.needs_translation()
    }

    pub fn translation_session_count(self) -> usize {
        self.outbound.needs_translation() as usize + self.inbound.needs_translation() as usize
    }
}

impl Default for CallLanguageSettings {
    fn default() -> Self {
        Self {
            outbound: AudioDirection {
                source: Language::TraditionalChinese,
                target: Some(Language::Japanese),
            },
            inbound: AudioDirection {
                source: Language::Japanese,
                target: Some(Language::TraditionalChinese),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_routes_are_bidirectional() {
        let routes = CallRoutes::default();
        assert_eq!(routes.outbound.source.code(), "zh-TW");
        assert_eq!(routes.outbound.target.code(), "ja-JP");
        assert_eq!(routes.inbound.source.code(), "ja-JP");
        assert_eq!(routes.inbound.target.code(), "zh-TW");
        assert_eq!(CallLanguageSettings::default().translation_session_count(), 2);
    }

    #[test]
    fn identical_languages_are_not_translation() {
        assert!(Direction::new(Language::Japanese, Language::Japanese).is_err());
        let passthrough = AudioDirection {
            source: Language::Japanese,
            target: Some(Language::Japanese),
        };
        assert!(!passthrough.needs_translation());
    }

    #[test]
    fn both_directions_passthrough_need_zero_ai_sessions() {
        let settings = CallLanguageSettings {
            outbound: AudioDirection {
                source: Language::TraditionalChinese,
                target: None,
            },
            inbound: AudioDirection {
                source: Language::Japanese,
                target: None,
            },
        };
        assert_eq!(settings.translation_session_count(), 0);
        assert!(!settings.needs_translation());
    }

    #[test]
    fn serialized_languages_are_api_output_codes() {
        assert_eq!(serde_json::to_string(&Language::Korean).unwrap(), "\"ko\"");
        let json = serde_json::to_string(&CallLanguageSettings::default()).unwrap();
        assert!(json.contains("\"target\":\"ja\""));
        assert_eq!(
            serde_json::from_str::<CallLanguageSettings>(&json).unwrap(),
            CallLanguageSettings::default()
        );
    }
}
