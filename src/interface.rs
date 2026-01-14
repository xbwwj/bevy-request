use bevy::prelude::*;

pub use crate::headers::*;

#[derive(Clone, Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

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
    pub entity: Entity,
    pub result: reqwest::Result<Response>,
}

impl RequestComplete {
    pub fn result(&self) -> &reqwest::Result<Response> {
        &self.result
    }
}

#[derive(EntityEvent, Debug)]
pub struct ResponseReceived {
    pub entity: Entity,
    pub result: reqwest::Result<u16>,
}
