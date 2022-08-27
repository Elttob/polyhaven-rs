use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use crate::response::asset;

#[derive(Deserialize)]
pub struct AssetInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub asset_type: i32,
    pub name: String,
    pub date_published: i64,
    pub download_count: u64,
    pub authors: HashMap<String, String>,
    pub donated: Option<bool>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub whitebalance: Option<u32>,
    pub backplates: Option<bool>,
    pub evs_cap: Option<u32>,
    pub coords: Option<(f32, f32)>,

    pub dimensions: Option<(f32, f32)>
}

impl From<AssetInfo> for asset::AssetInfo {
    fn from(json: AssetInfo) -> Self {
        Self {
            id: json.id,
            name: json.name,
            date_published: DateTime::from_utc(NaiveDateTime::from_timestamp(json.date_published, 0), Utc),
            download_count: json.download_count,
            authors: json.authors,
            donated: json.donated.unwrap_or(false),
            categories: json.categories,
            tags: json.tags,
            asset: match json.asset_type {
                0 => asset::Asset::HDRI(asset::HDRIAsset {
                    whitebalance: json.whitebalance,
                    backplates: json.backplates.unwrap_or(false),
                    evs_cap: json.evs_cap.unwrap_or(0),
                    coords: json.coords
                }),
                1 => asset::Asset::Texture(asset::TextureAsset {
                    dimensions: json.dimensions.unwrap_or((0.0, 0.0)),
                }),
                2 => asset::Asset::Model(asset::ModelAsset),
                _ => asset::Asset::Unparsed
            },
        }
    }
}