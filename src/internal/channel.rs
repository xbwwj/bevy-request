use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

#[derive(Resource, Clone, Debug)]
pub(crate) struct Tx(pub(crate) Sender<(Entity, Message)>);

impl Tx {
    pub(crate) fn send(&self, entity: Entity, message: impl Into<Message>) {
        if let Err(err) = self.0.send((entity, message.into())) {
            error!("fail to send response event to channel, {err:?}");
        }
    }
}

#[derive(Resource, Debug)]
pub(crate) struct Rx(pub(crate) Receiver<(Entity, Message)>);

pub(crate) fn init_channel_resources(app: &mut App) {
    let (tx, rx) = crossbeam_channel::unbounded();
    app.insert_resource(Tx(tx)).insert_resource(Rx(rx));
}

pub(crate) enum Message {
    Response(reqwest::Result<reqwest::Response>),
    Bytes(reqwest::Result<Vec<u8>>),
    Text(reqwest::Result<String>),
}

impl From<reqwest::Result<reqwest::Response>> for Message {
    fn from(value: reqwest::Result<reqwest::Response>) -> Self {
        Self::Response(value)
    }
}

impl From<reqwest::Result<Vec<u8>>> for Message {
    fn from(value: reqwest::Result<Vec<u8>>) -> Self {
        Self::Bytes(value)
    }
}

impl From<reqwest::Result<String>> for Message {
    fn from(value: reqwest::Result<String>) -> Self {
        Self::Text(value)
    }
}
