mod asset_loading;

use asset_loading::AssetTag;
use asset_loading::Terrain;
use asset_loading::get_assets_for_tag;
use asset_loading::load_assets_for_terrain;
use asset_loading::load_tag;
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use hexgridspiral as hgs;
use std::collections::HashMap;

const LEVELMAP_TILE_CIRCUMRADIUS: f32 = 50.0;
const NUM_TILES: u64 = 61;

fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // fill the entire browser window
                    fit_canvas_to_parent: true,
                    // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                    // prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })),
        )
        .insert_resource(WorldCoords::default())
        .insert_resource(HoveredTile::default())
        //.add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, cursor_system)
        .run();
}

#[derive(Resource)]
pub struct TileImageHandles {
    pub handles: HashMap<AssetTag, Vec<Handle<Image>>>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera with bluewhite background color.
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.7, 0.7, 0.73)),
            ..Default::default()
        },
        MainCamera,
    ));

    // Compute reusable shape
    //let shape = meshes.add(RegularPolygon::new(LEVELMAP_TILE_CIRCUMRADIUS, 6));

    // Load Assets
    let tags_to_load = vec![
        AssetTag::Blank,
        AssetTag::BaseLush,
        AssetTag::PlainsLush,
        AssetTag::MountainRocky,
    ];
    let tag_to_handles: HashMap<AssetTag, Vec<Handle<Image>>> = tags_to_load
        .into_iter()
        .map(|tag| (tag, load_tag(tag, &asset_server)))
        .collect();
    let start_image = get_image_handle(&tag_to_handles, &AssetTag::Blank, 0).unwrap();
    commands.insert_resource(TileImageHandles {
        handles: tag_to_handles,
    });

    // Compute step-size from tile center to tile center, given circumradius of a tile.
    //let tile_inradius = RegularPolygon::new(LEVELMAP_TILE_CIRCUMRADIUS, 6).inradius();
    //let step_size = 2. * tile_inradius + 3.;
    //log::warn!("step_size {} hover", step_size);
    let scale = 0.25 as f64;
    let image_size = ((466. * scale) / (3.0 as f64).sqrt(), (554. * scale) / 2.);
    let step_x: f64 = (457. * scale) / (3.0 as f64).sqrt();
    let step_y: f64 = (484. * scale) / 2.;

    // Spawn tiles
    let blank_handle = for tile_index in 0..NUM_TILES {
        spawn_tile_with_index(
            &mut commands,
            &hgs::TileIndex::from(tile_index),
            image_size,
            (step_x, step_y),
            start_image.clone(),
        );
    };
}

#[derive(Resource, Default)]
struct WorldCoords(Vec2);

#[derive(Resource, Default)]
struct HoveredTile {
    entity: Option<hgs::CCTile>,
}

#[derive(Component)]
struct MainCamera;

fn cursor_system(
    mut coords: ResMut<WorldCoords>,
    mut hovered_tile: ResMut<HoveredTile>,
    tile_image_handles: Res<TileImageHandles>,
    // query for TileMarkers
    mut q_all_tiles: Query<(&TileMarker, &mut Sprite)>,
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
        let scale = 0.25 as f64;
        let step_x: f64 = (457. * scale) / (3.0 as f64).sqrt(); // 65.9642
        let step_y: f64 = (484. * scale) / 2.; // 60.5

        coords.0 = world_position;
        // log::warn!("World coords: {}/{}", world_position.x, world_position.y);
        let new_hover_tile: hgs::HGSTile = hgs::CCTile::from_irregular_pixel(
            (world_position.x as f64, world_position.y as f64),
            (0., 0.),
            (step_x, step_y),
        )
        .into();

        let selected_index: hgs::TileIndex = new_hover_tile.spiral_index();
        // log::warn!("Selected Index: {}", selected_index.0);

        // Reset all tiles' colors to plain blue
        // Except if they are in reachable range
        for (tile_marker, mut sprite) in q_all_tiles.iter_mut() {
            let asset_tag = if selected_index.0 == tile_marker.0.0 {
                &AssetTag::BaseLush
            } else {
                &AssetTag::Blank
            };
            if let Some(handles) = tile_image_handles.handles.get(asset_tag) {
                if let Some(first_handle) = handles.get(0) {
                    sprite.image = first_handle.clone();
                }
            }
        }
    }
}

