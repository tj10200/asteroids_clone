use crate::damage::{Damage, Damageable};
use crate::shots::components::Weapon;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use rand::distributions::{Uniform, WeightedIndex};
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};

use super::*;

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
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MeteorType {
    Big,
    Med,
    Small,
}

impl MeteorType {
    pub fn density(meteor_type: MeteorType) -> f32 {
        match meteor_type {
            MeteorType::Big => 100.,
            MeteorType::Med => 80.,
            MeteorType::Small => 50.,
        }
    }

    pub fn health(&self) -> f32 {
        match self {
            MeteorType::Big => 200f32,
            MeteorType::Med => 100f32,
            MeteorType::Small => 50f32,
        }
    }

    pub fn damage(&self) -> f32 {
        match self {
            MeteorType::Big => 2500f32,
            MeteorType::Med => 1500f32,
            MeteorType::Small => 500f32,
        }
    }

    pub fn next_size(&self) -> Self {
        let mut rng = thread_rng();

        match self {
            MeteorType::Big => {
                let size_distributions = [(MeteorType::Med, 2), (MeteorType::Small, 1)];
                let weighted_index =
                    WeightedIndex::new(size_distributions.iter().map(|item| item.1)).unwrap();

                size_distributions[weighted_index.sample(&mut rng)].0
            }
            MeteorType::Med => MeteorType::Small,
            MeteorType::Small => MeteorType::Small,
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
    pub health: f32,
    damage: f32,
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
        let health = meteor_type.health();
        let damage = meteor_type.damage();
        Meteor {
            meteor_type,
            sprite_name: random_meteor_sprite_name(meteor_type),
            velocity: Velocity::linear(Vec2::new(speed_x, speed_y)),
            density: MeteorType::density(meteor_type),
            rotation,
            frame_cols: 1,
            frame_rows: 1,
            start_frame: 0,
            health,
            damage,
        }
    }

    pub fn spawn_next_size(&self) -> Vec<Meteor> {
        match self.meteor_type {
            MeteorType::Small => return vec![],
            _ => {}
        };

        let mut vec = Vec::new();
        let mut rng = thread_rng();
        let range = Uniform::from(0.01f32..1f32);
        for _ in 0..=NUM_METEORS_TO_SPAWN_ON_DESTRUCTION {
            let chance = range.sample(&mut rng);
            if chance >= CHANCE_TO_SPAWN_METEOR_ON_DESTRUCTION {
                vec.push(Meteor::new(self.meteor_type.next_size()))
            }
        }
        vec
    }
}

impl Damageable for Meteor {
    fn damage(&mut self, entity: &impl Damage) {
        self.health -= entity.hit_points();
    }

    fn health(&self) -> f32 {
        self.health
    }
}

impl Damage for Meteor {
    fn hit_points(&self) -> f32 {
        self.damage
    }
}
