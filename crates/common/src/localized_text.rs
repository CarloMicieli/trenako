use std::collections::HashMap;
use strum_macros;
use strum_macros::{Display, EnumString};

/// It represents a multi-language text.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct LocalizedText(HashMap<Language, String>);

impl LocalizedText {
    /// Creates a new `LocalizedText` with an English label
    pub fn with_english(label: &str) -> Self {
        let mut labels = HashMap::new();
        labels.insert(Language::English, label.to_string());
        LocalizedText(labels)
    }

    /// Creates a new `LocalizedText` with an Italian label
    pub fn with_italian(label: &str) -> Self {
        let mut labels = HashMap::new();
        labels.insert(Language::Italian, label.to_string());
        LocalizedText(labels)
    }

    /// Returns the label in English, if exists
    pub fn english(&self) -> Option<&String> {
        self.0.get(&Language::English)
    }

    /// Returns the label in Italian, if exists
    pub fn italian(&self) -> Option<&String> {
        self.0.get(&Language::Italian)
    }
}

#[derive(Debug, Default)]
pub struct LocalizedTextBuilder {
    english: Option<String>,
    italian: Option<String>,
}

impl LocalizedTextBuilder {
    /// add an English label
    pub fn english_text(mut self, label: &str) -> Self {
        self.english = Some(label.to_string());
        self
    }

    /// add an Italian label
    pub fn italian_text(mut self, label: &str) -> Self {
        self.italian = Some(label.to_string());
        self
    }

    /// Build a new `LocalizedText` value
    pub fn build(self) -> LocalizedText {
        let mut values = HashMap::with_capacity(2);

        if let Some(english) = self.english {
            values.insert(Language::English, english);
        }

        if let Some(italian) = self.italian {
            values.insert(Language::Italian, italian);
        }

        LocalizedText(values)
    }
}

/// The languages supported by the application
#[derive(Debug, Eq, PartialEq, Hash, Display, EnumString, Serialize, Deserialize, Clone, Copy)]
pub enum Language {
    /// the English language
    #[strum(serialize = "en")]
    #[serde(rename = "en")]
    English,

    /// the Italian language
    #[strum(serialize = "it")]
    #[serde(rename = "it")]
    Italian,
}

#[cfg(test)]
mod test {
    use super::*;

    mod languages {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case(Language::English, "en")]
        #[case(Language::Italian, "it")]
        fn it_should_display_languages(#[case] language: Language, #[case] expected: &str) {
            assert_eq!(expected, language.to_string());
        }

        #[rstest]
        #[case("en", Ok(Language::English))]
        #[case("it", Ok(Language::Italian))]
        fn it_should_parse_languages(#[case] input: &str, #[case] expected: Result<Language, ParseError>) {
            let result = input.parse::<Language>();
            assert_eq!(expected, result);
        }
    }

    mod localize_texts {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_english_localized_texts() {
            let localized_text = LocalizedText::with_english("Hello world");
            assert_eq!(Some(&String::from("Hello world")), localized_text.english());
            assert_eq!(None, localized_text.italian());
        }

        #[test]
        fn it_should_create_italian_localized_texts() {
            let localized_text = LocalizedText::with_italian("Buongiorno");
            assert_eq!(Some(&String::from("Buongiorno")), localized_text.italian());
            assert_eq!(None, localized_text.english());
        }

        #[test]
        fn it_should_build_localize_texts() {
            let localized_text = LocalizedTextBuilder::default()
                .english_text("hello world")
                .italian_text("Buongiorno")
                .build();

            assert_eq!(Some(&String::from("Buongiorno")), localized_text.italian());
            assert_eq!(Some(&String::from("hello world")), localized_text.english());
        }
    }
}