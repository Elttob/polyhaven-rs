use anyhow::Result;

use crate::{data, json};

pub async fn get(id: &str) -> Result<data::author::Author> {
    let url = format!("https://api.polyhaven.com/author/{}", id);
    let resp = reqwest::get(url).await?.json::<json::author::Author>().await?;
    Ok(resp.into())
}