use anyhow::Result;

use crate::{data, json};

pub async fn get(id: &str) -> Result<data::asset::AssetInfo> {
    let url = format!("https://api.polyhaven.com/info/{}", id);
    let resp = reqwest::get(url).await?.json::<json::asset::AssetInfo>().await?;
    Ok(data::asset::AssetInfo::from_json(resp, id.to_string()))
}