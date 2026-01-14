//! This module is the inner system

use async_compat::CompatExt;
use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    headers::Headers,
    interface::{Method, RequestComplete, Response, Uri},
};

/// Abbreviation for channel message type.
pub(crate) type ERR = (Entity, reqwest::Result<Response>);

#[derive(Resource, Debug)]
pub(crate) struct Tx(pub(crate) crossbeam_channel::Sender<ERR>);

#[derive(Resource, Debug)]
pub(crate) struct Rx(pub(crate) crossbeam_channel::Receiver<ERR>);

pub(crate) fn request_start(
    event: On<Add, Method>,
    query: Query<(&Uri, &Method, Option<&Headers>)>,
    tx: Res<Tx>,
) {
    let request_entity = event.entity;

    let Ok((uri, method, headers)) = query.get(request_entity) else {
        warn!("missing component in request");
        return;
    };

    IoTaskPool::get()
        .spawn({
            let uri = uri.0.clone();
            let method = method.0.clone();
            let tx = tx.0.clone();
            // PERF: avoid clone
            let headers = headers.cloned();

            async move {
                let client = reqwest::Client::new();

                let mut request = client.request(method, uri);

                if let Some(headers) = headers {
                    request = request.headers(headers.0);
                }

                let response = request.send().await;
                let response = Response::from_reqwest(response).await;
                tx.send((request_entity, response))
                    // TODO: better error handling for
                    .expect("fail to send");
            }
            .compat()
        })
        .detach();
}

pub(crate) fn request_poll(rx: ResMut<Rx>, mut commands: Commands) {
    while let Ok((entity, result)) = rx.0.try_recv() {
        commands.trigger(RequestComplete { entity, result });
    }
}
