use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use hexgridspiral as hgs;

use crate::asset_loading::AssetTag;
use crate::components::MainCamera;
use crate::components::TileMarker;
use crate::resources::SelectedHex;
use crate::resources::{TileImageHandles, WorldCoords};

/// Tracks the user's cursor and updates the hovered tile.
/// Also updates tile sprite and transform to highlight selection.
pub fn cursor_system(
    mut coords: ResMut<WorldCoords>,
    tile_image_handles: Res<TileImageHandles>,
    selected_hex: Res<SelectedHex>,
    buttons: Res<ButtonInput<MouseButton>>,
    // query for TileMarkers
    mut q_all_tiles: Query<(&mut TileMarker, &mut Sprite, &mut Transform)>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;

        let scale = 0.25_f64;
        let step_x: f64 = (457. * scale) / (3.0_f64).sqrt(); // 65.9642
        let step_y: f64 = (484. * scale) / 2.; // 60.5

        // log::warn!("World coords: {}/{}", world_position.x, world_position.y);
        let new_hover_tile: hgs::HGSTile = hgs::CCTile::from_irregular_pixel(
            (world_position.y as f64, -world_position.x as f64),
            (0., 0.),
            (step_x, step_y),
        )
        .into();

        let selected_index: hgs::TileIndex = new_hover_tile.spiral_index();
        // log::warn!("Selected Index: {}", selected_index.0);

        // Reset all tiles' colors to plain
        // Except if they are in reachable range
        for (mut tile_marker, mut sprite, mut transform) in q_all_tiles.iter_mut() {
            if selected_index.0 == tile_marker.index.0 {
                // tile_marker.tag = AssetTag::BaseLush;

                // Pop-out effect
                transform.translation.z += 1.0;
                transform.scale = Vec3::splat(1.1);

                // If clicked, apply hex
                if buttons.pressed(MouseButton::Left) {
                    tile_marker.tag = selected_hex.0;
                }
            } else {
                // Reset pop-out effect
                transform.translation.z = transform.translation.y * -0.0001;
                transform.scale = Vec3::ONE;
            }
            if let Some(handles) = tile_image_handles.handles.get(&tile_marker.tag) {
                if let Some(first_handle) = handles.get(0) {
                    sprite.image = first_handle.clone();
                }
            }
        }
    }
}
