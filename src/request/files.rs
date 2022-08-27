use anyhow::Result;

use crate::{data, json};

pub async fn get(id: &str) -> Result<data::files::Files> {
    let info_url = format!("https://api.polyhaven.com/info/{}", id);
    let info_resp = reqwest::get(info_url).await?.json::<json::asset::AssetInfo>().await?;

    let files_url = format!("https://api.polyhaven.com/files/{}", id);
    let files_resp = reqwest::get(files_url).await?;

    match info_resp.asset_type {
        0 => Ok(data::files::Files::HDRI(files_resp.json::<json::files::HDRIFiles>().await?.into())),
        1 => Ok(data::files::Files::Texture(files_resp.json::<json::files::TextureFiles>().await?.into())),
        2 => Ok(data::files::Files::Model(files_resp.json::<json::files::ModelFiles>().await?.into())),
        _ => anyhow::bail!("Couldn't detect asset type")
    }
}