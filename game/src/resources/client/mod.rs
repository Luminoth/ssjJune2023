#![cfg(feature = "client")]
#![allow(dead_code)]

pub mod auth;
pub mod splash;

use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Resource)]
pub struct User {
    user_id: String,
    display_name: String,
}

impl User {
    pub fn new(user_id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            display_name: display_name.into(),
        }
    }

    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }

    pub fn get_display_name(&self) -> &String {
        &self.display_name
    }
}
