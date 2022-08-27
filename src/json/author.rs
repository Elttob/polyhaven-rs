use serde::Deserialize;

use crate::data::author;

#[derive(Deserialize)]
pub struct Author {
    pub name: String,
    pub link: Option<String>,
    pub email: Option<String>,
    pub donate: Option<String>
}

impl From<Author> for author::Author {
    fn from(json: Author) -> Self {
        Self {
            name: json.name,
            link: json.link,
            email: json.email,
            donate: json.donate
        }
    }
}