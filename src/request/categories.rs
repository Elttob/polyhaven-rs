use std::collections::HashMap;

use anyhow::Result;

use crate::data::asset::AssetType;

pub struct Params {
    pub asset_type: AssetType,
    pub in_categories: Vec<String>,
}

impl Params {
    pub fn as_query_params(&self) -> String {
        let mut params = HashMap::new();

        if !self.in_categories.is_empty() {
            params.insert("in", self.in_categories.iter()
                .map(|x| x.to_string())
                .reduce(|a, b| format!("{},{}", a, b))
                .unwrap()
            );
        }

        let query_params = params.into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .reduce(|a, b| format!("{}&{}", a, b))
            .unwrap_or_default();

        query_params
    }
}

pub async fn categories(params: Params) -> Result<HashMap<String, u32>> {
    let asset_type = match params.asset_type {
        AssetType::HDRI => "hdris",
        AssetType::Texture => "textures",
        AssetType::Model => "models",
    };
    let url = format!("https://api.polyhaven.com/categories/{}?{}", asset_type, params.as_query_params());
    let resp = reqwest::get(url).await?.json::<HashMap<String, u32>>().await?;
    Ok(resp)
}