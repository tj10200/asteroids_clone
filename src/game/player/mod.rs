use bevy::prelude::*;

pub mod components;
mod systems;

use crate::game::states::SimulationState;
use crate::states::AppState;
use components::PlayerLives;
use systems::*;

pub const PLAYER_SHIP: &str = "playerShip2_orange.png";
pub const PLAYER_LIVES: i8 = 3;

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerLives>()
            .add_systems(OnEnter(AppState::Game), spawn_ship)
            .add_systems(
                Update,
                (
                    update_player_position,
                    update_player_position_from_coordinates,
                    handle_player_intersections_with_wall,
                    handle_player_collision_with_meteor,
                    handle_player_collision_with_planet,
                    handle_player_respawn_on_death,
                    render_player_health,
                )
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
