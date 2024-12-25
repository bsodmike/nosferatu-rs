use askama::Template;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use std::collections::HashMap;
use std::sync::LazyLock;
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub struct TaggedContent {
    content: HashMap<String, String>,
}

impl<'a> TaggedContent {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &'a str, value: String) {
        self.content.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &'a str) -> Option<&String> {
        self.content.get(key)
    }
}

pub struct TaggedContentBuilder {
    content: TaggedContent,
}

impl<'a> TaggedContentBuilder {
    pub fn new() -> Self {
        Self {
            content: TaggedContent::new(),
        }
    }

    pub fn from(content: Vec<(&'a str, String)>) -> Self {
        let mut builder = Self::new();
        for (key, value) in content {
            builder.add(key, value);
        }

        builder
    }

    pub fn add(&mut self, key: &'a str, value: String) -> &mut Self {
        self.content.add(key, value);
        self
    }

    pub fn build(&self) -> TaggedContent {
        self.content.clone()
    }
}

#[derive(Debug)]
pub struct I18nBundle<'a> {
    content: HashMap<&'a str, TaggedContent>,
}

impl<'a> I18nBundle<'a> {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
        }
    }

    pub fn get(&self, key: &'a str) -> Option<&TaggedContent> {
        self.content.get(key)
    }

    pub fn fetch(&self, language: &'a str, tag: &'a str) -> Option<String> {
        if let Some(content) = self.get(language) {
            content.get(tag).map(|s| s.clone())
        } else {
            None
        }
    }

    pub fn fetch_bundle(&self, language: &'a str) -> TaggedContent {
        self.get(language).unwrap().clone()
    }

    pub fn create_language(&mut self, key: &'a str) {
        let lang = self.get(key);

        // Do not replace content for language if it already exists
        if lang.is_some() {
            return;
        }

        let content = TaggedContent::new();
        self.content.insert(key, content);
    }

    fn replace_content(&mut self, key: &'a str, value: TaggedContent) {
        self.content.insert(key, value);
    }

    pub fn add_to_content(&mut self, key: &'a str, value: TaggedContent) {
        if let Some(collection) = self.get(key) {
            // FIXME: This is a temporary workaround borrowing issues; potential performance
            // penalty in terms of heap allocation
            let mut new_collection = collection.clone();
            new_collection.content.extend(value.content.clone());

            self.replace_content(key, new_collection);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fetch_items() {
        let mut i18n = I18nBundle::new();
        let mut content = TaggedContent::new();
        content.add("brown_fox", "The brown fox is just lazy".to_string());

        let mut content2 = TaggedContent::new();
        content2.add("random_text", "This is just random text".to_string());

        i18n.create_language("en");
        i18n.add_to_content("en", content);
        i18n.add_to_content("en", content2);

        let resp = i18n.fetch("en", "brown_fox");
        assert_eq!(resp, Some("The brown fox is just lazy".to_string()));

        let resp = i18n.fetch("en", "never_existed");

        assert_eq!(resp, None);
        let resp = i18n.fetch("fr", "never_existed");
        assert_eq!(resp, None);
    }

    #[test]
    fn add_translation_items2() {
        let mut i18n = I18nBundle::new();
        let mut content = TaggedContent::new();
        content.add("brown_fox", "The brown fox is just lazy".to_string());

        let mut content2 = TaggedContent::new();
        content2.add("random_text", "This is just random text".to_string());

        i18n.create_language("en");
        i18n.add_to_content("en", content);
        i18n.add_to_content("en", content2);

        let mut content = TaggedContent::new();
        content.add("brown_fox", "The brown fox is just lazy".to_string());
        content.add("random_text", "This is just random text".to_string());

        let reference = i18n.get("en").unwrap();
        assert_eq!(reference.content.values().len(), 2);
        assert_eq!(content.content.values().len(), 2);
        assert_eq!(reference, &content);
    }

    #[test]
    fn ensure_translations_cannot_be_destroyed() {
        let mut i18n = I18nBundle::new();
        let mut content = TaggedContent::new();
        content.add("brown_fox", "The brown fox is just lazy".to_string());

        i18n.create_language("en");
        i18n.replace_content("en", content);

        i18n.create_language("en");

        assert_eq!(
            i18n.get("en").unwrap().get("brown_fox"),
            Some(&"The brown fox is just lazy".to_string())
        );
    }
}
