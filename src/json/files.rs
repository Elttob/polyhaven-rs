use anyhow::Result;
use std::{collections::HashMap, str::FromStr};

use serde::Deserialize;

use crate::data::files;

fn parse_resolution(res_str: &str) -> Result<u64> {
    if res_str.ends_with("k") || res_str.ends_with("K") {
        match res_str[..res_str.len() - 1].parse::<u64>() {
            Ok(num) => Ok(num * 1024),
            Err(_) => anyhow::bail!("Couldn't parse file resolution")
        }
    } else {
        match res_str.parse() {
            Ok(num) => Ok(num),
            Err(_) => anyhow::bail!("Couldn't parse file resolution")
        }
    }
    
}

pub enum Files {
    HDRI(HDRIFiles),
    Texture(TextureFiles),
    Model(ModelFiles)
}

impl From<Files> for files::Files {
    fn from(json: Files) -> Self {
        match json {
            Files::HDRI(data) => files::Files::HDRI(data.into()),
            Files::Texture(data) => files::Files::Texture(data.into()),
            Files::Model(data) => files::Files::Model(data.into())
        }
    }
}

#[derive(Deserialize)]
pub struct FileData {
    pub url: Option<String>,
    pub md5: String,
    pub size: u64,
    pub include: Option<HashMap<String, FileData>>,
}

impl From<FileData> for files::FileData {
    fn from(json: FileData) -> Self {
        Self {
            url: json.url,
            md5: json.md5,
            size: json.size,
            include: json.include.unwrap_or_default().into_iter()
                .map(|(path, file_json)| (path, file_json.into()))
                .collect()
        }
    }
}

type FileResolution = String;

#[derive(Deserialize)]
pub struct HDRIFiles {
    pub hdri: HashMap<FileResolution, HashMap<HDRIFormat, FileData>>,
    pub backplates: HashMap<String, HashMap<HDRIBackplateFormat, FileData>>,
    pub colorchart: Option<FileData>,
    pub tonemapped: Option<FileData>
} 

impl From<HDRIFiles> for files::HDRIFiles {
    fn from(json: HDRIFiles) -> Self {
        Self {
            hdri: json.hdri.into_iter()
                .filter_map(|(res_str, formats_strs)| {
                    match parse_resolution(&res_str) {
                        Ok(res) => {
                            let formats = formats_strs.into_iter()
                                .map(|(format_str, data)| (
                                    files::HDRIFormat::from_str(&format_str).unwrap(), 
                                    files::FileData::from(data)
                                ))
                                .collect();
                            Some((res, formats))
                        },
                        Err(_) => None
                    }
                })
                .collect(),
            backplates: json.backplates.into_iter()
                .map(|(name, formats_strs)| {
                    let formats = formats_strs.into_iter()
                        .map(|(format_str, data)| (
                            files::HDRIBackplateFormat::from_str(&format_str).unwrap(),
                            files::FileData::from(data)
                        ))
                        .collect();
                    (name, formats)
                })
                .collect(),
            colorchart: json.colorchart.and_then(|data| Some(files::FileData::from(data))),
            tonemapped: json.tonemapped.and_then(|data| Some(files::FileData::from(data)))
        }
    }
}

pub type HDRIFormat = String;

pub type HDRIBackplateFormat = String;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct TextureFiles {
    pub blend: HashMap<FileResolution, FileData>,
    pub gltf: HashMap<FileResolution, FileData>,

    pub AO: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub arm: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Bump: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Diffuse: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Displacement: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Metal: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub nor_gl: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Rough: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub spec: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>
}

