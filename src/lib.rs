pub mod error;
mod inner;
pub mod interface;
pub mod plugin;

pub mod prelude {
    pub use crate::{
        interface::{content::*, events::*, method::*, url::*},
        plugin::RequestPlugin,
    };
}
