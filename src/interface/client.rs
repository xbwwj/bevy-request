use bevy::prelude::*;

/// Allow users to configure default client for http request.cloned
///
/// Note that proxy is incompatible with default client.
///
/// TODO: better default client with cached options
#[derive(Resource, Default, Clone, Debug)]
pub struct DefaultClient(pub reqwest::Client);

// pub enum DefaultClient {
// cache for same options
//     Cached,
// user specified
//     Specified
// }

#[derive(Component, Default, Clone, Debug)]
pub struct Client(pub reqwest::Client);
