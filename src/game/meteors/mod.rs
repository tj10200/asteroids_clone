use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use crate::game::states::SimulationState;
use crate::states::AppState;

use crate::game::systems::resume_simulation;
use resources::MeteorSpawnTimer;
use systems::*;

pub const NUMBER_OF_METEORS: u32 = 3;
pub const METEORS_SCALE: f32 = 0.3;
pub const METEOR_SPEED_RANGE: (f32, f32) = (-5.0, 5.0);
pub const METEOR_ROTATION_RANGE: (f32, f32) = (-3.0, 3.0);

pub const NUM_METEORS_TO_SPAWN_ON_DESTRUCTION: u32 = 3;
pub const METEOR_SPAWN_TIME: f32 = 8.0;
pub const CHANCE_TO_SPAWN_METEOR_ON_DESTRUCTION: f32 = 0.3;

// allows meteors to spawn in the first 5% of the screen or the last 5% of the screen
pub const METEOR_SPAWN_RANGE_REL_TO_WINDOW: (f32, f32) = (0.05, 0.95);

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MeteorSystemSet {
    Movement,
    Confinement,
}

pub struct MeteorPlugin;

impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MeteorSpawnTimer>()
            .configure_sets(
                Update,
                MeteorSystemSet::Movement.before(MeteorSystemSet::Confinement),
            )
            .add_systems(OnEnter(AppState::Game), spawn_meteors)
            .add_systems(
                Update,
                (
                    handle_weapon_collision,
                    constrain_meteor_velocity,
                    tick_meteor_spawn_timer,
                    spawn_meteors_over_time,
                    render_meteor_health,
                )
                    .in_set(MeteorSystemSet::Movement)
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            )
            .add_systems(
                Update,
                (
                    handle_meteor_intersections_with_wall,
                    constrain_meteor_velocity,
                )
                    .in_set(MeteorSystemSet::Confinement)
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            )
            .add_systems(OnExit(AppState::Game), despawn_meteor);
    }
}
