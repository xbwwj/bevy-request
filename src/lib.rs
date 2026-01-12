use std::ops::{Deref, DerefMut};

use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::EntityEvent,
        lifecycle::Add,
        observer::On,
        system::{Commands, Query},
    },
    tasks::{IoTaskPool, Task, futures::check_ready},
};
use blocking::unblock;
use ureq::http;

pub struct RequestPlugin;

impl Plugin for RequestPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_observer(request_start)
            .add_systems(Update, request_poll);
    }
}

#[derive(Component, Debug, Default)]
#[component(immutable)]
pub struct Method(pub http::Method);

pub const GET: Method = Method(http::Method::GET);
pub const POST: Method = Method(http::Method::POST);
pub const PUT: Method = Method(http::Method::PUT);
pub const DELETE: Method = Method(http::Method::DELETE);
pub const HEAD: Method = Method(http::Method::HEAD);
pub const OPTIONS: Method = Method(http::Method::OPTIONS);
pub const CONNECT: Method = Method(http::Method::CONNECT);
pub const PATCH: Method = Method(http::Method::PATCH);
pub const TRACE: Method = Method(http::Method::TRACE);

#[derive(Component, Debug)]
#[require(Method, Agent)]
#[component(immutable)]
pub struct Uri(pub http::Uri);

#[derive(Component, Debug)]
pub struct Agent(pub ureq::Agent);

impl Default for Agent {
    fn default() -> Self {
        Self(ureq::agent())
    }
}

#[derive(EntityEvent, Debug)]
pub struct Response {
    entity: Entity,
    pub result: Result<http::Response<ureq::Body>, ureq::Error>,
}

impl Deref for Response {
    type Target = Result<http::Response<ureq::Body>, ureq::Error>;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl DerefMut for Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.result
    }
}

fn request_start(
    event: On<Add, Method>,
    mut commands: Commands,
    query: Query<(&Uri, &Method, &Agent)>,
) {
    let request_entity = event.entity;

    if let Ok((uri, method, agent)) = query.get(request_entity) {
        let pool = IoTaskPool::get();

        let uri = uri.0.clone();
        let method = method.0.clone();
        let agent = agent.0.clone();

        let task = pool.spawn(async move {
            unblock(move || {
                http::Request::builder()
                    .uri(uri)
                    .method(method)
                    .body(())
                    .map_err(ureq::Error::Http)
                    .and_then(|request| agent.run(request))
            })
            .await
        });

        commands.spawn(RequestPoll(request_entity, task));
    } else {
        println!("false positive");
    }
}

fn request_poll(mut commands: Commands, query: Query<(Entity, &mut RequestPoll)>) {
    for (poll_entity, mut poll) in query {
        if let Some(result) = check_ready(&mut poll.1) {
            commands.trigger(Response {
                entity: poll.0,
                result,
            });
            commands.entity(poll_entity).despawn();
        }
    }
}

#[derive(Component, Debug)]
struct RequestPoll(
    /// Entity of request.
    Entity,
    Task<Result<http::Response<ureq::Body>, ureq::Error>>,
);
