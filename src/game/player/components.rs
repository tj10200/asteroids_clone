use crate::damage::{Damage, Damageable};
use crate::game::meteors::METEOR_SPAWN_TIME;
use crate::game::player::PLAYER_LIVES;
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

#[derive(Resource)]
pub struct PlayerLives {
    pub lives: i8,
}

impl Default for PlayerLives {
    fn default() -> PlayerLives {
        PlayerLives {
            lives: PLAYER_LIVES,
        }
    }
}
