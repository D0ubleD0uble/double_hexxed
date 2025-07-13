use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetTag {
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
            _ => AssetTag::Blank,
        }
    }
}

pub fn get_asset_for_tag(tag: AssetTag) -> &'static str {
    match tag {
        AssetTag::Outline => "Hex - Base (outline).png",
        AssetTag::BaseLush => "Hex - Base (lush).png",
        AssetTag::BaseOcean => "Hex - Base (ocean).png",
        AssetTag::BaseRocky => "Hex - Base (rocky).png",
        AssetTag::BaseSnowy => "Hex - Base (snowy).png",
        AssetTag::ForestConiferLush => "Hex - Forest, conifer (lush).png",
        AssetTag::ForestConiferSnowy => "Hex - Forest, conifer (snowy).png",
        AssetTag::ForestDeciduousLush => "Hex - Forest, deciduous (lush).png",
        AssetTag::ForestMixedLush => "Hex - Forest, mixed (lush).png",
        AssetTag::HillsDesert => "Hex - Hills (desert) 1.png",
        AssetTag::HillsLush => "Hex - Hills (lush) 1.png",
        AssetTag::HillsSnowy => "Hex - Hills (snowy) 1.png",
        AssetTag::MountainVolcanoLush => "Hex - Mountain, Volcano (lush) 1.png",
        AssetTag::MountainVolcanoRocky => "Hex - Mountain, Volcano (rocky) 1.png",
        AssetTag::MountainVolcanoSnowy => "Hex - Mountain, Volcano (snowy) 1.png",
        AssetTag::MountainFoothillsLush => "Hex - Mountains, foothills (lush).png",
        AssetTag::MountainFoothillsRocky => "Hex - Mountains, foothills (rocky).png",
        AssetTag::MountainFoothillsSnowy => "Hex - Mountains, foothills (snowy).png",
        AssetTag::MountainLowLush => "Hex - Mountains, low (lush).png",
        AssetTag::MountainLowRocky => "Hex - Mountains, low (rocky).png",
        AssetTag::MountainLowSnowy => "Hex - Mountains, low (snowy).png",
        AssetTag::MountainMediumLush => "Hex - Mountains, medium (lush).png",
        AssetTag::MountainMediumRocky => "Hex - Mountains, medium (rocky).png",
        AssetTag::MountainMediumSnowy => "Hex - Mountains, medium (snowy).png",
        AssetTag::MountainPeakLush => "Hex - Mountains, peak (lush).png",
        AssetTag::MountainPeakRocky => "Hex - Mountains, peak (rocky).png",
        AssetTag::MountainPeakSnowy => "Hex - Mountains, peak (snowy).png",
        AssetTag::PlainsDamp => "Hex - Plains (damp) 1.png",
        AssetTag::PlainsDesert => "Hex - Plains (desert) 4.png",
        AssetTag::PlainsFarmland => "Hex - Plains (farmland) 1.png",
        AssetTag::PlainsLush => "Hex - Plains (lush) 5.png",
        AssetTag::RuinDesert => "Hex - Ruin (desert).png",
        AssetTag::RuinLush => "Hex - Ruin (lush).png",
        AssetTag::SnowArea => "Hex - Snow (area) 1.png",
        AssetTag::SnowDrifts => "Hex - Snow (drifts) 1.png",
        AssetTag::SnowField => "Hex - Snow (field) 1.png",
        AssetTag::SparseTreesLush => "Hex - Sparse Trees (lush) 1.png",
        AssetTag::SparseTreesSnowy => "Hex - Sparse Trees (snowy).png",
        AssetTag::UrbanCityLush => "Hex - Urban - City (lush).png",
        AssetTag::UrbanFarmLush => "Hex - Urban - Farm (lush).png",
        AssetTag::UrbanFarmlandLush => "Hex - Urban - Farmland (lush) 1.png",
        AssetTag::UrbanTownAbandoned => "Hex - Urban - Modern Town, abandoned (lush) 1.png",
        AssetTag::UrbanTownInhabited => "Hex - Urban - Modern Town, inhabited (lush) 1.png",
        AssetTag::UrbanTownLumberyardLush => "Hex - Urban - Modern Town, lumber yard (lush).png",
        AssetTag::UrbanMonasteryLush => "Hex - Urban - Monastery (lush).png",
        AssetTag::UrbanTowerLush => "Hex - Urban - Tower (lush).png",
        AssetTag::UrbanTownLush => "Hex - Urban - Town (lush).png",
        AssetTag::OceanSoftWaves => "Hex - Water - Ocean (soft waves) 1.png",
        AssetTag::OceanStill => "Hex - Water - Ocean (still water) 1.png",
        AssetTag::OceanWaves => "Hex - Water - Ocean (waves) 1.png",
        AssetTag::SwampSoftWaves => "Hex - Water - Swamp (soft waves) 1.png",
        AssetTag::SwampStill => "Hex - Water - Swamp (still water) 1.png",
        AssetTag::SwanpWaves => "Hex - Water - Swamp (waves) 1.png",
        AssetTag::WetlandsDamp => "Hex - Wetlands (damp) 1.png",
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
