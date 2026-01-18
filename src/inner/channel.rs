use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use crate::interface::events::{ResponseBytes, ResponseError, ResponseReceived, ResponseText};

#[derive(Resource, Clone, Debug)]
pub(crate) struct Tx(pub(crate) Sender<ResponseEvent>);

#[derive(Resource, Debug)]
pub(crate) struct Rx(pub(crate) Receiver<ResponseEvent>);

pub(crate) fn init_channel(app: &mut App) {
    let (tx, rx) = crossbeam_channel::unbounded();
    app.insert_resource(Tx(tx)).insert_resource(Rx(rx));
}

pub(crate) enum ResponseEvent {
    Received(ResponseReceived),
    Bytes(ResponseBytes),
    Text(ResponseText),
    Error(ResponseError),
}

impl From<ResponseReceived> for ResponseEvent {
    fn from(value: ResponseReceived) -> Self {
        Self::Received(value)
    }
}

impl From<ResponseBytes> for ResponseEvent {
    fn from(value: ResponseBytes) -> Self {
        Self::Bytes(value)
    }
}

impl From<ResponseText> for ResponseEvent {
    fn from(value: ResponseText) -> Self {
        Self::Text(value)
    }
}

impl From<ResponseError> for ResponseEvent {
    fn from(value: ResponseError) -> Self {
        Self::Error(value)
    }
}
