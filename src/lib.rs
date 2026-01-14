use bevy::prelude::*;

use crate::inner::{Rx, Tx, request_poll, request_start};

mod headers;
mod inner;
mod interface;

pub struct RequestPlugin;

impl Plugin for RequestPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let (tx, rx) = crossbeam_channel::unbounded();

        app.insert_resource(Rx(rx))
            .insert_resource(Tx(tx))
            .add_observer(request_start)
            .add_systems(Update, request_poll);
    }
}

pub mod prelude {
    pub use crate::{RequestPlugin, interface::*};
}
