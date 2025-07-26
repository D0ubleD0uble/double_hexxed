use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetTag {
    None,
    Outline,
    Blank,
    BaseLush,
    BaseOcean,
    BaseRocky,
    BaseSnowy,
    ForestConiferLush,
    ForestConiferSnowy,
    ForestDeciduousLush,
    ForestMixedLush,
    HillsDesert,
    HillsLush,
    HillsSnowy,
    MountainVolcanoLush,
    MountainVolcanoRocky,
    MountainVolcanoSnowy,
    MountainFoothillsLush,
    MountainFoothillsRocky,
    MountainFoothillsSnowy,
    MountainLowLush,
    MountainLowRocky,
    MountainLowSnowy,
    MountainMediumLush,
    MountainMediumRocky,
    MountainMediumSnowy,
    MountainPeakLush,
    MountainPeakRocky,
    MountainPeakSnowy,
    PlainsDamp,
    PlainsDesert,
    PlainsFarmland,
    PlainsLush,
    RuinDesert,
    RuinLush,
    SnowArea,
    SnowDrifts,
    SnowField,
    SparseTreesLush,
    SparseTreesSnowy,
    UrbanCityLush,
    UrbanFarmLush,
    UrbanFarmlandLush,
    UrbanTownAbandoned,
    UrbanTownInhabited,
    UrbanTownLumberyardLush,
    UrbanMonasteryLush,
    UrbanTowerLush,
    UrbanTownLush,
    OceanSoftWaves,
    OceanStill,
    OceanWaves,
    SwampSoftWaves,
    SwampStill,
    SwanpWaves,
    WetlandsDamp,
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
            "lush (green)" => AssetTag::BaseLush,
            "ocean (blue)" => AssetTag::BaseOcean,
            "rocky (gray)" => AssetTag::BaseRocky,
            "snowy (off-white)" => AssetTag::BaseSnowy,
            "forest conifer, lush" => AssetTag::ForestConiferLush,
            "forest conifer, snowy" => AssetTag::ForestConiferSnowy,
            "forest deciduous, lush" => AssetTag::ForestDeciduousLush,
            "forest mixed, lush" => AssetTag::ForestMixedLush,
            "hills, desert" => AssetTag::HillsDesert,
            "hills, lush" => AssetTag::HillsLush,
            "hills, snowy" => AssetTag::HillsSnowy,
            "mountain volcano, lush" => AssetTag::MountainVolcanoLush,
            "mountain volcano, rocky" => AssetTag::MountainVolcanoRocky,
            "mountain volcano, snowy" => AssetTag::MountainVolcanoSnowy,
            "mountain foothills, lush" => AssetTag::MountainFoothillsLush,
            "mountain foothills, rocky" => AssetTag::MountainFoothillsRocky,
            "mountain foothills, snowy" => AssetTag::MountainFoothillsSnowy,
            "mountain low, lush" => AssetTag::MountainLowLush,
            "mountain low, rocky" => AssetTag::MountainLowRocky,
            "mountain low, snowy" => AssetTag::MountainLowSnowy,
            "mountain medium, lush" => AssetTag::MountainMediumLush,
            "mountain medium, rocky" => AssetTag::MountainMediumRocky,
            "mountain medium, snowy" => AssetTag::MountainMediumSnowy,
            "mountain peak, lush" => AssetTag::MountainPeakLush,
            "mountain peak, rocky" => AssetTag::MountainPeakRocky,
            "mountain peak, snowy" => AssetTag::MountainPeakSnowy,
            "plains, damp" => AssetTag::PlainsDamp,
            "plains, desert" => AssetTag::PlainsDesert,
            "plains, farmland" => AssetTag::PlainsFarmland,
            "plains, lush" => AssetTag::PlainsLush,
            "ruins, desert" => AssetTag::RuinDesert,
            "ruins, lush" => AssetTag::RuinLush,
            "snow area" => AssetTag::SnowArea,
            "snow drifts" => AssetTag::SnowDrifts,
            "snow field" => AssetTag::SnowField,
            "sparse trees, lush" => AssetTag::SparseTreesLush,
            "sparse trees, snowy" => AssetTag::SparseTreesSnowy,
            "urban city, lush" => AssetTag::UrbanCityLush,
            "urban farm, lush" => AssetTag::UrbanFarmLush,
            "urban farmland, lush" => AssetTag::UrbanFarmlandLush,
            "urban town, abandoned" => AssetTag::UrbanTownAbandoned,
            "urban town, inhabited" => AssetTag::UrbanTownInhabited,
            "urban town lumberyard" => AssetTag::UrbanTownLumberyardLush,
            "urban monastery" => AssetTag::UrbanMonasteryLush,
            "urban tower" => AssetTag::UrbanTowerLush,
            "urban town, lush" => AssetTag::UrbanTownLush,
            "ocean soft waves" => AssetTag::OceanSoftWaves,
            "ocean still water" => AssetTag::OceanStill,
            "ocean waves" => AssetTag::OceanWaves,
            "wetlands, damp" => AssetTag::WetlandsDamp,
            _ => AssetTag::None,
        }
    }
}

