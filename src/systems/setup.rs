use bevy::color::palettes::css;
use bevy::prelude::*;
use hexgridspiral as hgs;
use std::collections::HashMap;

use crate::asset_loading::{AssetTag, load_tag};
use crate::components::{MainCamera, TileMarker};
use crate::resources::{NUM_TILES, TileImageHandles};

/// Bevy startup system: sets up the 2D camera, loads assets, and spawns all tiles.
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn main camera with a blue-gray background color.
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.7, 0.7, 0.73)),
            ..Default::default()
        },
        MainCamera,
    ));

    // Load images for several asset tags.
    let tags_to_load = vec![
        AssetTag::Blank,
        AssetTag::BaseLush,
        AssetTag::PlainsLush,
        AssetTag::MountainFoothillsRocky,
        AssetTag::MountainLowRocky,
        AssetTag::MountainMediumRocky,
        AssetTag::MountainPeakRocky,
        AssetTag::OceanWaves,
    ];

    let tag_to_handles: HashMap<AssetTag, Handle<Image>> = tags_to_load
        .into_iter()
        .map(|tag| (tag, load_tag(tag, &asset_server)))
        .collect();

    let start_image = get_image_handle(&tag_to_handles, &AssetTag::Blank, 0)
        .expect("Failed to load default image");

    commands.insert_resource(TileImageHandles {
        handles: tag_to_handles,
    });

    let scale = 0.25;
    let image_size = (
        (466. * scale * 2.0) / (3.0 as f64).sqrt(),
        (554. * scale * 2.0) / 2.,
    );
    let step_x = (457. * scale) / (3.0_f64).sqrt();
    let step_y = (484. * scale) / 2.;

    // Spawn tiles
    for tile_index in 0..NUM_TILES {
        spawn_tile_with_index(
            &mut commands,
            &hgs::TileIndex::from(tile_index),
            image_size,
            (step_x, step_y),
            start_image.clone(),
        );
    }
}

/// Extracts an image handle from the asset map for a given tag and index.
fn get_image_handle(
    tag_to_handles: &HashMap<AssetTag, Handle<Image>>,
    asset_tag: &AssetTag,
    index: usize,
) -> Option<Handle<Image>> {
    tag_to_handles.get(asset_tag).cloned()
}

/// Spawns a tile entity (sprite + text) at a given hex grid location.
fn spawn_tile_with_index(
    commands: &mut Commands,
    tile_index: &hgs::TileIndex,
    image_size: (f64, f64),
    step_size: (f64, f64),
    start_image: Handle<Image>,
) {
    let t = hgs::HGSTile::new(*tile_index)
        .cc()
        .to_irregular_pixel((0., 0.), step_size);
    let (x, y, z) = (t.0 as f32, t.1 as f32, (t.0 as f32) * -0.0001);

    // Rotate position 90° CCW around origin (0, 0)
    let rotated_position = Vec3::new(-y, x, z);
    let rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
    let child_rotation = rotation.inverse();

    // Create hexagonal tile with a text as child node
    let mut tile_node = commands.spawn((
        Sprite {
            image: start_image.clone(),
            custom_size: Some(Vec2::new(image_size.0 as f32, image_size.1 as f32)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform {
            translation: rotated_position,
            rotation,
            ..default()
        },
        TileMarker {
            index: *tile_index,
            tag: AssetTag::Blank,
        },
    ));

    tile_node.with_children(|parent| {
        // Tile Index text node
        parent.spawn((
            Text2d::new(format!("{}", tile_index)),
            TextColor(css::DIM_GRAY.into()),
            // avoid z-fighting. The child transform is relative to the parent.
            Transform {
                translation: Vec3::new(0., 0., 0.0001),
                rotation: child_rotation,
                ..default()
            },
        ));
    });
}
