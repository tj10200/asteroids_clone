use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::time::Duration;

pub const DEFAULT_WEAPON_FIRE_DELAY: Duration = Duration::from_millis(150);

#[derive(Component)]
pub struct Weapon {
    pub damage: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct WeaponFireTimer {
    pub timer: Stopwatch,
    pub fire_delay: Duration,
}

impl Default for WeaponFireTimer {
    fn default() -> Self {
        let mut timer = WeaponFireTimer {
            timer: Stopwatch::new(),
            fire_delay: DEFAULT_WEAPON_FIRE_DELAY,
        };
        timer.timer.tick(timer.fire_delay);
        timer
    }
}
