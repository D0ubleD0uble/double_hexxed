use std::mem;

use bevy::prelude::*;

use crate::{LABEL_QUEUE, resources::ShowTileLabels};

/// An event to toggle whether tile labels should be visible.
#[derive(Event)]
pub struct ToggleTileLabelsEvent;

pub fn flush_tile_label_toggle_queue(mut event_writer: EventWriter<ToggleTileLabelsEvent>) {
    let mut queue = LABEL_QUEUE.lock().unwrap();
    if !queue.is_empty() {
        let events = mem::take(&mut *queue);
        for _ in events {
            event_writer.send(ToggleTileLabelsEvent);
        }
    }
}

pub fn handle_toggle_tile_labels_event(
    mut event_reader: EventReader<ToggleTileLabelsEvent>,
    mut show_labels: ResMut<ShowTileLabels>,
) {
    for _ in event_reader.read() {
        show_labels.0 = !show_labels.0;
        info!("ShowTileLabels toggled to {}", show_labels.0);
    }
}

pub fn toggle_tile_labels_system(
    show_labels: Res<ShowTileLabels>,
    mut label_query: Query<&mut Visibility, With<Text2d>>,
) {
    for mut visibility in &mut label_query {
        *visibility = if show_labels.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
