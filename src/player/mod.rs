use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct PlayerShipPlugin;

pub const PLAYER_SHIP_DENSITY: f32 = 1.0;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship).add_systems(
            Update,
            (
                update_player_position,
                update_player_position_from_coordinates,
                handle_player_intersections_with_wall,
            ),
        );
    }
}