fn get_image_handle(
    tag_to_handles: &HashMap<AssetTag, Vec<Handle<Image>>>,
    asset_tag: &AssetTag,
    index: usize,
) -> Option<Handle<Image>> {
    tag_to_handles
        .get(asset_tag)
        .and_then(|handles| handles.get(index))
        .cloned()
}

#[derive(Component)]
struct TileMarker(hgs::TileIndex);

// normal function, not a bevy system
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
    let position = Vec3::new(t.0 as f32, t.1 as f32, 0.);

    // Create hexagonal tile with a text as child node
    let mut tile_node = commands.spawn((
        Sprite {
            image: start_image.clone(),
            custom_size: Some(Vec2::new(image_size.0 as f32, image_size.1 as f32)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform::from_translation(position),
        TileMarker(*tile_index),
    ));
    tile_node.with_children(|parent| {
        // Tile Index text node
        parent.spawn((
            Text2d::new(format!("{}", tile_index)),
            TextColor(css::DIM_GRAY.into()),
            // avoid z-fighting. The child transform is relative to the parent.
            Transform::from_xyz(0., 0., 0.0001),
        ));

        //// Cube Coordinates text node
        //let cctile = hgs::CCTile::new(*tile_index);
        //let (q, r, s) = cctile.into_qrs_tuple();
        //let fontsize = 16.;

        //let tl_pos = (hgs::CCTile::unit(&hgs::RingCornerIndex::TOPLEFT))
        //    .to_irregular_pixel((0., 0.), step_size);
        //let r_pos =
        //    (hgs::CCTile::unit(&hgs::RingCornerIndex::RIGHT))
        //    .to_irregular_pixel((0., 0.), step_size);
        //let bl_pos = (hgs::CCTile::unit(&hgs::RingCornerIndex::BOTTOMLEFT))
        //    .to_irregular_pixel((0., 0.), step_size);
        //// only go less than half of the way toward the neightouring tile.
        //let distance = 0.33;

        //parent.spawn((
        //    Text2d::new(format!("{q}")),
        //    TextColor(css::GREEN.into()),
        //    Transform::from_xyz(
        //        distance * tl_pos.0 as f32,
        //        distance * tl_pos.1 as f32,
        //        0.0001,
        //    ),
        //    TextFont {
        //        font_size: fontsize,
        //        ..Default::default()
        //    },
        //));

        //parent.spawn((
        //    Text2d::new(format!("{r}")),
        //    TextColor(css::MEDIUM_TURQUOISE.into()),
        //    Transform::from_xyz(distance * r_pos.0 as f32, distance * r_pos.1 as f32, 0.0001),
        //    TextFont {
        //        font_size: fontsize,
        //        ..Default::default()
        //    },
        //));
        //parent.spawn((
        //    Text2d::new(format!("{s}")),
        //    TextColor(css::DEEP_PINK.into()),
        //    Transform::from_xyz(
        //        distance * bl_pos.0 as f32,
        //        distance * bl_pos.1 as f32,
        //        0.0001,
        //    ),
        //    TextFont {
        //        font_size: fontsize,
        //        ..Default::default()
        //    },
        //));
    });
}

/// Forward the on_click trigger event, with forced order of execution.
fn highlight_on_tile_click(
    trigger: Trigger<Pointer<Click>>,
    mut q_all_tiles: Query<(&TileMarker, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    highlight_movement_range_on_tile_click(&trigger, &mut q_all_tiles, &mut materials);
    highlight_axes_on_tile_click(&trigger, &mut q_all_tiles, &mut materials);
}

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
