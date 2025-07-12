use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Terrain {
    Plains,
    Desert,
    Aquatic,
    Mountain,
    Forest,
    Swamp,
    Arctic,
    Blank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    Landmark,
    Secret,
    Resource,
    Standard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetTag {
    Blank,
    BaseLush,
    BaseOcean,
    BaseSnowy,
    ForestLush,
    ForestSnowy,
    HillsDesert,
    HillsLush,
    HillsSnowy,
    MountainPeak,
    MountainMedium,
    MountainLow,
    MountainFoothills,
    MountainLush,
    MountainSnowy,
    PlainsLush,
    PlainsDesert,
    OceanWaves,
    SwampStill,
    WetlandsDamp,
    SnowField,
    // etc...
}

impl AssetTag {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "erase" => AssetTag::Blank,
            "mountain foothills" => AssetTag::MountainFoothills,
            "mountain low" => AssetTag::MountainLow,
            "mountain medium" => AssetTag::MountainMedium,
            "mountain peak" => AssetTag::MountainPeak,
            "lush plains" => AssetTag::PlainsLush,
            "ocean waves" => AssetTag::OceanWaves,
            _ => AssetTag::Blank,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AssetTag::Blank => "Erase",
            AssetTag::MountainFoothills => "Mountain Foothills",
            AssetTag::MountainLow => "Mountain Low",
            AssetTag::MountainMedium => "Mountain Medium",
            AssetTag::MountainPeak => "Mountain Peak",
            AssetTag::PlainsLush => "Lush Plains",
            AssetTag::OceanWaves => "Ocean Waves",
            _ => "Unknown",
        }
    }
}

fn terrain_to_tags(terrain: Terrain) -> Vec<AssetTag> {
    use Terrain::*;
    match terrain {
        Blank => vec![AssetTag::Blank],
        Plains => vec![AssetTag::PlainsLush, AssetTag::PlainsDesert],
        Desert => vec![AssetTag::HillsDesert, AssetTag::PlainsDesert],
        Aquatic => vec![AssetTag::BaseOcean, AssetTag::OceanWaves],
        Mountain => vec![
            AssetTag::MountainPeak,
            AssetTag::MountainLush,
            AssetTag::MountainSnowy,
        ],
        Forest => vec![AssetTag::ForestLush, AssetTag::ForestSnowy],
        Swamp => vec![AssetTag::SwampStill, AssetTag::WetlandsDamp],
        Arctic => vec![AssetTag::SnowField, AssetTag::BaseSnowy],
    }
}

pub fn get_assets_for_tag(tag: AssetTag) -> Vec<&'static str> {
    match tag {
        AssetTag::BaseLush => vec!["Hex - Base (lush).png"],
        AssetTag::PlainsLush => vec![
            //"Hex - Plains (lush) 1.png",
            //"Hex - Plains (lush) 2.png",
            //"Hex - Plains (lush) 3.png",
            //"Hex - Plains (lush) 4.png",
            "Hex - Plains (lush) 5.png",
        ],
        AssetTag::MountainPeak => vec![
            //"Hex - Mountains, foothills (rocky).png",
            //"Hex - Mountains, low (rocky).png",
            //"Hex - Mountains, medium (rocky).png",
            "Hex - Mountains, peak (rocky).png",
        ],
        AssetTag::MountainMedium => vec!["Hex - Mountains, medium (rocky).png"],
        AssetTag::MountainLow => vec!["Hex - Mountains, low (rocky).png"],
        AssetTag::MountainFoothills => vec!["Hex - Mountains, foothills (rocky).png"],
        AssetTag::OceanWaves => vec!["Hex - Water - Ocean (waves) 1.png"],
        AssetTag::Blank | _ => vec!["Hex - Base (blank).png"],
    }
}

pub fn load_assets_for_terrain(terrain: Terrain, asset_server: &AssetServer) -> Vec<Handle<Image>> {
    let mut asset_handles: Vec<Handle<Image>> = Vec::new();
    let tags = terrain_to_tags(terrain);
    for tag in tags {
        let asset_paths = get_assets_for_tag(tag);
        for path in asset_paths {
            let handle = asset_server.load(format!("hextiles_rotated/{}", path));
            asset_handles.push(handle);
        }
    }
    return asset_handles;
}

pub fn load_tag(asset_tag: AssetTag, asset_server: &AssetServer) -> Vec<Handle<Image>> {
    get_assets_for_tag(asset_tag)
        .iter()
        .map(|asset_path| asset_server.load(format!("hextiles_rotated/{}", asset_path)))
        .collect()
}
