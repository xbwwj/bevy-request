//! This module is the inner system

use async_compat::CompatExt;
use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    headers::Headers,
    interface::{Method, RequestComplete, Response, Uri},
    prelude::ResponseReceived,
};

enum RequestEvent {
    Received(ResponseReceived),
    Complete(RequestComplete),
}

#[derive(Resource, Debug)]
pub(crate) struct Tx(crossbeam_channel::Sender<RequestEvent>);

#[derive(Resource, Debug)]
pub(crate) struct Rx(crossbeam_channel::Receiver<RequestEvent>);

pub(crate) fn make_channel() -> (Tx, Rx) {
    let (tx, rx) = crossbeam_channel::unbounded();
    (Tx(tx), Rx(rx))
}

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

                let response = match request.send().await {
                    Ok(response) => response,
                    Err(err) => {
                        tx.send(RequestEvent::Received(ResponseReceived {
                            entity: request_entity,
                            result: Err(err),
                        }))
                        .expect("fail to send");
                        return;
                    }
                };
                let status = response.status().as_u16();
                tx.send(RequestEvent::Received(ResponseReceived {
                    entity: request_entity,
                    result: Ok(status),
                }))
                .expect("fail to send");

                let text = response.text().await;
                match text {
                    Ok(text) => tx.send(RequestEvent::Complete(RequestComplete {
                        entity: request_entity,
                        result: Ok(Response { status, body: text }),
                    })),
                    Err(err) => tx.send(RequestEvent::Complete(RequestComplete {
                        entity: request_entity,
                        result: Err(err),
                    })),
                }
                // TODO: better error handling for
                .expect("fail to send");
            }
            .compat()
        })
        .detach();
}

pub(crate) fn request_poll(rx: ResMut<Rx>, mut commands: Commands) {
    while let Ok(event) = rx.0.try_recv() {
        match event {
            RequestEvent::Received(event) => commands.trigger(event),
            RequestEvent::Complete(event) => commands.trigger(event),
        }
    }
}
