use anyhow::Result;

use crate::{data, json};

pub async fn get(id: &str) -> Result<data::files::Files> {
    let url = format!("https://api.polyhaven.com/files/{}", id);
    let resp = reqwest::get(url).await?.json::<json::files::Files>().await?;
    Ok(resp.into())
}