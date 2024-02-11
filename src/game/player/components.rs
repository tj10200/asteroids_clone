use crate::damage::{Damage, Damageable};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerShip {
    pub density: f32,
    pub health: f32,
}

impl Damageable for PlayerShip {
    fn damage(&mut self, damage: &impl Damage) {
        self.health -= damage.hit_points();
    }

    fn health(&self) -> f32 {
        self.health
    }
}
