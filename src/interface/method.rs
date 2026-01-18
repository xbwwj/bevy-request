use bevy::prelude::*;

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
