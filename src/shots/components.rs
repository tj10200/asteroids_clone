use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::time::Duration;

pub const DEFAULT_WEAPON_FIRE_DELAY: Duration = Duration::from_millis(150);
const DEFAULT_WEAPON_SPRITE_NAME: &str = "laserGreen02.png";
const DEFAULT_WEAPON_DAMAGE: f32 = 25.;
const DEFAULT_WEAPON_SPEED: f32 = 1500.;
const DEFAULT_WEAPON_SCALE: f32 = 0.5;

pub const FIRE_DISTANCE_FROM_PLAYER: f32 = 25.0;
const SHOT_DENSITY: f32 = 0.001;

#[derive(Component)]
pub struct Weapon {
    pub sprite_name: String,
    pub damage: f32,
    pub speed: f32,
    pub density: f32,
    pub scale: f32,
    pub frame_cols: usize,
    pub frame_rows: usize,
    pub start_frame: usize,
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon {
            sprite_name: DEFAULT_WEAPON_SPRITE_NAME.to_string(),
            damage: DEFAULT_WEAPON_DAMAGE,
            speed: DEFAULT_WEAPON_SPEED,
            density: SHOT_DENSITY,
            scale: DEFAULT_WEAPON_SCALE,
            frame_cols: 1,
            frame_rows: 1,
            start_frame: 0,
        }
    }
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
