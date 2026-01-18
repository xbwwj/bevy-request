use async_compat::CompatExt;
use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    interface::{
        client::{Client, DefaultClient},
        headers::Headers,
        method::Method,
        url::Url,
    },
    internal::channel::Tx,
};

pub(crate) fn start_request(
    // TODO: which component is the most foundamental one?
    // or use the bundle `or` semantic?
    add: On<Add, Method>,
    query: Query<(&Url, &Method, Option<&Headers>, Option<&Client>)>,
    tx: Res<Tx>,
    default_client: Option<Res<DefaultClient>>,
) {
    let entity = add.entity;

    // filter out unrelated
    let Ok((url, method, headers, client)) = query.get(entity) else {
        warn!("missing url in request");
        return;
    };

    let pool = IoTaskPool::get();
    pool.spawn({
        let tx = tx.clone();
        let url = url.0.clone();
        let method = method.0.clone();
        // TODO: avoid clone
        let headers = headers.cloned();

        // client order:
        // 1. `Client` component
        // 2. `DefaultClient` Resource
        // 3. create a new client TODO: should by default reuse
        let client = client
            .map(|c| c.0.clone())
            .or(default_client.map(|r| r.0.clone()))
            .unwrap_or_default();

        async move {
            // build request
            let mut request = client.request(method, url);
            if let Some(headers) = headers {
                request = request.headers(headers.0);
            }

            // await response
            let response = request.send().await;
            tx.send(entity, response);
        }
        .compat()
    })
    .detach();
}
