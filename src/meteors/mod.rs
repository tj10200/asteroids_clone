use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use systems::*;

pub const NUMBER_OF_METEORS: u32 = 3;
pub const METEORS_SCALE: f32 = 0.3;
pub const METEOR_SPEED_RANGE: (f32, f32) = (-5.0, 5.0);
pub const METEOR_ROTATION_RANGE: (f32, f32) = (-3.0, 3.0);

pub const NUM_METEORS_TO_SPAWN_ON_DESTRUCTION: u32 = 3;
pub const METEOR_SPAWN_TIME: f32 = 5.0;

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_meteors)).add_systems(
            Update,
            (
                handle_meteor_intersections_with_wall,
                handle_weapon_collision,
                despawn_meteor,
                constrain_meteor_velocity,
            ),
        );
    }
}
