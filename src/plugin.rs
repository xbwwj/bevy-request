use bevy::prelude::*;

use crate::internal::{
    channel::init_channel_resources, content::get_content, poll::poll_responses,
    start::start_request,
};

pub struct RequestPlugin;

impl Plugin for RequestPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        init_channel_resources(app);

        app.add_observer(start_request)
            .add_systems(Update, (poll_responses, get_content).chain());
    }
}
