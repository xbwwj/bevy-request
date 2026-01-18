use bevy::prelude::*;

use crate::inner::channel::Rx;

use super::channel::ResponseEvent;

/// Adapter system between background channel and world.
pub(crate) fn poll_responses(rx: ResMut<Rx>, mut commands: Commands) {
    while let Ok(event) = rx.0.try_recv() {
        match event {
            ResponseEvent::Received(e) => commands.trigger(e),
            ResponseEvent::Bytes(e) => commands.trigger(e),
            ResponseEvent::Text(e) => commands.trigger(e),
            ResponseEvent::Error(e) => commands.trigger(e),
        }
    }
}
