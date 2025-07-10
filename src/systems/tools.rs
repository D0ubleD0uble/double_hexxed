use crate::{TOOL_QUEUE, resources::SelectedTool};
use bevy::prelude::*;

/// Event sent when the user selects a new tool from the UI.
#[derive(Event)]
pub struct ToolSelectedEvent(pub String);

/// Updates the selected tool in response to a `ToolSelectedEvent`.
pub fn on_tool_selected(
    mut events: EventReader<ToolSelectedEvent>,
    mut selected: ResMut<SelectedTool>,
) {
    for event in events.read() {
        log::warn!("Tool selected via event: {}", event.0);
        selected.0 = event.0.clone(); // Update resource if needed
    }
}

/// Drains tool events from JS queue and injects them into Bevy’s system.
pub fn flush_tool_events_system(mut writer: EventWriter<ToolSelectedEvent>) {
    if let Ok(mut queue) = TOOL_QUEUE.lock() {
        for event in queue.drain(..) {
            writer.send(event);
        }
    }
}
