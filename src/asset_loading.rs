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
    MountainRocky,
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

fn terrain_to_tags(terrain: Terrain) -> Vec<AssetTag> {
    use Terrain::*;
    match terrain {
        Blank => vec![AssetTag::Blank],
        Plains => vec![AssetTag::PlainsLush, AssetTag::PlainsDesert],
        Desert => vec![AssetTag::HillsDesert, AssetTag::PlainsDesert],
        Aquatic => vec![AssetTag::BaseOcean, AssetTag::OceanWaves],
        Mountain => vec![
            AssetTag::MountainRocky,
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
        AssetTag::BaseLush => vec![
            "Hex - Base (lush) rotated.png"
        ],
        AssetTag::PlainsLush => vec![
            "Hex - Plains (lush) 1.png",
            "Hex - Plains (lush) 2.png",
            "Hex - Plains (lush) 3.png",
            "Hex - Plains (lush) 4.png",
            "Hex - Plains (lush) 5.png",
        ],
        AssetTag::MountainRocky => vec![
            "Hex - Mountains, foothills (rocky).png",
            "Hex - Mountains, low (rocky).png",
            "Hex - Mountains, medium (rocky).png",
            "Hex - Mountains, peak (rocky).png",
        ],
        _ => vec!["Hex - Base (blank) rotated.png"],
    }
}

pub fn load_assets_for_terrain(
    terrain: Terrain,
    asset_server: &AssetServer,
) -> Vec<Handle<Image>> {
    let mut asset_handles: Vec<Handle<Image>> = Vec::new();
    let tags = terrain_to_tags(terrain);
    for tag in tags {
        let asset_paths = get_assets_for_tag(tag);
        for path in asset_paths {
            let handle = asset_server.load(format!("hextiles_cropped/{}", path));
            asset_handles.push(handle);
        }
    }
    return asset_handles;
}

pub fn load_tag(
    asset_tag: AssetTag,
    asset_server: &AssetServer,
) -> Handle<Image> {
    let asset_path = get_assets_for_tag(asset_tag)[0];
    let handle = asset_server.load(format!("hextiles_cropped/{}", asset_path));
    return handle
}