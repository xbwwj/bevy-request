use std::str::FromStr;

use async_compat::CompatExt;
use bevy::{prelude::*, tasks::IoTaskPool};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

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

#[derive(Component, Clone, Debug)]
pub struct Headers(reqwest::header::HeaderMap);

// Header is just a component wrapper around reqwest `HeaderMap`.
impl Headers {
    pub fn new() -> Self {
        Self(HeaderMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn key_len(&self) -> usize {
        self.0.keys_len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    // TODO: trait
    pub fn insert(&mut self, key: &str, val: &str) {
        let key = HeaderName::from_str(key).unwrap();
        let val = HeaderValue::from_str(val).unwrap();
        self.0.insert(key, val);
    }
}

type ERR = (Entity, reqwest::Result<Response>);

#[derive(Resource, Debug)]
struct Tx(crossbeam_channel::Sender<ERR>);

#[derive(Resource, Debug)]
struct Rx(crossbeam_channel::Receiver<ERR>);

#[derive(Component, Clone, Default, Debug)]
#[component(immutable)]
pub struct Method(pub reqwest::Method);

pub const GET: Method = Method(reqwest::Method::GET);
pub const POST: Method = Method(reqwest::Method::POST);
pub const PUT: Method = Method(reqwest::Method::PUT);
pub const DELETE: Method = Method(reqwest::Method::DELETE);
pub const HEAD: Method = Method(reqwest::Method::HEAD);
pub const OPTIONS: Method = Method(reqwest::Method::OPTIONS);
pub const CONNECT: Method = Method(reqwest::Method::CONNECT);
pub const PATCH: Method = Method(reqwest::Method::PATCH);
pub const TRACE: Method = Method(reqwest::Method::TRACE);

#[derive(Component, Clone, Debug)]
#[component(immutable)]
pub struct Uri(pub String);

#[derive(EntityEvent, Debug)]
pub struct RequestComplete {
    entity: Entity,
    result: reqwest::Result<Response>,
}

impl RequestComplete {
    pub fn result(&self) -> &reqwest::Result<Response> {
        &self.result
    }
}

#[derive(Clone, Debug)]
pub struct Response {
    status: u16,
    body: String,
}

impl Response {
    async fn from_reqwest(response: reqwest::Result<reqwest::Response>) -> reqwest::Result<Self> {
        let response = response?;
        let status = response.status().as_u16();
        let body = response.text().await?;
        Ok(Self { status, body })
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

fn request_start(
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

fn request_poll(rx: ResMut<Rx>, mut commands: Commands) {
    while let Ok((entity, result)) = rx.0.try_recv() {
        commands.trigger(RequestComplete { entity, result });
    }
}
