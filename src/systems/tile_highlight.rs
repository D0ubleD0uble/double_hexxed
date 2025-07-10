use bevy::prelude::*;
use hexgridspiral as hgs;

use crate::components::TileMarker;

/// Forward the on_click trigger event, with forced order of execution.
pub fn highlight_on_tile_click(
    trigger: Trigger<Pointer<Click>>,
    mut q_all_tiles: Query<(&TileMarker, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    highlight_movement_range_on_tile_click(&trigger, &mut q_all_tiles, &mut materials);
    highlight_axes_on_tile_click(&trigger, &mut q_all_tiles, &mut materials);
}

/// Highlights all tiles in movement range from the clicked tile.
fn highlight_movement_range_on_tile_click(
    trigger: &Trigger<Pointer<Click>>,
    q_all_tiles: &mut Query<(&TileMarker, &mut MeshMaterial2d<ColorMaterial>)>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Check the current tile's index
    let clicked = q_all_tiles.get(trigger.entity());
    let (clicked_tile, clicked_material) = clicked.expect("Nothing was clicked?!");
    let selected_index: hgs::TileIndex = clicked_tile.0;

    // EXAMPLE 2:
    // Compute all reachable tiles, using hexgridspiral.
    let cctile = hgs::CCTile::new(selected_index);
    let movement_range: hgs::MovementRange = cctile.movement_range(2);

    // Reset all tiles' colors to plain blue
    // Except if they are in reachable range
    for (tile_marker, material_handle) in q_all_tiles.iter() {
        let tile_index = tile_marker.0;

        if movement_range.contains(&hgs::CCTile::new(tile_index)) {
            let color_reachable_yellow = Color::hsl(360. * 1. / 8. as f32, 0.95, 0.7);
            materials
                .get_mut(material_handle)
                .expect("A tile without color?!")
                .color = color_reachable_yellow;
        } else {
            let color_plain_blue = Color::hsl(360. * 5. / 8. as f32, 0.95, 0.7);
            materials
                .get_mut(material_handle)
                .expect("A tile without color?!")
                .color = color_plain_blue;
        }
    }

    // Set the clicked tile to yet another color
    let color_clicked_lime = Color::hsl(360. * 2. / 8. as f32, 0.95, 0.7);
    materials
        .get_mut(&clicked_material.0)
        .expect("A tile without color?!")
        .color = color_clicked_lime;
}

fn highlight_axes_on_tile_click(
    trigger: &Trigger<Pointer<Click>>,
    q_all_tiles: &mut Query<(&TileMarker, &mut MeshMaterial2d<ColorMaterial>)>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Check the current tile's index
    let clicked = q_all_tiles.get(trigger.entity());
    let (clicked_tile, _clicked_material) = clicked.expect("Nothing was clicked?!");
    let selected_index: hgs::TileIndex = clicked_tile.0;

    // Compute the CubeCoordinates
    let cctile = hgs::CCTile::new(selected_index);
    let (q, r, s) = cctile.into_qrs_tuple();

    // Reset all tiles' colors to plain blue: Happened already in
    // highlight_movement_range_on_tile_click.

    // Set all colors to a darker hue if they are reachable in a straight line from the
    // clicked tile.
    for (tile_marker, material_handle) in q_all_tiles.iter() {
        let (qq, rr, ss) = hgs::CCTile::new(tile_marker.0).into_qrs_tuple();

        // EXAMPLE 3:
        if qq == q || rr == r || ss == s {
            let mat = materials
                .get_mut(material_handle)
                .expect("A tile without color?!");

            let previous_color: bevy::prelude::Laba = mat.color.into();
            let adjusted_color = bevy::prelude::Laba {
                lightness: 0.5,
                ..previous_color
            };
            mat.color = bevy::prelude::Color::Laba(adjusted_color);
        }
    }
}