impl From<TextureFiles> for files::TextureFiles {
    fn from(json: TextureFiles) -> Self {
        let mut maps_json = HashMap::new();
        if let Some(map_json) = json.AO { maps_json.insert("AO", map_json); }
        if let Some(map_json) = json.arm { maps_json.insert("arm", map_json); }
        if let Some(map_json) = json.Bump { maps_json.insert("Bump", map_json); }
        if let Some(map_json) = json.Diffuse { maps_json.insert("Diffuse", map_json); }
        if let Some(map_json) = json.Displacement { maps_json.insert("Displacement", map_json); }
        if let Some(map_json) = json.Metal { maps_json.insert("Metal", map_json); }
        if let Some(map_json) = json.nor_gl { maps_json.insert("nor_gl", map_json); }
        if let Some(map_json) = json.Rough { maps_json.insert("Rough", map_json); }
        if let Some(map_json) = json.spec { maps_json.insert("spec", map_json); }
        Self {
            blend: json.blend.into_iter()
                .filter_map(|(res_str, file_json)| {
                    match parse_resolution(&res_str) {
                        Ok(res) => Some((res, files::FileData::from(file_json))),
                        Err(_) => None
                    }
                })
                .collect(),
            gltf: json.gltf.into_iter()
                .filter_map(|(res_str, file_json)| {
                    match parse_resolution(&res_str) {
                        Ok(res) => Some((res, files::FileData::from(file_json))),
                        Err(_) => None
                    }
                })
                .collect(),
            maps: maps_json.into_iter()
                .map(|(map_name, map_json)| {
                    let map = map_json.into_iter()
                        .filter_map(|(res_str, formats_strs)| {
                            match parse_resolution(&res_str) {
                                Ok(res) => {
                                    let formats = formats_strs.into_iter()
                                        .map(|(format_str, file_json)| (
                                            files::TextureFormat::from_str(&format_str).unwrap(),
                                            files::FileData::from(file_json)
                                        ))
                                        .collect::<HashMap<_, _>>();
                                    Some((res, formats))
                                },
                                Err(_) => None
                            }
                        })
                        .collect::<HashMap<_, _>>();
                    (files::TextureMap::from_str(map_name).unwrap(), map)
                })
                .collect(),
        }
    }
}

pub type TextureFormat = String;

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ModelFiles {
    pub blend: HashMap<FileResolution, FileData>,
    pub gltf: HashMap<FileResolution, FileData>,
    pub fbx: HashMap<FileResolution, FileData>,

    pub AO: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub arm: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Bump: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Diffuse: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Displacement: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Metal: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub nor_gl: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub Rough: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
    pub spec: Option<HashMap<FileResolution, HashMap<TextureFormat, FileData>>>
}

impl From<ModelFiles> for files::ModelFiles {
    fn from(json: ModelFiles) -> Self {
        let mut maps_json = HashMap::new();
        if let Some(map_json) = json.AO { maps_json.insert("AO", map_json); }
        if let Some(map_json) = json.arm { maps_json.insert("arm", map_json); }
        if let Some(map_json) = json.Bump { maps_json.insert("Bump", map_json); }
        if let Some(map_json) = json.Diffuse { maps_json.insert("Diffuse", map_json); }
        if let Some(map_json) = json.Displacement { maps_json.insert("Displacement", map_json); }
        if let Some(map_json) = json.Metal { maps_json.insert("Metal", map_json); }
        if let Some(map_json) = json.nor_gl { maps_json.insert("nor_gl", map_json); }
        if let Some(map_json) = json.Rough { maps_json.insert("Rough", map_json); }
        if let Some(map_json) = json.spec { maps_json.insert("spec", map_json); }

        Self {
            blend: json.blend.into_iter()
                .filter_map(|(res_str, file_json)| {
                    match parse_resolution(&res_str) {
                        Ok(res) => Some((res, files::FileData::from(file_json))),
                        Err(_) => None
                    }
                })
                .collect(),
            gltf: json.gltf.into_iter()
                .filter_map(|(res_str, file_json)| {
                    match parse_resolution(&res_str) {
                        Ok(res) => Some((res, files::FileData::from(file_json))),
                        Err(_) => None
                    }
                })
                .collect(),
            fbx: json.fbx.into_iter()
                .filter_map(|(res_str, file_json)| {
                    match parse_resolution(&res_str) {
                        Ok(res) => Some((res, files::FileData::from(file_json))),
                        Err(_) => None
                    }
                })
                .collect(),
            maps: maps_json.into_iter()
                .map(|(map_name, map_json)| {
                    let map = map_json.into_iter()
                        .filter_map(|(res_str, formats_strs)| {
                            match parse_resolution(&res_str) {
                                Ok(res) => {
                                    let formats = formats_strs.into_iter()
                                        .map(|(format_str, file_json)| (
                                            files::TextureFormat::from_str(&format_str).unwrap(),
                                            files::FileData::from(file_json)
                                        ))
                                        .collect::<HashMap<_, _>>();
                                    Some((res, formats))
                                },
                                Err(_) => None
                            }
                        })
                        .collect::<HashMap<_, _>>();
                    (files::TextureMap::from_str(map_name).unwrap(), map)
                })
                .collect(),
        }
    }
}