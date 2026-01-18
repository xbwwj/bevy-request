use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
#[component(immutable)]
pub struct Url(pub String);
