use crate::{TOOL_QUEUE, asset_loading::AssetTag, resources::SelectedHex};
use bevy::prelude::*;

/// Event sent when the user selects a new tool from the UI.
#[derive(Event)]
pub struct HexSelectedEvent(pub String);

/// Updates the selected tool in response to a `ToolSelectedEvent`.
pub fn on_hex_selected(
    mut events: EventReader<HexSelectedEvent>,
    mut selected: ResMut<SelectedHex>,
) {
    for event in events.read() {
        log::warn!("Tool selected via event: {}", event.0);
        selected.0 = AssetTag::from_str(&event.0);
    }
}

/// Drains click events from JS queue and injects them into Bevy’s system.
pub fn flush_click_events_system(mut writer: EventWriter<HexSelectedEvent>) {
    if let Ok(mut queue) = TOOL_QUEUE.lock() {
        for event in queue.drain(..) {
            writer.send(event);
        }
    }
}
