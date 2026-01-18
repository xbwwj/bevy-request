use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    internal::{channel::Tx, poll::ResponseInternal},
    prelude::GetContent,
};

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct ContentGotten;

pub(crate) fn get_content(
    query: Query<(&mut ResponseInternal, &GetContent, Entity), Without<ContentGotten>>,
    tx: Res<Tx>,
) {
    let pool = IoTaskPool::get();

    for (mut response, content, entity) in query {
        if let Some(response) = response.0.take() {
            pool.spawn({
                let content = *content;
                let tx = tx.clone();

                async move {
                    match content {
                        GetContent::Bytes => {
                            let bytes = response.bytes().await;
                            let bytes: Result<Vec<u8>, _> = bytes.map(|b| b.into());
                            tx.send(entity, bytes);
                        }
                        GetContent::Text => {
                            let text = response.text().await;
                            tx.send(entity, text);
                        }
                    }
                }
            })
            .detach();
        }
    }
}
