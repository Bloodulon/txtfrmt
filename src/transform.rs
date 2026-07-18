use unicode_segmentation::UnicodeSegmentation;
use crate::layout;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Transform {
    ToRussian,
    ToEnglish,
    ToggleLayout,
    ToggleCase,
    ToUpper,
    ToLower,
    TitleCase,
    CleanWhitespace,
    Reverse,
    ToCamelCase,
    ToSnakeCase,
}

impl Transform {
    pub fn apply(&self, text: &str) -> String {
        match self {
            Transform::ToRussian => layout::to_russian(text),
            Transform::ToEnglish => layout::to_english(text),
            Transform::ToggleLayout => layout::toggle_layout(text),
            Transform::ToggleCase => toggle_case(text),
            Transform::ToUpper => text.to_uppercase(),
            Transform::ToLower => text.to_lowercase(),
            Transform::TitleCase => to_title_case(text),
            Transform::CleanWhitespace => clean_whitespace(text),
            Transform::Reverse => reverse(text),
            Transform::ToCamelCase => to_camel_case(text),
            Transform::ToSnakeCase => to_snake_case(text),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Transform::ToRussian => "Russian layout",
            Transform::ToEnglish => "English layout",
            Transform::ToggleLayout => "Toggle layout",
            Transform::ToggleCase => "Toggle case",
            Transform::ToUpper => "UPPERCASE",
            Transform::ToLower => "lowercase",
            Transform::TitleCase => "Title Case",
            Transform::CleanWhitespace => "Clean spaces",
            Transform::Reverse => "Reverse",
            Transform::ToCamelCase => "camelCase",
            Transform::ToSnakeCase => "snake_case",
        }
    }
}

fn toggle_case(text: &str) -> String {
    let upper = text.chars().filter(|c| c.is_uppercase()).count();
    let lower = text.chars().filter(|c| c.is_lowercase()).count();
    if upper > lower { text.to_lowercase() } else { text.to_uppercase() }
}

fn to_title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn clean_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn reverse(text: &str) -> String {
    text.graphemes(true).rev().collect()
}

fn to_camel_case(text: &str) -> String {
    let mut result = String::new();
    let mut cap = false;
    for c in text.chars() {
        if c == '_' || c == '-' || c == ' ' {
            cap = true;
        } else if cap {
            result.extend(c.to_uppercase());
            cap = false;
        } else {
            result.push(c);
        }
    }
    result
}

fn to_snake_case(text: &str) -> String {
    let mut result = String::new();
    for (i, c) in text.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap_or(c));
    }
    result
}
