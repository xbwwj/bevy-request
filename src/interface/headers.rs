use std::str::FromStr;

use bevy::prelude::*;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Component, Clone, Debug)]
pub struct Headers(pub(crate) reqwest::header::HeaderMap);

// Header is just a component wrapper around reqwest `HeaderMap`.
impl Headers {
    pub fn new() -> Self {
        Self(HeaderMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn key_len(&self) -> usize {
        self.0.keys_len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    // TODO: trait
    pub fn insert(&mut self, key: &str, val: &str) {
        let key = HeaderName::from_str(key).unwrap();
        let val = HeaderValue::from_str(val).unwrap();
        self.0.insert(key, val);
    }
}
