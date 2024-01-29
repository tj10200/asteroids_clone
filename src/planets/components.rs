use crate::damage::{Damage, Damageable};
use bevy::prelude::*;

pub const GRAVITATIONAL_CONSTANT: f32 = 0.2;
pub const PLANET_DAMAGE: f32 = 1e6f32;
pub const PLANET_HEALTH: f32 = 1e4;

#[derive(Component)]
pub struct Planet {
    pub coordinates: Vec2,
    pub radius: f32,
    pub density: f32,
    pub color: Color,
    health: f32,
}

impl Planet {
    pub fn new(coordinates: Vec2, radius: f32, density: f32, color: Color) -> Self {
        Planet {
            coordinates,
            radius,
            density,
            color,
            health: PLANET_HEALTH,
        }
    }
    pub fn gravity(&self, other_density: f32) -> f32 {
        GRAVITATIONAL_CONSTANT * self.density * other_density
    }
}

impl Damageable for Planet {
    fn damage(&mut self, entity: &impl Damage) {
        self.health -= entity.hit_points();
    }

    fn health(&self) -> f32 {
        self.health
    }
}

impl Damage for Planet {
    fn hit_points(&self) -> f32 {
        PLANET_DAMAGE
    }
}
