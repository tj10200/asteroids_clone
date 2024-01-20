use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_meteors))
            .add_systems(Update, handle_meteor_intersections_with_wall);
    }
}