pub fn get_asset_for_tag(tag: AssetTag) -> Option<&'static str> {
    match tag {
        AssetTag::Blank => Some("Hex - Base (blank).webp"),
        AssetTag::Outline => Some("Hex - Base (outline).webp"),
        AssetTag::BaseLush => Some("Hex - Base (lush).webp"),
        AssetTag::BaseOcean => Some("Hex - Base (ocean).webp"),
        AssetTag::BaseRocky => Some("Hex - Base (rocky).webp"),
        AssetTag::BaseSnowy => Some("Hex - Base (snowy).webp"),
        AssetTag::ForestConiferLush => Some("Hex - Forest, conifer (lush).webp"),
        AssetTag::ForestConiferSnowy => Some("Hex - Forest, conifer (snowy).webp"),
        AssetTag::ForestDeciduousLush => Some("Hex - Forest, deciduous (lush).webp"),
        AssetTag::ForestMixedLush => Some("Hex - Forest, mixed (lush).webp"),
        AssetTag::HillsDesert => Some("Hex - Hills (desert) 1.webp"),
        AssetTag::HillsLush => Some("Hex - Hills (lush) 1.webp"),
        AssetTag::HillsSnowy => Some("Hex - Hills (snowy) 1.webp"),
        AssetTag::MountainVolcanoLush => Some("Hex - Mountain, Volcano (lush) 1.webp"),
        AssetTag::MountainVolcanoRocky => Some("Hex - Mountain, Volcano (rocky) 1.webp"),
        AssetTag::MountainVolcanoSnowy => Some("Hex - Mountain, Volcano (snowy) 1.webp"),
        AssetTag::MountainFoothillsLush => Some("Hex - Mountains, foothills (lush).webp"),
        AssetTag::MountainFoothillsRocky => Some("Hex - Mountains, foothills (rocky).webp"),
        AssetTag::MountainFoothillsSnowy => Some("Hex - Mountains, foothills (snowy).webp"),
        AssetTag::MountainLowLush => Some("Hex - Mountains, low (lush).webp"),
        AssetTag::MountainLowRocky => Some("Hex - Mountains, low (rocky).webp"),
        AssetTag::MountainLowSnowy => Some("Hex - Mountains, low (snowy).webp"),
        AssetTag::MountainMediumLush => Some("Hex - Mountains, medium (lush).webp"),
        AssetTag::MountainMediumRocky => Some("Hex - Mountains, medium (rocky).webp"),
        AssetTag::MountainMediumSnowy => Some("Hex - Mountains, medium (snowy).webp"),
        AssetTag::MountainPeakLush => Some("Hex - Mountains, peak (lush).webp"),
        AssetTag::MountainPeakRocky => Some("Hex - Mountains, peak (rocky).webp"),
        AssetTag::MountainPeakSnowy => Some("Hex - Mountains, peak (snowy).webp"),
        AssetTag::PlainsDamp => Some("Hex - Plains (damp) 1.webp"),
        AssetTag::PlainsDesert => Some("Hex - Plains (desert) 4.webp"),
        AssetTag::PlainsFarmland => Some("Hex - Plains (farmland) 1.webp"),
        AssetTag::PlainsLush => Some("Hex - Plains (lush) 5.webp"),
        AssetTag::RuinDesert => Some("Hex - Ruin (desert).webp"),
        AssetTag::RuinLush => Some("Hex - Ruin (lush).webp"),
        AssetTag::SnowArea => Some("Hex - Snow (area) 1.webp"),
        AssetTag::SnowDrifts => Some("Hex - Snow (drifts) 1.webp"),
        AssetTag::SnowField => Some("Hex - Snow (field) 1.webp"),
        AssetTag::SparseTreesLush => Some("Hex - Sparse Trees (lush) 1.webp"),
        AssetTag::SparseTreesSnowy => Some("Hex - Sparse Trees (snowy).webp"),
        AssetTag::UrbanCityLush => Some("Hex - Urban - City (lush).webp"),
        AssetTag::UrbanFarmLush => Some("Hex - Urban - Farm (lush).webp"),
        AssetTag::UrbanFarmlandLush => Some("Hex - Urban - Farmland (lush) 1.webp"),
        AssetTag::UrbanTownAbandoned => Some("Hex - Urban - Modern Town, abandoned (lush) 1.webp"),
        AssetTag::UrbanTownInhabited => Some("Hex - Urban - Modern Town, inhabited (lush) 1.webp"),
        AssetTag::UrbanTownLumberyardLush => {
            Some("Hex - Urban - Modern Town, lumber yard (lush).webp")
        }
        AssetTag::UrbanMonasteryLush => Some("Hex - Urban - Monastery (lush).webp"),
        AssetTag::UrbanTowerLush => Some("Hex - Urban - Tower (lush).webp"),
        AssetTag::UrbanTownLush => Some("Hex - Urban - Town (lush).webp"),
        AssetTag::OceanSoftWaves => Some("Hex - Water - Ocean (soft waves) 1.webp"),
        AssetTag::OceanStill => Some("Hex - Water - Ocean (still water) 1.webp"),
        AssetTag::OceanWaves => Some("Hex - Water - Ocean (waves) 1.webp"),
        AssetTag::SwampSoftWaves => Some("Hex - Water - Swamp (soft waves) 1.webp"),
        AssetTag::SwampStill => Some("Hex - Water - Swamp (still water) 1.webp"),
        AssetTag::SwanpWaves => Some("Hex - Water - Swamp (waves) 1.webp"),
        AssetTag::WetlandsDamp => Some("Hex - Wetlands (damp) 1.webp"),
        _ => None,
    }
}

pub fn load_tag(
    asset_tag: AssetTag,
    asset_server: &AssetServer,
    missing_handle: Handle<Image>,
) -> Handle<Image> {
    match get_asset_for_tag(asset_tag) {
        Some(path) => asset_server.load(format!("hextiles/{}", path)),
        None => missing_handle,
    }
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
