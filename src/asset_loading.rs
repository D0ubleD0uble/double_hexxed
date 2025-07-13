use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetTag {
    Outline,
    Blank,
    BaseLush,
    BaseOcean,
    BaseSnowy,
    ForestDeciduousLush,
    ForestSnowy,
    HillsDesert,
    HillsLush,
    HillsSnowy,
    MountainPeakRocky,
    MountainPeakLush,
    MountainPeakSnowy,
    MountainMediumRocky,
    MountainLowRocky,
    MountainFoothillsRocky,
    MountainLush,
    MountainSnowy,
    PlainsLush,
    PlainsDesert,
    OceanWaves,
    SwampStill,
    WetlandsDamp,
    SnowField,
    UrbanCityLush,
    // etc...
}

pub fn all_asset_tags() -> Vec<AssetTag> {
    AssetTag::iter().collect()
}

impl AssetTag {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "outline" => AssetTag::Outline,
            "blank (white)" => AssetTag::Blank,
            "forest deciduous, lush" => AssetTag::ForestDeciduousLush,
            "mountain foothills" => AssetTag::MountainFoothillsRocky,
            "mountain low" => AssetTag::MountainLowRocky,
            "mountain medium" => AssetTag::MountainMediumRocky,
            "mountain peak, rocky" => AssetTag::MountainPeakRocky,
            "mountain peak, lush" => AssetTag::MountainPeakLush,
            "mountain peak, snowy" => AssetTag::MountainPeakSnowy,
            "lush plains" => AssetTag::PlainsLush,
            "ocean waves" => AssetTag::OceanWaves,
            "urban city, lush" => AssetTag::UrbanCityLush,
            _ => AssetTag::Blank,
        }
    }

    //pub fn as_str(&self) -> &'static str {
    //    match self {
    //        AssetTag::Outline => "Outline",
    //        AssetTag::Blank => "Erase",
    //        AssetTag::MountainFoothillsRocky => "Mountain Foothills, Rocky",
    //        AssetTag::MountainLowRocky => "Mountain Low, Rocky",
    //        AssetTag::MountainMediumRocky => "Mountain Medium, Rocky",
    //        AssetTag::MountainPeakRocky => "Mountain Peak, Rocky",
    //        AssetTag::MountainPeakLush => "Mountain Peak, Lush",
    //        AssetTag::MountainPeakSnowy => "Mountain Peak, Snowy",
    //        AssetTag::PlainsLush => "Lush Plains",
    //        AssetTag::OceanWaves => "Ocean Waves",
    //        _ => "Unknown",
    //    }
    //}
}

pub fn get_asset_for_tag(tag: AssetTag) -> &'static str {
    match tag {
        AssetTag::BaseLush => "Hex - Base (lush).png",
        AssetTag::PlainsLush => "Hex - Plains (lush) 5.png",
        AssetTag::ForestDeciduousLush => "Hex - Forest, deciduous (lush).png",
        AssetTag::MountainPeakRocky => "Hex - Mountains, peak (rocky).png",
        AssetTag::MountainPeakLush => "Hex - Mountains, peak (lush).png",
        AssetTag::MountainPeakSnowy => "Hex - Mountains, peak (snowy).png",
        AssetTag::MountainMediumRocky => "Hex - Mountains, medium (rocky).png",
        AssetTag::MountainLowRocky => "Hex - Mountains, low (rocky).png",
        AssetTag::MountainFoothillsRocky => "Hex - Mountains, foothills (rocky).png",
        AssetTag::OceanWaves => "Hex - Water - Ocean (waves) 1.png",
        AssetTag::UrbanCityLush => "Hex - Urban - City (lush).png",
        AssetTag::Outline => "Hex - Base (outline).png",
        AssetTag::Blank | _ => "Hex - Base (blank).png",
    }
}

pub fn load_tag(asset_tag: AssetTag, asset_server: &AssetServer) -> Handle<Image> {
    let asset_path = get_asset_for_tag(asset_tag);
    asset_server.load(format!("hextiles_rotated/{}", asset_path))
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum Terrain {
//     Plains,
//     Desert,
//     Aquatic,
//     Mountain,
//     Forest,
//     Swamp,
//     Arctic,
//     Blank,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum Feature {
//     Landmark,
//     Secret,
//     Resource,
//     Standard,
// }

// fn terrain_to_tags(terrain: Terrain) -> Vec<AssetTag> {
//     use Terrain::*;
//     match terrain {
//         Blank => vec![AssetTag::Blank],
//         Plains => vec![AssetTag::PlainsLush, AssetTag::PlainsDesert],
//         Desert => vec![AssetTag::HillsDesert, AssetTag::PlainsDesert],
//         Aquatic => vec![AssetTag::BaseOcean, AssetTag::OceanWaves],
//         Mountain => vec![
//             AssetTag::MountainPeakRocky,
//             AssetTag::MountainLush,
//             AssetTag::MountainSnowy,
//         ],
//         Forest => vec![AssetTag::ForestLush, AssetTag::ForestSnowy],
//         Swamp => vec![AssetTag::SwampStill, AssetTag::WetlandsDamp],
//         Arctic => vec![AssetTag::SnowField, AssetTag::BaseSnowy],
//     }
// }

// pub fn load_assets_for_terrain(terrain: Terrain, asset_server: &AssetServer) -> Vec<Handle<Image>> {
//     let mut asset_handles: Vec<Handle<Image>> = Vec::new();
//     let tags = terrain_to_tags(terrain);
//     for tag in tags {
//         let asset_path = get_asset_for_tag(tag);
//         let handle = asset_server.load(format!("hextiles_rotated/{}", asset_path));
//         asset_handles.push(handle);
//     }
//     return asset_handles;
// }
