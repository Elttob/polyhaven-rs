use std::{collections::HashMap, str::FromStr, convert::Infallible};

#[derive(Debug)]
pub struct FileData {
    pub url: Option<String>,
    pub md5: String,
    pub size: u64,
    pub include: HashMap<String, FileData>,
}

type FileResolution = u64;

#[derive(Debug)]
pub enum Files {
    HDRI(HDRIFiles),
    Texture(TextureFiles),
    Model(ModelFiles)
}

#[derive(Debug)]
pub struct HDRIFiles {
    pub hdri: HashMap<FileResolution, HashMap<HDRIFormat, FileData>>,
    pub backplates: HashMap<String, HashMap<HDRIBackplateFormat, FileData>>,
    pub colorchart: Option<FileData>,
    pub tonemapped: Option<FileData>
} 

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum HDRIFormat {
    Hdr,
    Exr,
    Unparsed(String)
}

impl FromStr for HDRIFormat {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "hdr" => Self::Hdr,
            "exr" => Self::Exr,
            _ => Self::Unparsed(s.to_string())
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum HDRIBackplateFormat {
    JpgPretty,
    JpgPlain,
    Raw,
    Unparsed(String)
}

impl FromStr for HDRIBackplateFormat {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "jpg_pretty" => Self::JpgPretty,
            "jpg_plain" => Self::JpgPlain,
            "raw" => Self::Raw,
            _ => Self::Unparsed(s.to_string())
        })
    }
}

#[derive(Debug)]
pub struct TextureFiles {
    pub blend: HashMap<FileResolution, FileData>,
    pub gltf: HashMap<FileResolution, FileData>,
    pub maps: HashMap<TextureMap, HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TextureMap {
    AO,
    ARM,
    Bump,
    Diffuse,
    Displacement,
    Metal,
    NorGL,
    Rough,
    Spec,
    Unparsed(String)
}

impl FromStr for TextureMap {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "ao" => Self::AO,
            "arm" => Self::ARM,
            "bump" => Self::Bump,
            "diffuse" => Self::Diffuse,
            "displacement" => Self::Displacement,
            "metal" => Self::Metal,
            "nor_gl" => Self::NorGL,
            "rough" => Self::Rough,
            "spec" => Self::Spec,
            _ => Self::Unparsed(s.to_string())
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TextureFormat {
    Exr,
    Jpg,
    Png,
    Unparsed(String)
}

impl FromStr for TextureFormat {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "exr" => Self::Exr,
            "jpg" => Self::Jpg,
            "png" => Self::Png,
            _ => Self::Unparsed(s.to_string())
        })
    }
}

#[derive(Debug)]
pub struct ModelFiles {
    pub blend: HashMap<FileResolution, FileData>,
    pub gltf: HashMap<FileResolution, FileData>,
    pub fbx: HashMap<FileResolution, FileData>,
    pub maps: HashMap<TextureMap, HashMap<FileResolution, HashMap<TextureFormat, FileData>>>,
}