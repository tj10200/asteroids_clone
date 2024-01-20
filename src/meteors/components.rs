use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};

pub const METEOR_SPEED_RANGE: (f32, f32) = (-35.0, 35.0);
pub const METEOR_ROTATION_RANGE: (f32, f32) = (-3.0, 3.0);

pub fn random_meteor_sprite_name(meteor_type: MeteorType) -> String {
    let mut rng = thread_rng();
    match meteor_type {
        MeteorType::Big => {
            let items = [("1", 4), ("2", 3), ("3", 3), ("4", 2)];
            let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();

            format!("meteorBrown_big{}.png", items[dist.sample(&mut rng)].1).to_string()
        }
        MeteorType::Med => {
            let items = [("1", 4), ("3", 3)];
            let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();

            format!("meteorBrown_med{}.png", items[dist.sample(&mut rng)].1).to_string()
        }
        MeteorType::Small => {
            let items = [("1", 5), ("2", 5)];
            let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();

            format!("meteorBrown_small{}.png", items[dist.sample(&mut rng)].1).to_string()
        }
        MeteorType::Tiny => {
            let items = [("1", 2), ("2", 1)];
            let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
            format!("meteorBrown_tiny{}.png", items[dist.sample(&mut rng)].1).to_string()
        }
    }
}

pub enum MeteorType {
    Big,
    Med,
    Small,
    Tiny,
}

impl MeteorType {
    pub fn density(meteor_type: MeteorType) -> f32 {
        match meteor_type {
            MeteorType::Big => 1.5,
            MeteorType::Med => 0.1,
            MeteorType::Small => 0.1,
            MeteorType::Tiny => 0.01,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Meteor {
    pub sprite_name: String,
    pub velocity: Velocity,
    pub density: f32,
    pub rotation: f32,
    pub frame_cols: usize,
    pub frame_rows: usize,
    pub start_frame: usize,
}

impl Default for Meteor {
    fn default() -> Self {
        let mut rng = thread_rng();
        let speed_x = rng.gen_range(METEOR_SPEED_RANGE.0..=METEOR_SPEED_RANGE.1);
        let speed_y = rng.gen_range(METEOR_SPEED_RANGE.0..=METEOR_SPEED_RANGE.1);
        let rotation = rng.gen_range(METEOR_ROTATION_RANGE.0..=METEOR_ROTATION_RANGE.1);
        Meteor {
            sprite_name: "meteorBrown_big4.png".to_string(), //random_meteor_sprite_name(MeteorType::Big),
            velocity: Velocity::linear(Vec2::new(speed_x, speed_y)),
            density: MeteorType::density(MeteorType::Big),
            rotation: rotation,
            frame_cols: 1,
            frame_rows: 1,
            start_frame: 0,
        }
    }
}
