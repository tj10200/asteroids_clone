use bevy::prelude::*;

pub mod components;
mod systems;

use crate::game::states::SimulationState;
use crate::states::AppState;
use systems::*;

pub struct WeaponFirePlugin;

impl Plugin for WeaponFirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_fire_weapon, handle_shot_intersections_with_wall)
                .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
        )
        .add_systems(OnExit(AppState::Game), despawn_weapons);
    }
}
