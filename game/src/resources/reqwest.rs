use bevy::prelude::*;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct ReqwestClient(pub reqwest::Client);

impl Default for ReqwestClient {
    fn default() -> Self {
        Self(reqwest::Client::new())
    }
}
