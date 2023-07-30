use chrono::{DateTime, Utc};
use std::fmt::Debug;
use serde::Serialize;

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize, Clone)]
pub struct SnippetCreateForm {
    pub title: String,
    pub content: String,
    pub expires: i32,
}

impl Default  for SnippetCreateForm {
    fn default() -> Self  {
          SnippetCreateForm{
              title: String::default(), content: String::default(), expires: 365
          }
    }
}

impl SnippetCreateForm {
    pub fn validate(&self) -> HashMap<&'static str, &'static str> {
        let mut error_map = HashMap::new();
        let title_is_empty = self.title.trim().is_empty();
        let title_is_too_long = self.title.graphemes(true).count() > 100;
        let content_is_empty = self.content.trim().is_empty();
        if title_is_empty {
            error_map.insert("title", "title cannot be blank");
        }
        if title_is_too_long {
            error_map.insert("title", "title cannot be more than 100 characters long");
        }
        if content_is_empty {
            error_map.insert("content", "content cannot be blank");
        }
        error_map
    }
}



#[derive(Debug, Serialize)]
pub struct Snippet {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
}

