use bevy::prelude::*;

use crate::error::Error;

#[derive(EntityEvent, Debug)]
pub struct ResponseReceived {
    pub entity: Entity,
    pub status: u16,
    // TODO: headers
}

/// Result of `GetContent::BYTES`.
#[derive(EntityEvent, Debug)]
pub struct ResponseBytes {
    pub entity: Entity,
    pub bytes: Vec<u8>,
}

/// Result of `GetContent::TEXT`.
#[derive(EntityEvent, Debug)]
pub struct ResponseText {
    pub entity: Entity,
    pub text: String,
}

#[derive(EntityEvent, Debug)]
pub struct ResponseError {
    pub entity: Entity,
    pub error: Error,
}
