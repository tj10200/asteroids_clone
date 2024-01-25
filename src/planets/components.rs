use crate::planets::MAIN_PLANET_DENSITY;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const GRAVITATIONAL_CONSTANT: f32 = 0.2;
#[derive(Component)]
pub struct Planet {
    pub coordinates: Vec2,
    pub radius: f32,
    pub density: f32,
}

impl Planet {
    pub fn new(coordinates: Vec2, radius: f32, density: f32) -> Self {
        Planet {
            coordinates,
            radius,
            density,
        }
    }
    pub fn gravity(&self, other_density: f32) -> f32 {
        GRAVITATIONAL_CONSTANT * self.density * other_density
    }
}
