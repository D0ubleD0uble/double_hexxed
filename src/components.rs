use bevy::prelude::*;
use hexgridspiral as hgs;

/// Marker component for the main 2D camera.
#[derive(Component)]
pub struct MainCamera;

/// Marker component attached to each hex tile with its spiral index.
#[derive(Component)]
pub struct TileMarker(pub(crate) hgs::TileIndex);
