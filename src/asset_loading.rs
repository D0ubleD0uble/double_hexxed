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

pub fn get_asset_for_tag(tag: AssetTag) -> &'static str {
    match tag {
        AssetTag::Blank => "Hex - Base (blank).webp",
        AssetTag::Outline => "Hex - Base (outline).webp",
        AssetTag::BaseLush => "Hex - Base (lush).webp",
        AssetTag::BaseOcean => "Hex - Base (ocean).webp",
        AssetTag::BaseRocky => "Hex - Base (rocky).webp",
        AssetTag::BaseSnowy => "Hex - Base (snowy).webp",
        AssetTag::ForestConiferLush => "Hex - Forest, conifer (lush).webp",
        AssetTag::ForestConiferSnowy => "Hex - Forest, conifer (snowy).webp",
        AssetTag::ForestDeciduousLush => "Hex - Forest, deciduous (lush).webp",
        AssetTag::ForestMixedLush => "Hex - Forest, mixed (lush).webp",
        AssetTag::HillsDesert => "Hex - Hills (desert) 1.webp",
        AssetTag::HillsLush => "Hex - Hills (lush) 1.webp",
        AssetTag::HillsSnowy => "Hex - Hills (snowy) 1.webp",
        AssetTag::MountainVolcanoLush => "Hex - Mountain, Volcano (lush) 1.webp",
        AssetTag::MountainVolcanoRocky => "Hex - Mountain, Volcano (rocky) 1.webp",
        AssetTag::MountainVolcanoSnowy => "Hex - Mountain, Volcano (snowy) 1.webp",
        AssetTag::MountainFoothillsLush => "Hex - Mountains, foothills (lush).webp",
        AssetTag::MountainFoothillsRocky => "Hex - Mountains, foothills (rocky).webp",
        AssetTag::MountainFoothillsSnowy => "Hex - Mountains, foothills (snowy).webp",
        AssetTag::MountainLowLush => "Hex - Mountains, low (lush).webp",
        AssetTag::MountainLowRocky => "Hex - Mountains, low (rocky).webp",
        AssetTag::MountainLowSnowy => "Hex - Mountains, low (snowy).webp",
        AssetTag::MountainMediumLush => "Hex - Mountains, medium (lush).webp",
        AssetTag::MountainMediumRocky => "Hex - Mountains, medium (rocky).webp",
        AssetTag::MountainMediumSnowy => "Hex - Mountains, medium (snowy).webp",
        AssetTag::MountainPeakLush => "Hex - Mountains, peak (lush).webp",
        AssetTag::MountainPeakRocky => "Hex - Mountains, peak (rocky).webp",
        AssetTag::MountainPeakSnowy => "Hex - Mountains, peak (snowy).webp",
        AssetTag::PlainsDamp => "Hex - Plains (damp) 1.webp",
        AssetTag::PlainsDesert => "Hex - Plains (desert) 4.webp",
        AssetTag::PlainsFarmland => "Hex - Plains (farmland) 1.webp",
        AssetTag::PlainsLush => "Hex - Plains (lush) 5.webp",
        AssetTag::RuinDesert => "Hex - Ruin (desert).webp",
        AssetTag::RuinLush => "Hex - Ruin (lush).webp",
        AssetTag::SnowArea => "Hex - Snow (area) 1.webp",
        AssetTag::SnowDrifts => "Hex - Snow (drifts) 1.webp",
        AssetTag::SnowField => "Hex - Snow (field) 1.webp",
        AssetTag::SparseTreesLush => "Hex - Sparse Trees (lush) 1.webp",
        AssetTag::SparseTreesSnowy => "Hex - Sparse Trees (snowy).webp",
        AssetTag::UrbanCityLush => "Hex - Urban - City (lush).webp",
        AssetTag::UrbanFarmLush => "Hex - Urban - Farm (lush).webp",
        AssetTag::UrbanFarmlandLush => "Hex - Urban - Farmland (lush) 1.webp",
        AssetTag::UrbanTownAbandoned => "Hex - Urban - Modern Town, abandoned (lush) 1.webp",
        AssetTag::UrbanTownInhabited => "Hex - Urban - Modern Town, inhabited (lush) 1.webp",
        AssetTag::UrbanTownLumberyardLush => "Hex - Urban - Modern Town, lumber yard (lush).webp",
        AssetTag::UrbanMonasteryLush => "Hex - Urban - Monastery (lush).webp",
        AssetTag::UrbanTowerLush => "Hex - Urban - Tower (lush).webp",
        AssetTag::UrbanTownLush => "Hex - Urban - Town (lush).webp",
        AssetTag::OceanSoftWaves => "Hex - Water - Ocean (soft waves) 1.webp",
        AssetTag::OceanStill => "Hex - Water - Ocean (still water) 1.webp",
        AssetTag::OceanWaves => "Hex - Water - Ocean (waves) 1.webp",
        AssetTag::SwampSoftWaves => "Hex - Water - Swamp (soft waves) 1.webp",
        AssetTag::SwampStill => "Hex - Water - Swamp (still water) 1.webp",
        AssetTag::SwanpWaves => "Hex - Water - Swamp (waves) 1.webp",
        AssetTag::WetlandsDamp => "Hex - Wetlands (damp) 1.webp",
        _ => "",
    }
}

pub fn load_tag(asset_tag: AssetTag, asset_server: &AssetServer) -> Handle<Image> {
    let asset_path = get_asset_for_tag(asset_tag);
    asset_server.load(format!("hextiles/{}", asset_path))
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
