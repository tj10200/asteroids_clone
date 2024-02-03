use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub const MAIN_PLANET_RADIUS: f32 = 100.;
pub const MAIN_PLANET_DENSITY: f32 = 50.;

pub struct PlanetsPlugin;

impl Plugin for PlanetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planets).add_systems(
            Update,
            (
                render_planets,
                simulate_meteor_gravity_toward_planets,
                simulate_player_gravity_toward_planets,
            ),
        );
    }
}
