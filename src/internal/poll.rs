use bevy::prelude::*;

use crate::{
    internal::{channel::Rx, content::ContentGotten},
    prelude::{ResponseBytes, ResponseError, ResponseReceived, ResponseText},
};

use super::channel::Message;

/// Adapter system between background channel and world.
pub(crate) fn poll_responses(rx: ResMut<Rx>, mut commands: Commands) {
    while let Ok((entity, message)) = rx.0.try_recv() {
        match message {
            Message::Response(response) => match response {
                Ok(response) => {
                    let status = response.status().as_u16();
                    commands
                        .entity(entity)
                        .insert(ResponseInternal(Some(response)));
                    commands.trigger(ResponseReceived { entity, status });
                }
                Err(error) => {
                    commands.trigger(ResponseError { entity, error });
                }
            },
            Message::Bytes(bytes) => {
                commands.entity(entity).insert(ContentGotten);
                match bytes {
                    Ok(bytes) => {
                        commands.trigger(ResponseBytes { entity, bytes });
                    }
                    Err(error) => {
                        commands.trigger(ResponseError { entity, error });
                    }
                }
            }
            Message::Text(text) => {
                commands.entity(entity).insert(ContentGotten);
                match text {
                    Ok(text) => {
                        commands.trigger(ResponseText { entity, text });
                    }
                    Err(error) => {
                        commands.trigger(ResponseError { entity, error });
                    }
                }
            }
        }
    }
}

#[derive(Component, Debug)]
pub(crate) struct ResponseInternal(pub Option<reqwest::Response>);
