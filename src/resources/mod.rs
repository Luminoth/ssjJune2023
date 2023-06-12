pub mod looking_for_work;
pub mod main_menu;
pub mod server;
pub mod splash;

use bevy::prelude::*;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::prelude::*;

// TODO: would SmallRng be better here? we don't need a secure rng
#[derive(Resource, Deref, DerefMut)]
pub struct Random(StdRng);

impl Default for Random {
    fn default() -> Self {
        Self(StdRng::from_entropy())
    }
}

impl Random {
    pub fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.0.gen_range(range)
    }
}
