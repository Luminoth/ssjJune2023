use bevy::prelude::*;

use crate::resources::Random;

pub struct Cooldown {
    timer: Timer,
    duration: bevy::utils::Duration,
}

impl Cooldown {
    pub fn new(duration: bevy::utils::Duration) -> Self {
        Self {
            timer: Timer::default(),
            duration,
        }
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }

    pub fn start(&mut self) {
        self.timer.set_duration(self.duration);
        self.timer.reset();
    }

    pub fn tick(&mut self, delta: bevy::utils::Duration) {
        self.timer.tick(delta);
    }
}

pub struct Throttle {
    timer: Timer,
    attempts: u32,
}

impl Default for Throttle {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
            attempts: 0,
        }
    }
}

impl Throttle {
    pub fn finished(&self) -> bool {
        self.timer.finished()
    }

    pub fn reset(&mut self) {
        self.attempts = 0;
    }

    pub fn start(&mut self, random: &mut Random) {
        // exponential backoff with "full" jitter - base 250ms, max of 10 seconds
        let millis = random.random_range(0..(250 * 2_u64.pow(self.attempts)).min(10000));

        info!("throttling for {}ms ...", millis);

        self.timer
            .set_duration(bevy::utils::Duration::from_millis(millis));
        self.timer.reset();

        self.attempts += 1;
    }

    pub fn tick(&mut self, delta: bevy::utils::Duration) {
        self.timer.tick(delta);
    }
}
