mod asset_loading;
mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use once_cell::sync::Lazy;
use resources::{HoveredTile, SelectedTool, WorldCoords};
use std::sync::Mutex;
use systems::{
    cursor::cursor_system,
    setup::setup,
    tools::{ToolSelectedEvent, flush_tool_events_system, on_tool_selected},
};
use wasm_bindgen::prelude::*;

/// Global queue used to forward tool events from JavaScript to Bevy.
pub static TOOL_QUEUE: Lazy<Mutex<Vec<ToolSelectedEvent>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// External JavaScript function used for logging (when compiled to WebAssembly).
#[wasm_bindgen]
extern "C" {
    fn log(s: &str);
}

/// Called from JavaScript to set the current tool.
#[wasm_bindgen]
pub fn set_tool(tool: &str) {
    log::warn!("Tool selected in Rust: {}", tool);
    let event = ToolSelectedEvent(tool.to_string());
    TOOL_QUEUE.lock().unwrap().push(event);
}

fn main() {
    let mut app = App::new()
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
        .add_event::<ToolSelectedEvent>()
        .insert_resource(WorldCoords::default())
        .insert_resource(HoveredTile::default())
        .insert_resource(SelectedTool("Erase".into()))
        .add_systems(Startup, setup)
        .add_systems(Update, flush_tool_events_system)
        .add_systems(Update, on_tool_selected)
        .add_systems(Update, cursor_system)
        .run();
}
