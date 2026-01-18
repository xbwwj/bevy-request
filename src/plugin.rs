use bevy::prelude::*;

use crate::inner::{channel::init_channel, poll::poll_responses, start::start_requests};

pub struct RequestPlugin;

impl Plugin for RequestPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        init_channel(app);
        app.add_observer(start_requests)
            .add_systems(Update, poll_responses);
    }
}
