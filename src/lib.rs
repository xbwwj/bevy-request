use async_compat::CompatExt;
use bevy::{prelude::*, tasks::IoTaskPool};

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
#[require(Method)]
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

fn request_start(event: On<Add, Method>, query: Query<(&Uri, &Method)>, tx: Res<Tx>) {
    let request_entity = event.entity;

    let Ok((uri, method)) = query.get(request_entity) else {
        warn!("missing component in request");
        return;
    };
    IoTaskPool::get()
        .spawn({
            let uri = uri.0.clone();
            let method = method.0.clone();
            let tx = tx.0.clone();

            async move {
                let client = reqwest::Client::new();

                let response = client.request(method, uri).send().await;
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
