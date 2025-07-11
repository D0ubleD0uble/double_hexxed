use bevy::prelude::*;
use hexgridspiral as hgs;

use crate::asset_loading::AssetTag;

/// Marker component for the main 2D camera.
#[derive(Component)]
pub struct MainCamera;

/// Marker component attached to each hex tile with its spiral index.
#[derive(Component)]
pub struct TileMarker {
    pub index: hgs::TileIndex,
    pub tag: AssetTag,
}
