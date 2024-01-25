use bevy::prelude::*;
use bevy_rapier2d::prelude::ColliderMassProperties;

#[derive(Component)]
pub struct PlayerShip {
    pub density: f32,
}
