use bevy::prelude::*;

use crate::inner::{make_channel, request_poll, request_start};

mod headers;
mod inner;
mod interface;

pub struct RequestPlugin;

impl Plugin for RequestPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let (tx, rx) = make_channel();

        app.insert_resource(tx)
            .insert_resource(rx)
            .add_observer(request_start)
            .add_systems(Update, request_poll);
    }
}

pub mod prelude {
    pub use crate::{RequestPlugin, interface::*};
}
