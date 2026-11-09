//! Domain types for bidirectional language routes.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Language {
    TraditionalChinese,
    Japanese,
}

impl Language {
    pub const fn code(self) -> &'static str {
        match self {
            Self::TraditionalChinese => "zh-TW",
            Self::Japanese => "ja-JP",
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
    }

    #[test]
    fn identical_languages_are_not_translation() {
        assert!(Direction::new(Language::Japanese, Language::Japanese).is_err());
    }
}
