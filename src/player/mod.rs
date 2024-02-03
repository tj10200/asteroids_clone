use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship).add_systems(
            Update,
            (
                update_player_position,
                update_player_position_from_coordinates,
                handle_player_intersections_with_wall,
                handle_player_collision_with_meteor,
                handle_player_collision_with_planet,
            ),
        );
    }
}
