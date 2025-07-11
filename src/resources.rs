use crate::asset_loading::AssetTag;
use bevy::prelude::*;
use hexgridspiral as hgs;
use std::collections::HashMap;

pub const NUM_TILES: u64 = 61;

/// Stores handles to tile image assets.
#[derive(Resource)]
pub struct TileImageHandles {
    pub handles: HashMap<AssetTag, Vec<Handle<Image>>>,
}

/// Tracks the user’s selected tool.
#[derive(Resource)]
pub struct SelectedHex(pub AssetTag);

/// Stores current world cursor position.
#[derive(Resource, Default)]
pub struct WorldCoords(pub(crate) Vec2);

/// Tracks the currently hovered tile entity.
#[derive(Resource, Default)]
pub struct HoveredTile {
    entity: Option<hgs::CCTile>,
}
