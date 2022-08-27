use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct AssetInfo {
    pub id: String,
    pub name: String,
    pub date_published: DateTime<Utc>,
    pub download_count: u64,
    pub authors: HashMap<String, String>,
    pub donated: bool,
    pub categories: Vec<String>,
    pub tags: Vec<String>,

    pub asset: Asset
}

impl AssetInfo {
    pub fn thumbnail(&self, resolution: u32) -> String {
        format!("https://cdn.polyhaven.com/asset_img/thumbs/{}.png?height={}", self.id, resolution)
    }
}

#[derive(Debug)]
pub enum AssetType {
    HDRI,
    Texture,
    Model
}

#[derive(Debug)]
pub enum Asset {
    HDRI(HDRIAsset),
    Texture(TextureAsset),
    Model(ModelAsset),
    Unparsed
}

#[derive(Debug)]
pub struct HDRIAsset {
    pub whitebalance: Option<u32>,
    pub backplates: bool,
    pub evs_cap: u32,
    pub coords: Option<(f32, f32)>
}

#[derive(Debug)]
pub struct TextureAsset {
    pub dimensions: (f32, f32)
}

#[derive(Debug)]
pub struct ModelAsset;

