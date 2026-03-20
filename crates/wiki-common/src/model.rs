use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WikiPage {
    pub title: String,
    pub content: String,
}
