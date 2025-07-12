use bevy::input::ButtonInput;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use hexgridspiral as hgs;

use crate::components::MainCamera;
use crate::components::TileMarker;
use crate::resources::SelectedHex;
use crate::resources::{TileImageHandles, WorldCoords};
use crate::systems::setup::spawn_tile_with_index;
use crate::tile_config::image_size;
use crate::tile_config::step_size;

/// Tracks the user's cursor and updates the hovered tile.
/// Also updates tile sprite and transform to highlight selection.
pub fn cursor_system(
    mut coords: ResMut<WorldCoords>,
    tile_image_handles: Res<TileImageHandles>,
    selected_hex: Res<SelectedHex>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut commands: Commands,
    // query for TileMarkers

    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut param_set: ParamSet<(
        // Param 0: Access to camera
        Query<(&Camera, &GlobalTransform, &mut Transform), With<MainCamera>>,
        // Param 1: Access to tiles with Transform
        Query<(&mut TileMarker, &mut Sprite, &mut Transform)>,
    )>,
) {
    // --- Mouse Wheel Zoom ---
    {
        let mut camera_query = param_set.p0();
        let (_, _, mut camera_transform_mut) = camera_query.single_mut();

        let mut zoom_delta = 0.0;
        for ev in mouse_wheel.read() {
            zoom_delta += ev.y * 0.01;
        }
        let scale_factor = 1.1f32.powf(-zoom_delta);
        camera_transform_mut.scale *= Vec3::splat(scale_factor);
        camera_transform_mut.scale = camera_transform_mut
            .scale
            .clamp(Vec3::splat(0.1), Vec3::splat(5.0));
    }
    // --- Keyboard Movement ---
    {
        let mut camera_query = param_set.p0();
        let (_, _, mut camera_transform_mut) = camera_query.single_mut();

        let mut movement = Vec3::ZERO;
        let speed = 500.0; // world units per second
        let delta = time.delta_secs();

        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }
        if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
            movement.y += 1.0;
        }
        if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
            movement.y -= 1.0;
        }
        if movement.length_squared() > 0.0 {
            camera_transform_mut.translation += movement.normalize() * speed * delta;
        }
    }
    // --- Cursor tracking ---
    let world_position = {
        let camera_query = param_set.p0();
        let (camera, camera_global, _) = camera_query.single();
        let window = q_window.single();

        window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_global, cursor).ok())
            .map(|ray| ray.origin.truncate())
    };
    let (camera, camera_global, _) = param_set.p0().single();
    let window = q_window.single();

    if let Some(world_pos) = world_position {
        coords.0 = world_pos;

        let (step_x, step_y) = step_size();
        let (image_w, image_h) = image_size();

        // log::warn!("World coords: {}/{}", world_position.x, world_position.y);
        let new_hover_tile: hgs::HGSTile = hgs::CCTile::from_irregular_pixel(
            (world_pos.y as f64, -world_pos.x as f64),
            (0., 0.),
            (step_x, step_y),
        )
        .into();

        let selected_index: hgs::TileIndex = new_hover_tile.spiral_index();
        let mut missing_tile = true;
        // log::warn!("Selected Index: {}", selected_index.0);

        // --- TILE UPDATE ---
        {
            let mut tile_query = param_set.p1();

            for (mut tile_marker, mut sprite, mut transform) in tile_query.iter_mut() {
                if selected_index.0 == tile_marker.index.0 {
                    missing_tile = false;
                    // tile_marker.tag = AssetTag::BaseLush;

                    // Pop-out effect
                    transform.translation.z += 1.0;
                    transform.scale = Vec3::splat(1.1);

                    // If clicked, apply hex
                    if buttons.pressed(MouseButton::Left) {
                        tile_marker.tag = selected_hex.0;
                    }

                    if let Some(handle) = tile_image_handles.handles.get(&tile_marker.tag) {
                        sprite.image = handle.clone();
                    }
                } else {
                    // Reset pop-out effect
                    transform.translation.z = transform.translation.y * -0.0001;
                    transform.scale = Vec3::ONE;
                }
            }

            // --- Spawn New Tile if Not Found
            if buttons.pressed(MouseButton::Left) && missing_tile {
                if let Some(handle) = tile_image_handles.handles.get(&selected_hex.0) {
                    spawn_tile_with_index(
                        &mut commands,
                        &selected_index,
                        (image_w, image_h),
                        (step_x, step_y),
                        handle.clone(),
                    );
                }
            }
        }
    }
}
