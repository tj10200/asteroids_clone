use crate::shots::components::Weapon;
use crate::world::RigidBodyBehaviors;
use bevy::a11y::accesskit::Role::Meter;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::rapier::prelude::RigidBodyBuilder;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};

pub const METEOR_SPEED_RANGE: (f32, f32) = (-5.0, 5.0);
pub const METEOR_ROTATION_RANGE: (f32, f32) = (-3.0, 3.0);

pub const NUM_METEORS_TO_SPAWN_ON_DESTRUCTION: u32 = 3;

pub fn random_meteor_sprite_name(meteor_type: MeteorType) -> String {
    let mut rng = thread_rng();
    match meteor_type {
        MeteorType::Big => {
            let items = [("1", 4), ("2", 3), ("3", 3), ("4", 2)];
            let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();

            format!("meteorBrown_big{}.png", items[dist.sample(&mut rng)].0).to_string()
        }
        MeteorType::Med => {
            let items = [("1", 4), ("3", 3)];
            let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();

            format!("meteorBrown_med{}.png", items[dist.sample(&mut rng)].0).to_string()
        }
        MeteorType::Small => {
            let items = [("1", 5), ("2", 5)];
            let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();

            format!("meteorBrown_small{}.png", items[dist.sample(&mut rng)].0).to_string()
        }
        MeteorType::Tiny => {
            let items = [("1", 2), ("2", 1)];
            let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
            format!("meteorBrown_tiny{}.png", items[dist.sample(&mut rng)].0).to_string()
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    pub fn max_damage(&self) -> f32 {
        match self {
            MeteorType::Big => 200f32,
            MeteorType::Med => 100f32,
            MeteorType::Small => 50f32,
            MeteorType::Tiny => 25f32,
        }
    }

    pub fn next_size(&self) -> Self {
        match self {
            MeteorType::Big => MeteorType::Med,
            MeteorType::Med => MeteorType::Small,
            MeteorType::Small => MeteorType::Tiny,
            MeteorType::Tiny => MeteorType::Tiny,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Meteor {
    meteor_type: MeteorType,
    pub sprite_name: String,
    pub velocity: Velocity,
    pub density: f32,
    pub rotation: f32,
    pub frame_cols: usize,
    pub frame_rows: usize,
    pub start_frame: usize,
    pub damage: f32,
}

impl Default for Meteor {
    fn default() -> Self {
        Meteor::new(MeteorType::Big)
    }
}

impl Meteor {
    pub fn new(meteor_type: MeteorType) -> Meteor {
        let mut rng = thread_rng();
        let speed_x = rng.gen_range(METEOR_SPEED_RANGE.0..=METEOR_SPEED_RANGE.1);
        let speed_y = rng.gen_range(METEOR_SPEED_RANGE.0..=METEOR_SPEED_RANGE.1);
        let rotation = rng.gen_range(METEOR_ROTATION_RANGE.0..=METEOR_ROTATION_RANGE.1);
        Meteor {
            meteor_type,
            sprite_name: random_meteor_sprite_name(meteor_type),
            velocity: Velocity::linear(Vec2::new(speed_x, speed_y)),
            density: MeteorType::density(meteor_type),
            rotation,
            frame_cols: 1,
            frame_rows: 1,
            start_frame: 0,
            damage: 0f32,
        }
    }
    pub fn damage(&mut self, weapon: &Weapon) {
        self.damage += weapon.damage
    }

    pub fn destroyed(&self) -> bool {
        self.damage >= self.meteor_type.max_damage()
    }

    pub fn spawn_next_size(&self) -> Vec<Meteor> {
        match self.meteor_type {
            MeteorType::Tiny => return vec![],
            _ => {}
        };

        let mut vec = Vec::new();
        for i in 0..=NUM_METEORS_TO_SPAWN_ON_DESTRUCTION {
            vec.push(Meteor::new(self.meteor_type.next_size()))
        }
        vec
    }
}
