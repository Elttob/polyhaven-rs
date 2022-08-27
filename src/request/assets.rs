use std::collections::HashMap;

use anyhow::Result;

use crate::{data::{self, asset::AssetType}, json};

pub struct Params {
    pub asset_type: Option<AssetType>,
    pub categories: Vec<String>,
    pub author: Option<String>
}

impl Params {
    pub fn as_query_params(&self) -> String {
        let mut params = HashMap::new();

        if !self.categories.is_empty() {
            params.insert("categories", self.categories.iter()
                .map(|x| x.to_string())
                .reduce(|a, b| format!("{},{}", a, b))
                .unwrap()
            );
        }
        if let Some(asset_type) = &self.asset_type {
            params.insert("type", match asset_type {
                AssetType::HDRI => "hdris".to_string(),
                AssetType::Model => "models".to_string(),
                AssetType::Texture => "textures".to_string()
            });
        }
        if let Some(author) = &self.author {
            params.insert("author", author.to_string());
        }
        
        let query_params = params.into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .reduce(|a, b| format!("{}&{}", a, b))
            .unwrap_or_default();

        query_params
    }
}

pub async fn get(params: Params) -> Result<HashMap<String, data::asset::AssetInfo>> {
    let url = format!("https://api.polyhaven.com/assets?{}", params.as_query_params());
    let resp = reqwest::get(url).await?.json::<HashMap<String, json::asset::AssetInfo>>().await?;
    Ok(
        resp.into_iter()
        .map(|(id, json)| (id.to_string(), data::asset::AssetInfo::from_json(json, id)))
        .collect()
    )
}