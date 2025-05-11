use bevy::prelude::*;
use bevy::color::palettes::css;
use hexgridspiral as hgs;

const LEVELMAP_TILE_CIRCUMRADIUS: f32 = 50.0;
const NUM_TILES: u64 = 61;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // fill the entire browser window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                // prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }),
        MeshPickingPlugin))
        //.add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera with bluewhite background color.
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.9, 0.9, 1.0)),
            ..Default::default()
        },
    ));

    //Compute reusable shape
    let shape = meshes.add(RegularPolygon::new(LEVELMAP_TILE_CIRCUMRADIUS, 6));

    // Compute step-size from tile center to tile center, given circumradius of a tile.
    let tile_inradius = RegularPolygon::new(LEVELMAP_TILE_CIRCUMRADIUS, 6).inradius();
    let step_size = 2. * tile_inradius + 3.;

    // Spawn tiles
    for tile_index in 0..NUM_TILES {
        spawn_tile_with_index(
            &mut commands,
            &hgs::TileIndex::from(tile_index),
            step_size,
            &mut materials,
            shape.clone(),
        );
    }
}

#[derive(Component)]
struct TileMarker(hgs::TileIndex);

// normal function, not a bevy system
fn spawn_tile_with_index(
    commands: &mut Commands,
    tile_index: &hgs::TileIndex,
    step_size: f32,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    shape: Handle<Mesh>,
) {
    let t = hgs::HGSTile::new(*tile_index)
        .cc()
        .to_pixel((0., 0.), step_size.into());
    let position = Vec3::new(t.0 as f32, t.1 as f32, 0.);

    // Create blue material that later get modified to green on hover
    let color_blue = Color::hsl(360. * 5. / 8. as f32, 0.95, 0.7);
    let color_handle = materials.add(color_blue);

    // Create hexagonal tile with a text as child node
    let mut tile_node = commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(color_handle.clone()),
        Transform::from_translation(position),
        TileMarker(*tile_index),
    ));
    tile_node.with_children( |parent| {
        // Tile Index text node
        parent.spawn((
            Text2d::new(format!("{}", tile_index)),
            TextColor(css::ALICE_BLUE.into()),
            // avoid z-fighting. The child transform is relative to the parent.
            Transform::from_xyz(0., 0., 0.0001),
        ));

        // Cube Coordinates text node
        let cctile = hgs::CCTile::new(*tile_index);
        let (q, r, s) = cctile.into_qrs_tuple();
        let fontsize = 16.;

        // Compute neighboring tile positions to simplify placement of the text
        let tl_pos = (hgs::CCTile::unit(&hgs::RingCornerIndex::TOPLEFT))
            .to_pixel((0., 0.), step_size as f64);
        let r_pos = (hgs::CCTile::unit(&hgs::RingCornerIndex::RIGHT))
            .to_pixel((0., 0.), step_size as f64);
        let bl_pos = (hgs::CCTile::unit(&hgs::RingCornerIndex::BOTTOMLEFT))
            .to_pixel((0., 0.), step_size as f64);
        // go less than halfway toward the neighboring tile
        let distance = 0.33;

        parent.spawn((
            Text2d::new(format!("{q}")),
            TextColor(css::GREEN.into()),
            Transform::from_xyz(
                distance * tl_pos.0 as f32,
                distance * tl_pos.1 as f32,
                0.0001,
            ),
            TextFont {
                font_size: fontsize,
                ..Default::default()
            },
        ));

        parent.spawn((
            Text2d::new(format!("{r}")),
            TextColor(css::MEDIUM_TURQUOISE.into()),
            Transform::from_xyz(
                distance * r_pos.0 as f32, 
                distance * r_pos.1 as f32, 
                0.0001
            ),
            TextFont {
                font_size: fontsize,
                ..Default::default()
            },
        ));

        parent.spawn((
            Text2d::new(format!("{s}")),
            TextColor(css::DEEP_PINK.into()),
            Transform::from_xyz(
                distance * bl_pos.0 as f32,
                distance * bl_pos.1 as f32,
                0.0001,
            ),
            TextFont {
                font_size: fontsize,
                ..Default::default()
            },
        ));
    });

    // on hover, change color
    let mut color_handle1 = color_handle.clone();
    tile_node.observe(
        move |over: Trigger<Pointer<Over>>,
              mut q: Query<(&TileMarker,)>,
              mut materials: ResMut<Assets<ColorMaterial>>| {
            let (index,) = q
                .get_mut(over.entity())
                .expect("Entity that was hovered over no longer seems to exist...");
            log::debug!("Labelmap Tile {} hover", index.0);
            let color_green = Color::hsl(360. * 4. / 8. as f32, 0.95, 0.7);
            // Assumption that there is always a material associated with the TileMarker
            // Entity.
            let color_material = materials.get_mut(&mut color_handle1).unwrap();
            color_material.color = color_green;
        },
    );

    // on unhover, remove color change
    let mut color_handle2 = color_handle.clone();
    tile_node.observe(
        move |out: Trigger<Pointer<Out>>,
              mut q: Query<(&TileMarker,)>,
              mut materials: ResMut<Assets<ColorMaterial>>| {
            let (index,) = q
                .get_mut(out.entity())
                .expect("Entity that was hovered over no longer seems to exist...");
            log::debug!("Labelmap Tile {} hover", index.0);
            let color_material = materials
                .get_mut(&mut color_handle2)
                .expect("A tile without color?!");
            let color_blue = Color::hsl(360. * 5. / 8. as f32, 0.95, 0.7);
            color_material.color = color_blue;
        },
    );

    tile_node.observe(highlight_on_tile_click);
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