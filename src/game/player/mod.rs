use bevy::prelude::*;

pub mod components;
mod systems;

use crate::game::states::SimulationState;
use crate::states::AppState;
use systems::*;

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_ship)
            .add_systems(
                Update,
                (
                    update_player_position,
                    update_player_position_from_coordinates,
                    handle_player_intersections_with_wall,
                    handle_player_collision_with_meteor,
                    handle_player_collision_with_planet,
                )
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
