use bevy::prelude::*;

// Taken from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct WorldCoordinates(pub Vec2);
